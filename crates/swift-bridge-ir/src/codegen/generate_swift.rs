use std::collections::HashMap;
use syn::Path;

use crate::bridged_type::{BridgeableType, BridgedType, TypePosition};
use crate::codegen::generate_swift::generate_function_swift_calls_rust::gen_func_swift_calls_rust;
use crate::codegen::generate_swift::opaque_copy_type::generate_opaque_copy_struct;
use crate::codegen::generate_swift::swift_class::generate_swift_class;
use crate::codegen::generate_swift::vec::generate_vectorizable_extension;
use crate::codegen::CodegenConfig;
use crate::parse::{
    HostLang, OpaqueForeignTypeDeclaration, SharedTypeDeclaration, TypeDeclaration,
    TypeDeclarations,
};
use crate::parsed_extern_fn::ParsedExternFn;
use crate::SwiftBridgeModule;

mod vec;

mod generate_function_swift_calls_rust;
mod opaque_copy_type;
mod shared_enum;
mod shared_struct;
mod swift_class;

impl SwiftBridgeModule {
    /// Generate the corresponding Swift code for the bridging module.
    pub(crate) fn generate_swift(&self, config: &CodegenConfig) -> String {
        let mut swift = "".to_string();

        if !self.module_will_be_compiled(config) {
            return swift;
        }

        let mut associated_funcs_and_methods: HashMap<String, Vec<&ParsedExternFn>> =
            HashMap::new();
        let mut class_protocols: HashMap<String, ClassProtocols> = HashMap::new();

        let mut has_encountered_at_least_one_sendable_swift_type = false;

        for function in &self.functions {
            if function.host_lang.is_rust() {
                if let Some(ty) = function.associated_type.as_ref() {
                    match ty {
                        TypeDeclaration::Shared(_) => {
                            //
                            todo!("Think about what to do here..")
                        }
                        TypeDeclaration::Opaque(opaque_ty) => {
                            associated_funcs_and_methods
                                .entry(opaque_ty.to_string())
                                .or_default()
                                .push(function);

                            if function.is_swift_identifiable {
                                let identifiable_protocol = IdentifiableProtocol {
                                    func_name: function.func.sig.ident.to_string(),
                                    return_ty: BridgedType::new_with_return_type(
                                        &function.func.sig.output,
                                        &self.types,
                                    )
                                    .unwrap()
                                    .to_swift_type(
                                        TypePosition::FnReturn(opaque_ty.host_lang),
                                        &self.types,
                                        &self.swift_bridge_path,
                                    ),
                                };
                                class_protocols
                                    .entry(opaque_ty.to_string())
                                    .or_default()
                                    .identifiable = Some(identifiable_protocol);
                            }
                        }
                    };
                    continue;
                }
            }
            let func_definition = match function.host_lang {
                HostLang::Rust => {
                    gen_func_swift_calls_rust(function, &self.types, &self.swift_bridge_path)
                }
                HostLang::Swift => gen_function_exposes_swift_to_rust(
                    function,
                    &self.types,
                    &self.swift_bridge_path,
                ),
            };
            swift += &func_definition;
            swift += "\n";
        }

        for ty in self.types.types() {
            match ty {
                TypeDeclaration::Shared(SharedTypeDeclaration::Struct(shared_struct)) => {
                    if let Some(swift_struct) = self.generate_shared_struct_string(shared_struct) {
                        swift += &swift_struct;
                        swift += "\n";
                    }
                }
                TypeDeclaration::Shared(SharedTypeDeclaration::Enum(shared_enum)) => {
                    if let Some(swift_enum) = self.generate_shared_enum_string(shared_enum) {
                        swift += &swift_enum;
                        swift += "\n";
                    }
                }
                TypeDeclaration::Opaque(ty) => {
                    match ty.host_lang {
                        HostLang::Rust => {
                            if let Some(_copy) = ty.attributes.copy {
                                swift += &generate_opaque_copy_struct(
                                    ty,
                                    &associated_funcs_and_methods,
                                    &self.types,
                                    &self.swift_bridge_path,
                                );
                            } else {
                                let class_protocols = class_protocols.get(&ty.ty.to_string());
                                let default_cp = ClassProtocols::default();
                                let class_protocols = class_protocols.unwrap_or(&default_cp);

                                swift += &generate_swift_class(
                                    ty,
                                    &associated_funcs_and_methods,
                                    class_protocols,
                                    &self.types,
                                    &self.swift_bridge_path,
                                );
                            }

                            swift += "\n";

                            if !ty.attributes.already_declared {
                                // TODO: Support Vec<OpaqueCopyType>. Add codegen tests and then
                                //  make them pass.
                                // TODO: Support Vec<GenericOpaqueRustType
                                if ty.attributes.copy.is_none() && ty.generics.len() == 0 {
                                    swift += &generate_vectorizable_extension(&ty);
                                    swift += "\n";
                                }
                            }

                            if ty.attributes.sendable {
                                let ty_name = ty.ty_name_string();
                                swift += &format!("extension {ty_name}: @unchecked Sendable {{}}")
                            }
                        }
                        HostLang::Swift => {
                            swift += &generate_drop_swift_instance_reference_count(ty);

                            if ty.attributes.sendable {
                                if !has_encountered_at_least_one_sendable_swift_type {
                                    swift += create_swift_sendable_protocol_check();
                                    swift += "\n";

                                    has_encountered_at_least_one_sendable_swift_type = true;
                                }

                                swift += &implement_swift_sendable_protocol(ty);
                            }

                            swift += "\n";
                        }
                    }
                }
            };
        }

        swift
    }
}

