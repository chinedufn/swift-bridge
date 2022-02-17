use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use quote::ToTokens;
use syn::{Path, ReturnType, Type};

use crate::bridged_type::{fn_arg_name, BridgedType, StdLibType, TypePosition};
use crate::codegen::generate_swift::vec::generate_vectorizable_extension;
use crate::codegen::CodegenConfig;
use crate::parse::{
    HostLang, OpaqueForeignTypeDeclaration, SharedTypeDeclaration, TypeDeclaration,
    TypeDeclarations,
};
use crate::parsed_extern_fn::ParsedExternFn;
use crate::{SwiftBridgeModule, SWIFT_BRIDGE_PREFIX};

mod vec;

mod shared_enum;
mod shared_struct;

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
                                .entry(opaque_ty.ident.to_string())
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
                                    .to_swift_type(TypePosition::FnReturn(opaque_ty.host_lang)),
                                };
                                class_protocols
                                    .entry(opaque_ty.ident.to_string())
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
                TypeDeclaration::Opaque(ty) => match ty.host_lang {
                    HostLang::Rust => {
                        let class_protocols = class_protocols.get(&ty.ty.ident.to_string());
                        let default_cp = ClassProtocols::default();
                        let class_protocols = class_protocols.unwrap_or(&default_cp);

                        swift += &generate_swift_class(
                            ty,
                            &associated_funcs_and_methods,
                            class_protocols,
                            &self.types,
                            &self.swift_bridge_path,
                        );
                        swift += "\n";

                        if !ty.already_declared {
                            swift += &generate_vectorizable_extension(&ty.ident);
                            swift += "\n";
                        }
                    }
                    HostLang::Swift => {
                        swift += &generate_drop_swift_instance_reference_count(ty);
                        swift += "\n";
                    }
                },
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

fn generate_swift_class(
    ty: &OpaqueForeignTypeDeclaration,
    associated_funcs_and_methods: &HashMap<String, Vec<&ParsedExternFn>>,
    class_protocols: &ClassProtocols,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let type_name = ty.ident.to_string();

    let mut initializers = vec![];

    let mut owned_self_methods = vec![];
    let mut ref_self_methods = vec![];
    let mut ref_mut_self_methods = vec![];

    if let Some(methods) = associated_funcs_and_methods.get(&type_name) {
        for type_method in methods {
            // TODO: Normalize with freestanding func codegen above

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

    let class_decl = if ty.already_declared {
        "".to_string()
    } else {
        let free_func_call = format!("{}${}$_free(ptr)", SWIFT_BRIDGE_PREFIX, type_name);

        format!(
            r#"public class {type_name}: {type_name}RefMut {{
    var isOwned: Bool = true

    override init(ptr: UnsafeMutableRawPointer) {{
        super.init(ptr: ptr)
    }}

    deinit {{
        if isOwned {{
            {free_func_call}
        }}
    }}
}}"#,
            type_name = type_name,
            free_func_call = free_func_call
        )
    };
    let class_ref_mut_decl = if ty.already_declared {
        "".to_string()
    } else {
        format!(
            r#"
public class {type_name}RefMut: {type_name}Ref {{
    override init(ptr: UnsafeMutableRawPointer) {{
        super.init(ptr: ptr)
    }}
}}"#,
            type_name = type_name
        )
    };
    let mut class_ref_decl = if ty.already_declared {
        "".to_string()
    } else {
        format!(
            r#"
public class {type_name}Ref {{
    var ptr: UnsafeMutableRawPointer

    init(ptr: UnsafeMutableRawPointer) {{
        self.ptr = ptr
    }}
}}"#,
            type_name = type_name,
        )
    };
    if let Some(identifiable) = class_protocols.identifiable.as_ref() {
        let identifiable_var = if identifiable.func_name == "id" {
            "".to_string()
        } else {
            format!(
                r#"
    public var id: {identifiable_return_ty} {{
        return self.{identifiable_func}()
    }}
"#,
                identifiable_func = identifiable.func_name,
                identifiable_return_ty = identifiable.return_ty
            )
        };

        class_ref_decl += &format!(
            r#"
extension {type_name}Ref: Identifiable {{{identifiable_var}}}"#,
            type_name = type_name,
            identifiable_var = identifiable_var,
        );
    }

    let initializers = if initializers.len() == 0 {
        "".to_string()
    } else {
        let initializers: String = initializers.join("\n\n");
        format!(
            r#"
extension {type_name} {{
{initializers}
}}"#,
            type_name = type_name,
            initializers = initializers
        )
    };

    let owned_instance_methods = if owned_self_methods.len() == 0 {
        "".to_string()
    } else {
        let owned_instance_methods: String = owned_self_methods.join("\n\n");
        format!(
            r#"
extension {type_name} {{
{owned_instance_methods}
}}"#,
            type_name = type_name,
            owned_instance_methods = owned_instance_methods
        )
    };

    let ref_instance_methods = if ref_self_methods.len() == 0 {
        "".to_string()
    } else {
        let ref_instance_methods: String = ref_self_methods.join("\n\n");
        format!(
            r#"
extension {type_name}Ref {{
{ref_instance_methods}
}}"#,
            type_name = type_name,
            ref_instance_methods = ref_instance_methods
        )
    };

    let ref_mut_instance_methods = if ref_mut_self_methods.len() == 0 {
        "".to_string()
    } else {
        let ref_mut_instance_methods: String = ref_mut_self_methods.join("\n\n");
        format!(
            r#"
extension {type_name}RefMut {{
{ref_mut_instance_methods}
}}"#,
            type_name = type_name,
            ref_mut_instance_methods = ref_mut_instance_methods
        )
    };

    let class = format!(
        r#"
{class_decl}{initializers}{owned_instance_methods}{class_ref_decl}{ref_mut_instance_methods}{class_ref_mut_decl}{ref_instance_methods}"#,
        class_decl = class_decl,
        class_ref_decl = class_ref_mut_decl,
        class_ref_mut_decl = class_ref_decl,
        initializers = initializers,
        owned_instance_methods = owned_instance_methods,
        ref_mut_instance_methods = ref_mut_instance_methods,
        ref_instance_methods = ref_instance_methods,
    );

    return class;
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
    let link_name = ty.free_link_name();
    let fn_name = ty.free_func_name();

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

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
enum SwiftFuncGenerics {
    String,
    Str,
}

impl SwiftFuncGenerics {
    fn as_bound(&self) -> &'static str {
        match self {
            SwiftFuncGenerics::String => "GenericIntoRustString: IntoRustString",
            SwiftFuncGenerics::Str => "GenericToRustStr: ToRustStr",
        }
    }
}