#[derive(Default)]
struct ClassProtocols {
    // The name of the function to use for the Identifiable protocol implementation.
    identifiable: Option<IdentifiableProtocol>,
}
struct IdentifiableProtocol {
    func_name: String,
    return_ty: String,
}

// Generate functions to drop the reference count on a Swift class instance.
//
// # Example
//
// ```
// @_cdecl("__swift_bridge__$Foo$_free")
// func __swift_bridge__Foo__free (ptr: UnsafeMutableRawPointer) {
//     let _ = Unmanaged<Foo>.fromOpaque(ptr).takeRetainedValue()
// }
// ```
fn generate_drop_swift_instance_reference_count(ty: &OpaqueForeignTypeDeclaration) -> String {
    let link_name = ty.free_swift_class_link_name();
    let fn_name = ty.free_swift_class_func_name();

    format!(
        r##"
@_cdecl("{link_name}")
func {fn_name} (ptr: UnsafeMutableRawPointer) {{
    let _ = Unmanaged<{ty_name}>.fromOpaque(ptr).takeRetainedValue()
}}
"##,
        link_name = link_name,
        fn_name = fn_name,
        ty_name = ty.ty_name_ident()
    )
}

fn gen_function_exposes_swift_to_rust(
    func: &ParsedExternFn,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let is_async = func.sig.asyncness.is_some();

    if is_async {
        gen_async_function_exposes_swift_to_rust(func, types, swift_bridge_path)
    } else {
        gen_sync_function_exposes_swift_to_rust(func, types, swift_bridge_path)
    }
}

/// Common metadata extracted from a ParsedExternFn for generating Swift wrappers.
struct SwiftFnMetadata {
    /// The link name used in @_cdecl (e.g., "__swift_bridge__$some_function")
    link_name: String,
    /// The prefixed function name (e.g., "__swift_bridge__some_function")
    prefixed_fn_name: String,
    /// The Swift function name to call (respects swift_name_override)
    fn_name: String,
}

impl SwiftFnMetadata {
    fn from_parsed_extern_fn(func: &ParsedExternFn) -> Self {
        let link_name = func.link_name();
        let prefixed_fn_name = func.prefixed_fn_name().to_string();
        let fn_name = if let Some(swift_name) = func.swift_name_override.as_ref() {
            swift_name.value()
        } else {
            func.sig.ident.to_string()
        };
        Self {
            link_name,
            prefixed_fn_name,
            fn_name,
        }
    }
}

/// Build the Swift call expression for calling a method or function.
///
/// For methods, this generates code like:
/// `Unmanaged<TypeName>.fromOpaque(this).takeUnretainedValue().fn_name(args)`
///
/// For static methods:
/// `TypeName::fn_name(args)`
///
/// For freestanding functions:
/// `fn_name(args)`
fn build_swift_call_expression(func: &ParsedExternFn, fn_name: &str, args: &str) -> String {
    if let Some(associated_type) = func.associated_type.as_ref() {
        let ty_name = match associated_type {
            TypeDeclaration::Shared(_) => todo!(),
            TypeDeclaration::Opaque(associated_type) => associated_type.to_string(),
        };

        if func.is_method() {
            format!("Unmanaged<{ty_name}>.fromOpaque(this).takeUnretainedValue().{fn_name}({args})")
        } else {
            format!("{ty_name}::{fn_name}({args})")
        }
    } else {
        format!("{fn_name}({args})")
    }
}