fn gen_func_swift_calls_rust(
    function: &ParsedExternFn,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let fn_name = function.sig.ident.to_string();
    let params = function.to_swift_param_names_and_types(false, types);
    let call_args = function.to_swift_call_args(true, false, types, swift_bridge_path);

    let call_fn = if function.sig.asyncness.is_some() {
        format!("{}(wrapperPtr, onComplete)", fn_name)
    } else {
        format!("{}({})", fn_name, call_args)
    };

    let type_name_segment = if let Some(ty) = function.associated_type.as_ref() {
        match ty {
            TypeDeclaration::Shared(_) => {
                //
                todo!()
            }
            TypeDeclaration::Opaque(ty) => {
                format!("${}", ty.ident.to_string())
            }
        }
    } else {
        "".to_string()
    };

    let maybe_static_class_func = if function.associated_type.is_some()
        && (!function.is_method() && !function.is_swift_initializer)
    {
        "class "
    } else {
        ""
    };

    let swift_class_func_name = if function.is_swift_initializer {
        "convenience init".to_string()
    } else {
        format!("public func {}", fn_name.as_str())
    };

    let indentation = if function.associated_type.is_some() {
        "    "
    } else {
        ""
    };

    let call_rust = format!(
        "{prefix}{type_name_segment}${call_fn}",
        prefix = SWIFT_BRIDGE_PREFIX,
        type_name_segment = type_name_segment,
        call_fn = call_fn
    );
    let mut call_rust = if function.sig.asyncness.is_some() {
        call_rust
    } else if function.is_swift_initializer {
        call_rust
    } else if let Some(built_in) = function.return_ty_built_in(types) {
        built_in.convert_ffi_value_to_swift_value(
            &call_rust,
            TypePosition::FnReturn(function.host_lang),
        )
    } else {
        if function.host_lang.is_swift() {
            call_rust
        } else {
            match &function.sig.output {
                ReturnType::Default => {
                    // () is a built in type so this would have been handled in the previous block.
                    unreachable!()
                }
                ReturnType::Type(_, ty) => {
                    let ty_name = match ty.deref() {
                        Type::Reference(reference) => reference.elem.to_token_stream().to_string(),
                        Type::Path(path) => path.path.segments.to_token_stream().to_string(),
                        _ => todo!(),
                    };

                    match types.get(&ty_name).unwrap() {
                        TypeDeclaration::Shared(_) => call_rust,
                        TypeDeclaration::Opaque(opaque) => {
                            if opaque.host_lang.is_rust() {
                                let (is_owned, ty) = match ty.deref() {
                                    Type::Reference(reference) => ("false", &reference.elem),
                                    _ => ("true", ty),
                                };

                                let ty = ty.to_token_stream().to_string();
                                format!("{}(ptr: {}, isOwned: {})", ty, call_rust, is_owned)
                            } else {
                                let ty = ty.to_token_stream().to_string();
                                format!(
                                    "Unmanaged<{}>.fromOpaque({}.ptr).takeRetainedValue()",
                                    ty, call_rust
                                )
                            }
                        }
                    }
                }
            }
        }
    };

    let returns_null = Some(BridgedType::StdLib(StdLibType::Null))
        == BridgedType::new_with_return_type(&function.func.sig.output, types);

    let maybe_return = if returns_null || function.is_swift_initializer {
        ""
    } else {
        "return "
    };

    let mut maybe_generics = HashSet::new();

    for arg in function.func.sig.inputs.iter() {
        let bridged_arg = BridgedType::new_with_fn_arg(arg, types);
        if bridged_arg.is_none() {
            continue;
        }
        let bridged_arg = bridged_arg.unwrap();

        let arg_name = fn_arg_name(arg).unwrap().to_string();

        if bridged_arg.contains_owned_string_recursive() {
            maybe_generics.insert(SwiftFuncGenerics::String);
        } else if bridged_arg.contains_ref_string_recursive() {
            maybe_generics.insert(SwiftFuncGenerics::Str);
        }

        // TODO: Refactor to make less duplicative
        match bridged_arg {
            BridgedType::StdLib(StdLibType::Str) => {
                call_rust = format!(
                    r#"{maybe_return}{arg}.toRustStr({{ {arg}AsRustStr in
{indentation}        {call_rust}
{indentation}    }})"#,
                    maybe_return = maybe_return,
                    indentation = indentation,
                    arg = arg_name,
                    call_rust = call_rust
                );
            }
            BridgedType::StdLib(StdLibType::Option(briged_opt))
                if briged_opt.ty.deref() == &BridgedType::StdLib(StdLibType::Str) =>
            {
                call_rust = format!(
                    r#"{maybe_return}optionalRustStrToRustStr({arg}, {{ {arg}AsRustStr in
{indentation}        {call_rust}
{indentation}    }})"#,
                    maybe_return = maybe_return,
                    indentation = indentation,
                    arg = arg_name,
                    call_rust = call_rust
                );
            }
            _ => {}
        }
    }

    if function.is_swift_initializer {
        call_rust = format!("self.init(ptr: {})", call_rust)
    }

    let maybe_return = if function.is_swift_initializer {
        "".to_string()
    } else {
        function.to_swift_return_type(types)
    };

    let maybe_generics = if maybe_generics.is_empty() {
        "".to_string()
    } else {
        let mut m = vec![];

        let generics: Vec<SwiftFuncGenerics> = maybe_generics.into_iter().collect();

        for generic in generics {
            m.push(generic.as_bound())
        }

        format!("<{}>", m.join(", "))
    };

    let func_definition = if function.sig.asyncness.is_some() {
        let func_ret_ty = function.return_ty_built_in(types).unwrap();
        let rust_fn_ret_ty = func_ret_ty.to_swift_type(TypePosition::FnReturn(HostLang::Rust));

        let (maybe_on_complete_sig_ret_val, on_complete_ret_val) = if func_ret_ty.is_null() {
            ("".to_string(), "()".to_string())
        } else {
            (
                format!(
                    ", rustFnRetVal: {}",
                    func_ret_ty.to_swift_type(TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy)
                ),
                func_ret_ty.convert_ffi_value_to_swift_value(
                    "rustFnRetVal",
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy,
                ),
            )
        };

        let fn_body = format!(
            r#"class CbWrapper {{
    var cb: (Result<{rust_fn_ret_ty}, Never>) -> ()

    init(cb: @escaping (Result<{rust_fn_ret_ty}, Never>) -> ()) {{
        self.cb = cb
    }}
}}