/// Generate Swift code that exposes an async Swift function to Rust.
///
/// For async functions, we generate a wrapper that:
/// 1. Takes callback wrapper and callback function pointer(s) as parameters
/// 2. Spawns a Task to call the async Swift function
/// 3. When the async function completes, calls the Rust callback with the result
fn gen_async_function_exposes_swift_to_rust(
    func: &ParsedExternFn,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let metadata = SwiftFnMetadata::from_parsed_extern_fn(func);
    let link_name = &metadata.link_name;
    let prefixed_fn_name = &metadata.prefixed_fn_name;
    let fn_name = &metadata.fn_name;

    // Get the original function arguments (excluding callback params which we add)
    let original_params = func.to_swift_param_names_and_types(true, types, swift_bridge_path);
    let args = func.to_swift_call_args(false, true, types, swift_bridge_path);

    // Check if this is a Result type
    let return_ty = BridgedType::new_with_return_type(&func.sig.output, types);
    let maybe_result = return_ty.as_ref().and_then(|ty| ty.as_result());

    // Build the async call expression
    let call_expression = build_swift_call_expression(func, fn_name, &args);

    // Build params_str and task_body based on whether this is a Result type or not
    let (params_str, task_body) = if let Some(result) = maybe_result {
        // Result type: generate two callbacks (on_success and on_error)

        // For the catch clause, we need the actual Swift wrapper type name (e.g., "ErrorType"),
        // not the FFI type ("UnsafeMutableRawPointer"). Using HostLang::Rust gives us the
        // Swift wrapper class name that conforms to Error.
        let err_swift_ty = result.err_ty.to_swift_type(
            TypePosition::FnReturn(HostLang::Rust),
            types,
            swift_bridge_path,
        );

        let ok_ffi_convert = result.ok_ty.convert_swift_expression_to_ffi_type(
            "result",
            types,
            TypePosition::FnReturn(func.host_lang),
        );
        let err_ffi_convert = result.err_ty.convert_swift_expression_to_ffi_type(
            "error",
            types,
            TypePosition::FnReturn(func.host_lang),
        );

        // Get FFI types for ok and error values
        let ok_ffi_ty = result.ok_ty.to_swift_type(
            TypePosition::FnReturn(func.host_lang),
            types,
            swift_bridge_path,
        );
        let err_ffi_ty = result.err_ty.to_swift_type(
            TypePosition::FnReturn(func.host_lang),
            types,
            swift_bridge_path,
        );

        // Build params: this (if method), callbackWrapper, onSuccess, onError, then original params
        let mut all_params = Vec::new();
        if func.is_method() {
            all_params.push("_ this: UnsafeMutableRawPointer".to_string());
        }
        all_params.push("_ callbackWrapper: UnsafeMutableRawPointer".to_string());
        all_params.push(format!(
            "_ onSuccess: @escaping @convention(c) (UnsafeMutableRawPointer, {ok_ffi_ty}) -> Void"
        ));
        all_params.push(format!(
            "_ onError: @escaping @convention(c) (UnsafeMutableRawPointer, {err_ffi_ty}) -> Void"
        ));
        if !original_params.is_empty() {
            all_params.push(original_params.clone());
        }

        // This `fatalError` can occur if the Swift function throws a type that cannot be cast to the
        // error type in the user's bridge module. See the "Functions" chapter in the internal book for
        // more documentation.
        let task_body = format!(
            r#"do {{
            let result = try await {call_expression}
            onSuccess(callbackWrapper, {ok_ffi_convert})
        }} catch let error as {err_swift_ty} {{
            onError(callbackWrapper, {err_ffi_convert})
        }} catch {{
            fatalError("Error could not be cast to {err_swift_ty}")
        }}"#
        );

        (all_params.join(", "), task_body)
    } else {
        // Non-Result type: single callback
        let return_ty_ref = return_ty.as_ref();
        let has_return_value = return_ty_ref
            .map(|ty| !ty.can_be_encoded_with_zero_bytes())
            .unwrap_or(false);

        let callback_signature = if has_return_value {
            let built_in = return_ty_ref.unwrap();
            let swift_return_ty = built_in.to_swift_type(
                TypePosition::FnReturn(func.host_lang),
                types,
                swift_bridge_path,
            );
            format!(
                "_ callback: @escaping @convention(c) (UnsafeMutableRawPointer, {}) -> Void",
                swift_return_ty
            )
        } else {
            "_ callback: @escaping @convention(c) (UnsafeMutableRawPointer) -> Void".to_string()
        };

        // Build params: this (if method), callbackWrapper, callback, then original params
        let mut all_params = Vec::new();
        if func.is_method() {
            all_params.push("_ this: UnsafeMutableRawPointer".to_string());
        }
        all_params.push("_ callbackWrapper: UnsafeMutableRawPointer".to_string());
        all_params.push(callback_signature);
        if !original_params.is_empty() {
            all_params.push(original_params.clone());
        }

        let callback_call = if has_return_value {
            let built_in = return_ty_ref.unwrap();
            let convert = built_in.convert_swift_expression_to_ffi_type(
                "result",
                types,
                TypePosition::FnReturn(func.host_lang),
            );
            format!("callback(callbackWrapper, {convert})")
        } else {
            "callback(callbackWrapper)".to_string()
        };

        let result_binding = if has_return_value {
            "let result = "
        } else {
            "let _ = "
        };

        let task_body = format!("{result_binding}await {call_expression}\n        {callback_call}");

        (all_params.join(", "), task_body)
    };

    format!(
        r#"@_cdecl("{link_name}")
func {prefixed_fn_name} ({params_str}) {{
    Task {{
        {task_body}
    }}
}}
"#
    )
}

/// Generate Swift code that exposes a synchronous Swift function to Rust.
fn gen_sync_function_exposes_swift_to_rust(
    func: &ParsedExternFn,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let metadata = SwiftFnMetadata::from_parsed_extern_fn(func);
    let link_name = &metadata.link_name;
    let prefixed_fn_name = &metadata.prefixed_fn_name;
    let fn_name = &metadata.fn_name;

    let params = func.to_swift_param_names_and_types(true, types, swift_bridge_path);
    let ret = func.to_swift_return_type(types, swift_bridge_path);

    let args = func.to_swift_call_args(false, true, types, swift_bridge_path);
    let mut call_fn = format!("{}({})", fn_name, args);
    if let Some(built_in) = BridgedType::new_with_return_type(&func.sig.output, types) {
        if let Some(associated_type) = func.associated_type.as_ref() {
            let ty_name = match associated_type {
                TypeDeclaration::Shared(_) => {
                    //
                    todo!()
                }
                TypeDeclaration::Opaque(associated_type) => associated_type.to_string(),
            };

            if func.is_method() {
                call_fn = format!(
                    "Unmanaged<{ty_name}>.fromOpaque(this).takeUnretainedValue().{call_fn}",
                    ty_name = ty_name,
                    call_fn = call_fn
                );
                call_fn = built_in.convert_swift_expression_to_ffi_type(
                    &call_fn,
                    types,
                    TypePosition::FnReturn(func.host_lang),
                );
            } else if func.is_swift_initializer {
                call_fn = format!("Unmanaged.passRetained({}({})).toOpaque()", ty_name, args);
            } else {
                call_fn = format!("{}::{}", ty_name, call_fn);
            }
        } else {
            call_fn = built_in.convert_swift_expression_to_ffi_type(
                &call_fn,
                types,
                TypePosition::FnReturn(func.host_lang),
            );
        }
    } else {
        todo!("Push to ParsedErrors")
    };

    let mut rust_fn_once_callback_classes = "".to_string();

    let maybe_associated_ty = if let Some(ty) = func.associated_type.as_ref() {
        format!("${}", ty.as_opaque().unwrap().ty.to_string())
    } else {
        "".to_string()
    };

    for (idx, boxed_fn) in func.args_filtered_to_boxed_fns(types) {
        if boxed_fn.does_not_have_params_or_return() {
            continue;
        }

        let params_as_swift = boxed_fn.params_to_swift_types(types, swift_bridge_path);
        let swift_ffi_call_args = boxed_fn.to_from_swift_to_rust_ffi_call_args(types);

        let maybe_ret = if boxed_fn.ret.is_null() {
            "".to_string()
        } else {
            let ret = boxed_fn.ret.to_swift_type(
                TypePosition::FnArg(HostLang::Rust, idx),
                types,
                swift_bridge_path,
            );
            format!(" -> {}", ret)
        };

        let ret_value = format!(
            "__swift_bridge__{maybe_associated_ty}${fn_name}$param{idx}(ptr{swift_ffi_call_args})"
        );
        let ret_value = boxed_fn.ret.convert_ffi_expression_to_swift_type(
            &ret_value,
            TypePosition::FnReturn(HostLang::Rust),
            types,
            swift_bridge_path,
        );

        let maybe_generics = boxed_fn.maybe_swift_generics(types);

        rust_fn_once_callback_classes += &format!(
            r#"
class __private__RustFnOnceCallback{maybe_associated_ty}${fn_name}$param{idx} {{
    var ptr: UnsafeMutableRawPointer
    var called = false

    init(ptr: UnsafeMutableRawPointer) {{
        self.ptr = ptr
    }}

    deinit {{
        if !called {{
            __swift_bridge__{maybe_associated_ty}${fn_name}$_free$param{idx}(ptr)
        }}
    }}

    func call{maybe_generics}({params_as_swift}){maybe_ret} {{
        if called {{
            fatalError("Cannot call a Rust FnOnce function twice")
        }}
        called = true
        return {ret_value}
    }}
}}"#
        );
    }

    let callback_initializers =
        func.fnonce_callback_initializers(&fn_name, &maybe_associated_ty, types);
    if !callback_initializers.is_empty() {
        let maybe_ret = if ret.is_empty() {
            "let _ = "
        } else {
            "return "
        };

        call_fn = format!("{{ {callback_initializers} {maybe_ret}{call_fn} }}()")
    }

    let generated_func = format!(
        r#"@_cdecl("{link_name}")
func {prefixed_fn_name} ({params}){ret} {{
    {call_fn}
}}{rust_fn_once_callback_classes}
"#,
        link_name = link_name,
        prefixed_fn_name = prefixed_fn_name,
        params = params,
        ret = ret,
        call_fn = call_fn
    );

    generated_func
}