func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?{maybe_on_complete_sig_ret_val}) {{
    let wrapper = Unmanaged<CbWrapper>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
    wrapper.cb(.success({on_complete_ret_val}))
}}

return await withCheckedContinuation({{ (continuation: CheckedContinuation<{rust_fn_ret_ty}, Never>) in
    let callback = {{ rustFnRetVal in
        continuation.resume(with: rustFnRetVal)
    }}

    let wrapper = CbWrapper(cb: callback)
    let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

    {call_rust}
}})"#,
            rust_fn_ret_ty = rust_fn_ret_ty,
            maybe_on_complete_sig_ret_val = maybe_on_complete_sig_ret_val,
            on_complete_ret_val = on_complete_ret_val,
            call_rust = call_rust,
        );

        let mut fn_body_indented = "".to_string();
        for line in fn_body.lines() {
            if line.len() > 0 {
                fn_body_indented += &format!("{}    {}\n", indentation, line);
            } else {
                fn_body_indented += "\n"
            }
        }
        let fn_body_indented = fn_body_indented.trim_end();

        format!(
            r#"{indentation}{maybe_static_class_func}{swift_class_func_name}{maybe_generics}({params}) async{maybe_ret} {{
{fn_body_indented}
{indentation}}}"#,
            indentation = indentation,
            maybe_static_class_func = maybe_static_class_func,
            swift_class_func_name = swift_class_func_name,
            maybe_generics = maybe_generics,
            params = params,
            maybe_ret = maybe_return,
            fn_body_indented = fn_body_indented,
        )
    } else {
        format!(
            r#"{indentation}{maybe_static_class_func}{swift_class_func_name}{maybe_generics}({params}){maybe_ret} {{
{indentation}    {call_rust}
{indentation}}}"#,
            indentation = indentation,
            maybe_static_class_func = maybe_static_class_func,
            swift_class_func_name = swift_class_func_name,
            maybe_generics = maybe_generics,
            params = params,
            maybe_ret = maybe_return,
            call_rust = call_rust
        )
    };

    func_definition
}