struct ClassMethods {
    initializers: Vec<String>,
    owned_self_methods: Vec<String>,
    ref_self_methods: Vec<String>,
    ref_mut_self_methods: Vec<String>,
}

fn generate_swift_class_methods(
    type_name: &str,
    associated_funcs_and_methods: &HashMap<String, Vec<&ParsedExternFn>>,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> ClassMethods {
    let mut initializers = vec![];
    let mut owned_self_methods = vec![];
    let mut ref_self_methods = vec![];
    let mut ref_mut_self_methods = vec![];

    if let Some(methods) = associated_funcs_and_methods.get(type_name) {
        for type_method in methods {
            let func_definition = gen_func_swift_calls_rust(type_method, types, swift_bridge_path);

            let is_class_func = type_method.func.sig.inputs.is_empty();

            if type_method.is_swift_initializer {
                initializers.push(func_definition);
            } else if is_class_func {
                ref_self_methods.push(func_definition);
            } else {
                if type_method.self_reference().is_some() {
                    if type_method.self_mutability().is_some() {
                        ref_mut_self_methods.push(func_definition);
                    } else {
                        ref_self_methods.push(func_definition);
                    }
                } else {
                    owned_self_methods.push(func_definition);
                }
            }
        }
    }

    ClassMethods {
        initializers,
        owned_self_methods,
        ref_self_methods,
        ref_mut_self_methods,
    }
}

/// A Swift protocol that inherits from Swift's `Sendable` protocol.
/// We use this to validate at compile time that a Swift type is `Sendable`.
fn create_swift_sendable_protocol_check() -> &'static str {
    "protocol __swift_bridge__IsSendable: Sendable {}"
}

/// A Swift protocol that inherits from Swift's `Sendable` protocol.
/// We use this to validate at compile time that a Swift type is `Sendable`.
fn implement_swift_sendable_protocol(ty: &OpaqueForeignTypeDeclaration) -> String {
    let ty_name = ty.ty_name_string();
    format!("extension {ty_name}: __swift_bridge__IsSendable {{}}")
}

#[cfg(test)]
mod tests {
    //! More tests can be found in src/codegen/codegen_tests.rs and its submodules.
    //! TODO: Gradually delete these tests and replace them with tests in the existing
    //!  `mod codegen_tests`.
    //!  This way we have one place to analyze the related Rust+Swift+C generated code
    //!  vs. currently needing to look at `generate_swift.rs` `generate_c.rs` and `generate_rust.rs`
    //!  to get a full picture of the codegen.