fn gen_function_exposes_swift_to_rust(
    func: &ParsedExternFn,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let link_name = func.link_name();
    let prefixed_fn_name = func.prefixed_fn_name();
    let fn_name = if let Some(swift_name) = func.swift_name_override.as_ref() {
        swift_name.value()
    } else {
        func.sig.ident.to_string()
    };

    let params = func.to_swift_param_names_and_types(true, types);
    let ret = func.to_swift_return_type(types);

    let args = func.to_swift_call_args(false, true, types, swift_bridge_path);
    let mut call_fn = format!("{}({})", fn_name, args);

    if let Some(built_in) = BridgedType::new_with_return_type(&func.sig.output, types) {
        call_fn = built_in.convert_swift_expression_to_ffi_compatible(
            &call_fn,
            TypePosition::FnReturn(func.host_lang),
        );

        if let Some(associated_type) = func.associated_type.as_ref() {
            let ty_name = match associated_type {
                TypeDeclaration::Shared(_) => {
                    //
                    todo!()
                }
                TypeDeclaration::Opaque(associated_type) => associated_type.ident.to_string(),
            };

            if func.is_method() {
                call_fn = format!(
                    "Unmanaged<{ty_name}>.fromOpaque(this).takeUnretainedValue().{call_fn}",
                    ty_name = ty_name,
                    call_fn = call_fn
                );
            } else if func.is_swift_initializer {
                call_fn = format!(
                    "__private__PointerToSwiftType(ptr: Unmanaged.passRetained({}({})).toOpaque())",
                    ty_name, args
                );
            } else {
                call_fn = format!("{}::{}", ty_name, call_fn);
            }
        }
    } else {
        todo!("Push to ParsedErrors")
    };

    let generated_func = format!(
        r#"@_cdecl("{link_name}")
func {prefixed_fn_name} ({params}){ret} {{
    {call_fn}
}}
"#,
        link_name = link_name,
        prefixed_fn_name = prefixed_fn_name,
        params = params,
        ret = ret,
        call_fn = call_fn
    );

    generated_func
}

#[cfg(test)]
mod tests {
    //! More tests can be found in src/codegen/codegen_tests.rs and its submodules.

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
func __swift_bridge__Foo_new (_ a: UInt8) -> __private__PointerToSwiftType {
    __private__PointerToSwiftType(ptr: Unmanaged.passRetained(Foo(a: a)).toOpaque())
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

    override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }
}
extension Foo {
    convenience init() {
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

    override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }
}
extension Foo {
    convenience init(_ val: UInt8) {
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

    init(ptr: UnsafeMutableRawPointer) {
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

    init(ptr: UnsafeMutableRawPointer) {
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

    init(ptr: UnsafeMutableRawPointer) {
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
    __swift_bridge__$some_function(__private__PointerToSwiftType(ptr: Unmanaged.passRetained(arg).toOpaque()))
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
    Unmanaged<Foo>.fromOpaque(__swift_bridge__$some_function().ptr).takeRetainedValue()
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