    use crate::codegen::generate_swift::CodegenConfig;
    use quote::quote;
    use syn::parse_quote;

    use crate::test_utils::assert_trimmed_generated_contains_trimmed_expected;
    use crate::SwiftBridgeModule;

    /// Verify that we generated a Swift function to call our freestanding function.
    #[test]
    fn freestanding_rust_function_no_args() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo ();
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public func foo() {
    __swift_bridge__$foo()
} 
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we generate code to expose a freestanding Swift function.
    #[test]
    fn freestanding_swift_function_no_args() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    fn foo ();
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$foo")
func __swift_bridge__foo () {
    foo()
} 
"#;

        assert_trimmed_generated_contains_trimmed_expected(generated.trim(), expected.trim());
    }

    /// Verify that we convert slices.
    #[test]
    fn freestanding_swift_function_return_slice() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    fn foo () -> &'static [u8];
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$foo")
func __swift_bridge__foo () -> __private__FfiSlice {
    foo().toFfiSlice()
} 
"#;

        assert_trimmed_generated_contains_trimmed_expected(generated.trim(), expected.trim());
    }

    /// Verify that we convert a Swift method's returned slice.
    #[test]
    fn swift_function_return_slice() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    type MyType;
                    fn foo (&self) -> &'static [u8];
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$MyType$foo")
func __swift_bridge__MyType_foo (_ this: UnsafeMutableRawPointer) -> __private__FfiSlice {
    Unmanaged<MyType>.fromOpaque(this).takeUnretainedValue().foo().toFfiSlice()
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(generated.trim(), expected.trim());
    }

    /// Verify that we generated a Swift function to call a freestanding function with one arg.
    #[test]
    fn freestanding_rust_function_one_arg() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo (bar: u8);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public func foo(_ bar: UInt8) {
    __swift_bridge__$foo(bar)
} 
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we generated a Swift function to call a freestanding function with a return
    /// type.
    #[test]
    fn freestanding_function_with_return() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo () -> u32;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public func foo() -> UInt32 {
    __swift_bridge__$foo()
} 
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we can convert a slice reference into an UnsafeBufferPointer
    #[test]
    fn freestanding_func_return_ref_byte_slice() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo () -> &[u8];
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
func foo() -> UnsafeBufferPointer<UInt8> {
    let slice = __swift_bridge__$foo(); return UnsafeBufferPointer(start: slice.start.assumingMemoryBound(to: UInt8.self), count: Int(slice.len));
} 
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we generated a function that Rust can use to reduce a Swift class instance's
    /// reference count.
    #[test]
    fn free_class_memory() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    type Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$Foo$_free")
func __swift_bridge__Foo__free (ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<Foo>.fromOpaque(ptr).takeRetainedValue()
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we generated a function that Rust can use to reduce a Swift class instance's
    /// reference count.
    #[test]
    fn extern_swift_class_init() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn new (a: u8) -> Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$Foo$new")
func __swift_bridge__Foo_new (_ a: UInt8) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(Foo(a: a)).toOpaque()
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we generated a Swift class with an init method.
    #[test]
    fn class_with_init() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn new() -> Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public class Foo: FooRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }
}
extension Foo {
    public convenience init() {
        self.init(ptr: __swift_bridge__$Foo$new())
    }
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, expected);
    }

    /// Verify that we generated a Swift class with an init method with params.
    #[test]
    fn class_with_init_and_param() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn new(val: u8) -> Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public class Foo: FooRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }
}
extension Foo {
    public convenience init(_ val: UInt8) {
        self.init(ptr: __swift_bridge__$Foo$new(val))
    }
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, expected);
    }

    /// Verify that we generate a Swift function that allows us to access a class instance method
    /// from Rust using a pointer.
    #[test]
    fn extern_swift_class_instance_method() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    type Foo;

                    fn push(&self, arg: u8);
                    fn pop(self: &mut Foo);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$Foo$push")
func __swift_bridge__Foo_push (_ this: UnsafeMutableRawPointer, _ arg: UInt8) {
    Unmanaged<Foo>.fromOpaque(this).takeUnretainedValue().push(arg: arg)
}

@_cdecl("__swift_bridge__$Foo$pop")
func __swift_bridge__Foo_pop (_ this: UnsafeMutableRawPointer) {
    Unmanaged<Foo>.fromOpaque(this).takeUnretainedValue().pop()
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we can generate an instance method that has a return value.
    #[test]
    fn instance_method_with_return() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    fn bar(&self) -> u8;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public class FooRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FooRef {
    public func bar() -> UInt8 {
        __swift_bridge__$Foo$bar(ptr)
    }
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, expected);
    }

    /// Verify that we can generate a Swift instance method with an argument to a declared type.
    #[test]
    fn instance_method_with_declared_type_arg() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    fn bar(self: &Foo, other: &Foo);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public class FooRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FooRef {
    public func bar(_ other: FooRef) {
        __swift_bridge__$Foo$bar(ptr, other.ptr)
    }
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, expected);
    }

    /// Verify that we can generate a static class method.
    #[test]
    fn static_class_method() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn bar();
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public class FooRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension FooRef {
    class public func bar() {
        __swift_bridge__$Foo$bar()
    }
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, expected);
    }

    /// Verify that we generate a Swift function that allows us to access a static class method
    /// from Rust using a pointer.
    #[test]
    fn extern_swift_static_class_method() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn bar(arg: u8);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$Foo$bar")
func __swift_bridge__Foo_bar (_ arg: UInt8) {
    Foo::bar(arg: arg)
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, expected);
    }

    /// Verify that we properly generate a Swift function that returns a String.
    #[test]
    fn return_string() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo () -> String;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
public func foo() -> RustString {
    RustString(ptr: __swift_bridge__$foo())
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we generate the corresponding Swift for extern "Rust" functions that accept
    /// a *const void pointer.
    #[test]
    fn extern_rust_const_void_pointer_argument() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn void_pointer (arg1: *const c_void);
                }
            }
        };
        let module: SwiftBridgeModule = syn::parse2(start).unwrap();
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
func void_pointer(_ arg1: UnsafeRawPointer) {
    __swift_bridge__$void_pointer(UnsafeMutableRawPointer(mutating: arg1))
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we generate the corresponding Swift for extern "Rust" functions that returns
    /// a *const void pointer.
    #[test]
    fn extern_rust_return_const_void_pointer() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn void_pointer () -> *const c_void;
                }
            }
        };
        let module: SwiftBridgeModule = syn::parse2(start).unwrap();
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
func void_pointer() -> UnsafeRawPointer {
    UnsafeRawPointer(__swift_bridge__$void_pointer()!)
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we generate the corresponding Swift for extern "Rust" functions that accept
    /// a *const void pointer.
    #[test]
    fn extern_swift_const_void_pointer_argument() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    fn void_pointer (arg: *const c_void);
                }
            }
        };
        let module: SwiftBridgeModule = syn::parse2(start).unwrap();
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$void_pointer")
func __swift_bridge__void_pointer (_ arg: UnsafeRawPointer) {
    void_pointer(arg: arg)
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we generate the correct function for an extern "Rust" fn that takes an owned
    /// opaque Swift type.
    #[test]
    fn extern_rust_fn_with_extern_swift_owned_opaque_arg() {
        let tokens = quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function (arg: Foo);
                }

                extern "Swift" {
                    type Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
func some_function(_ arg: Foo) {
    __swift_bridge__$some_function(Unmanaged.passRetained(arg).toOpaque())
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we generate the correct function for an extern "Rust" fn that returns an owned
    /// opaque Swift type.
    #[test]
    fn extern_rust_fn_returns_extern_swift_owned_opaque_type() {
        let tokens = quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function () -> Foo;
                }

                extern "Swift" {
                    type Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
func some_function() -> Foo {
    Unmanaged<Foo>.fromOpaque(__swift_bridge__$some_function()).takeRetainedValue()
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }

    /// Verify that we use a function's `swift_name = "..."` attribute during Swift codegen for
    /// extern Swift functions.
    #[test]
    fn extern_swift_uses_swift_name_function_attribute() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    #[swift_bridge(swift_name = "someFunctionSwiftName")]
                    fn some_function ();
                }
            }
        };
        let module: SwiftBridgeModule = syn::parse2(start).unwrap();
        let generated = module.generate_swift(&CodegenConfig::no_features_enabled());

        let expected = r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function () {
    someFunctionSwiftName()
}
"#;

        assert_trimmed_generated_contains_trimmed_expected(&generated, &expected);
    }
}
