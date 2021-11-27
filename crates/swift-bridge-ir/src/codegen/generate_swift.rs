use crate::built_in_types::BuiltInType;
use crate::parse::{HostLang, TypeDeclarations};
use crate::parsed_extern_fn::ParsedExternFn;
use crate::{
    BridgedType, OpaqueForeignType, SharedType, StructSwiftRepr, SwiftBridgeModule,
    SWIFT_BRIDGE_PREFIX,
};
use quote::ToTokens;
use std::collections::HashMap;
use std::ops::Deref;
use syn::{ReturnType, Type};

mod option;

impl SwiftBridgeModule {
    /// Generate the corresponding Swift code for the bridging module.
    pub fn generate_swift(&self) -> String {
        let mut swift = "".to_string();

        let mut associated_funcs_and_methods: HashMap<String, Vec<&ParsedExternFn>> =
            HashMap::new();

        for function in &self.functions {
            if function.host_lang.is_rust() {
                if let Some(ty) = function.associated_type.as_ref() {
                    match ty {
                        BridgedType::Shared(_) => {
                            //
                            todo!("Think about what to do here..")
                        }
                        BridgedType::Opaque(ty) => {
                            associated_funcs_and_methods
                                .entry(ty.ident.to_string())
                                .or_default()
                                .push(function);
                        }
                    };
                    continue;
                }
            }

            let func_definition = match function.host_lang {
                HostLang::Rust => gen_func_swift_calls_rust(function, &self.types),
                HostLang::Swift => gen_function_exposes_swift_to_rust(function, &self.types),
            };

            swift += &func_definition;
            swift += "\n";
        }

        for ty in self.types.types() {
            let generated = match ty {
                BridgedType::Shared(SharedType::Struct(shared_struct)) => {
                    match shared_struct.swift_repr {
                        StructSwiftRepr::Class => {
                            todo!()
                        }
                        StructSwiftRepr::Structure => {
                            // No need to generate any code. Swift will automatically generate a
                            //  struct from our C header typedef that we generate for this struct.

                            continue;
                        }
                    }
                }
                BridgedType::Opaque(ty) => match ty.host_lang {
                    HostLang::Rust => {
                        generate_swift_class(ty, &associated_funcs_and_methods, &self.types)
                    }
                    HostLang::Swift => generate_drop_swift_instance_reference_count(ty),
                },
            };

            swift += &generated;
            swift += "\n";
        }

        swift
    }
}

fn generate_swift_class(
    ty: &OpaqueForeignType,
    associated_funcs_and_methods: &HashMap<String, Vec<&ParsedExternFn>>,
    types: &TypeDeclarations,
) -> String {
    let type_name = ty.ident.to_string();

    let mut initializers = vec![];
    let mut instance_methods = vec![];

    let default_init = r#"    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }"#;

    if let Some(methods) = associated_funcs_and_methods.get(&type_name) {
        for type_method in methods {
            // TODO: Normalize with freestanding func codegen above

            let func_definition = gen_func_swift_calls_rust(type_method, types);

            if type_method.is_initializer {
                initializers.push(func_definition);
            } else {
                instance_methods.push(func_definition);
            }
        }
    }

    if initializers.len() == 0 {
        initializers.push(default_init.to_string());
    }

    let initializers: String = initializers.join("\n\n");
    let mut instance_methods: String = instance_methods.join("\n\n");
    if instance_methods.len() > 0 {
        instance_methods = format!("\n\n{}", instance_methods);
    }

    let free_func_call = format!("{}${}$_free(ptr)", SWIFT_BRIDGE_PREFIX, type_name);

    let class = format!(
        r#"
public class {type_name} {{
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

{initializers}

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {{
        self.ptr = ptr
        self.isOwned = isOwned
    }}

    deinit {{
        if isOwned {{
            {free_func_call}
        }}
    }}{instance_methods}
}}"#,
        type_name = type_name,
        initializers = initializers,
        instance_methods = instance_methods,
        free_func_call = free_func_call
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
fn generate_drop_swift_instance_reference_count(ty: &OpaqueForeignType) -> String {
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

fn gen_func_swift_calls_rust(function: &ParsedExternFn, types: &TypeDeclarations) -> String {
    let fn_name = function.sig.ident.to_string();
    let params = function.to_swift_param_names_and_types(false, types);
    let call_args = function.to_swift_call_args(true, false, types);
    let call_fn = format!("{}({})", fn_name, call_args);

    let type_name_segment = if let Some(ty) = function.associated_type.as_ref() {
        match ty {
            BridgedType::Shared(_) => {
                //
                todo!()
            }
            BridgedType::Opaque(ty) => {
                format!("${}", ty.ident.to_string())
            }
        }
    } else {
        "".to_string()
    };

    let maybe_static_class_func = if function.associated_type.is_some()
        && (!function.is_method() && !function.is_initializer)
    {
        "class "
    } else {
        ""
    };

    let maybe_return = if function.is_initializer {
        "".to_string()
    } else {
        function.to_swift_return_type(false, types)
    };

    let swift_class_func_name = if function.is_initializer {
        "init".to_string()
    } else {
        format!("func {}", fn_name.as_str())
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
    let call_rust = if function.is_initializer {
        format!("ptr = {}", call_rust)
    } else if let Some(built_in) = function.return_ty_built_in() {
        built_in.convert_ffi_value_to_swift_value(&call_rust)
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
                        BridgedType::Shared(_) => call_rust,
                        BridgedType::Opaque(opaque) => {
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

    let func_definition = format!(
        r#"{indentation}{maybe_static_class_func}{swift_class_func_name}({params}){maybe_ret} {{
{indentation}    {call_rust}
{indentation}}}"#,
        indentation = indentation,
        maybe_static_class_func = maybe_static_class_func,
        swift_class_func_name = swift_class_func_name,
        params = params,
        maybe_ret = maybe_return,
        call_rust = call_rust
    );

    func_definition
}

fn gen_function_exposes_swift_to_rust(func: &ParsedExternFn, types: &TypeDeclarations) -> String {
    let link_name = func.link_name();
    let prefixed_fn_name = func.prefixed_fn_name();
    let fn_name = if let Some(swift_name) = func.swift_name_override.as_ref() {
        swift_name.value()
    } else {
        func.sig.ident.to_string()
    };

    let params = func.to_swift_param_names_and_types(true, types);
    let ret = func.to_swift_return_type(true, types);

    let args = func.to_swift_call_args(false, true, types);
    let mut call_fn = format!("{}({})", fn_name, args);

    if let Some(associated_type) = func.associated_type.as_ref() {
        let ty_name = match associated_type {
            BridgedType::Shared(_) => {
                //
                todo!()
            }
            BridgedType::Opaque(associated_type) => associated_type.ident.to_string(),
        };

        if func.is_method() {
            call_fn = format!(
                "Unmanaged<{ty_name}>.fromOpaque(this).takeUnretainedValue().{call_fn}",
                ty_name = ty_name,
                call_fn = call_fn
            );
        } else if func.is_initializer {
            call_fn = format!("Unmanaged.passRetained({}({})).toOpaque()", ty_name, args);
        } else {
            call_fn = format!("{}::{}", ty_name, call_fn);
        }
    }

    if let Some(built_in) = BuiltInType::new_with_return_type(&func.sig.output) {
        call_fn = built_in.convert_swift_expression_to_ffi_compatible(&call_fn, func.host_lang);
    }

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
    use crate::test_utils::assert_generated_contains_expected;
    use crate::SwiftBridgeModule;
    use quote::quote;
    use syn::parse_quote;

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
        let generated = module.generate_swift();

        let expected = r#"
func foo() {
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
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$foo")
func __swift_bridge__foo () {
    foo()
} 
"#;

        assert_generated_contains_expected(generated.trim(), expected.trim());
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
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$foo")
func __swift_bridge__foo () -> __private__FfiSlice {
    foo().toFfiSlice()
} 
"#;

        assert_generated_contains_expected(generated.trim(), expected.trim());
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
        let generated = module.generate_swift();

        let expected = r#"
func foo(_ bar: UInt8) {
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
        let generated = module.generate_swift();

        let expected = r#"
func foo() -> UInt32 {
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
        let generated = module.generate_swift();

        let expected = r#"
func foo() -> UnsafeBufferPointer<UInt8> {
    let slice = __swift_bridge__$foo(); return UnsafeBufferPointer(start: slice.start.assumingMemoryBound(to: UInt8.self), count: Int(slice.len));
} 
"#;

        assert_generated_contains_expected(&generated, &expected);
    }

    /// Verify that we generated a Swift class for a Rust type.
    #[test]
    fn generate_class() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift();

        let expected = r#"
public class Foo {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
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
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$Foo$_free")
func __swift_bridge__Foo__free (ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<Foo>.fromOpaque(ptr).takeRetainedValue()
}
"#;

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
public class Foo {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init() {
        ptr = __swift_bridge__$Foo$new()
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we generated a function that Rust can use to reduce a Swift class instance's
    /// reference count.
    #[test]
    fn extern_swift_claas_init() {
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
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$Foo$new")
func __swift_bridge__Foo_new (_ a: UInt8) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(Foo(a: a)).toOpaque()
}
"#;

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
public class Foo {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init(_ val: UInt8) {
        ptr = __swift_bridge__$Foo$new(val)
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we can generate an instance method.
    #[test]
    fn instance_method() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    fn bar(&self);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift();

        let expected = r#"
public class Foo {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }

    func bar() {
        __swift_bridge__$Foo$bar(ptr)
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
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
        let generated = module.generate_swift();

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

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
public class Foo {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }

    func bar() -> UInt8 {
        __swift_bridge__$Foo$bar(ptr)
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
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
        let generated = module.generate_swift();

        let expected = r#"
public class Foo {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }

    func bar(_ other: Foo) {
        __swift_bridge__$Foo$bar(ptr, other.ptr)
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we can generate a static class mehod.
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
        let generated = module.generate_swift();

        let expected = r#"
public class Foo {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$Foo$_free(ptr)
        }
    }

    class func bar() {
        __swift_bridge__$Foo$bar()
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
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
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$Foo$bar")
func __swift_bridge__Foo_bar (_ arg: UInt8) {
    Foo::bar(arg: arg)
}
"#;

        assert_generated_contains_expected(&generated, expected);
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
        let generated = module.generate_swift();

        let expected = r#"
func foo() -> RustString {
    RustString(ptr: __swift_bridge__$foo(), isOwned: true)
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we properly generate a Swift function that returns an &String.
    #[test]
    fn return_string_reference() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo () -> &String;
                }
            }
        };
        let module: SwiftBridgeModule = syn::parse2(tokens).unwrap();
        let generated = module.generate_swift();

        let expected = r#"
func foo() -> RustString {
    RustString(ptr: __swift_bridge__$foo(), isOwned: false)
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
        let generated = module.generate_swift();

        let expected = r#"
func void_pointer(_ arg1: UnsafeRawPointer) {
    __swift_bridge__$void_pointer(UnsafeMutableRawPointer(mutating: arg1))
}
"#;

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
func void_pointer() -> UnsafeRawPointer {
    UnsafeRawPointer(__swift_bridge__$void_pointer()!)
}
"#;

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$void_pointer")
func __swift_bridge__void_pointer (_ arg: UnsafeRawPointer) {
    void_pointer(arg: arg)
}
"#;

        assert_generated_contains_expected(&generated, &expected);
    }

    /// Verify that we can return a reference to a declared type.
    #[test]
    fn extern_rust_return_owned_declared_type() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    fn get_owned_foo () -> Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift();

        let expected = r#"
func get_owned_foo() -> Foo {
    Foo(ptr: __swift_bridge__$get_owned_foo(), isOwned: true)
}
"#;

        assert_generated_contains_expected(&generated, &expected);
    }

    /// Verify that we can return a reference to a declared type.
    #[test]
    fn extern_rust_return_reference_to_declared_type() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    fn get_ref_foo () -> &Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift();

        let expected = r#"
func get_ref_foo() -> Foo {
    Foo(ptr: __swift_bridge__$get_ref_foo(), isOwned: false)
}
"#;

        assert_generated_contains_expected(&generated, &expected);
    }

    /// Verify that we can return a shared struct type.
    #[test]
    fn extern_rust_return_shared_struct() {
        let tokens = quote! {
            mod ffi {
                struct Foo;

                extern "Rust" {
                    fn get_foo () -> Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift();

        let expected = r#"
func get_foo() -> Foo {
    __swift_bridge__$get_foo()
}
"#;

        assert_generated_contains_expected(&generated, &expected);
    }

    /// Verify that we can take a shared struct as an argument.
    #[test]
    fn extern_rust_shared_struct_arg() {
        let tokens = quote! {
            mod ffi {
                struct Foo;

                extern "Rust" {
                    fn some_function (arg: Foo);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift();

        let expected = r#"
func some_function(_ arg: Foo) {
    __swift_bridge__$some_function(arg)
}
"#;

        assert_generated_contains_expected(&generated, &expected);
    }

    /// Verify that we rename shared struct arguments and return values if there is a swift_name
    /// attribute.
    #[test]
    fn extern_rust_fn_uses_swift_name_for_shared_struct_attrib() {
        let tokens = quote! {
            mod ffi {
                #[swift_bridge(swift_name = "Renamed")]
                struct Foo;

                extern "Rust" {
                    fn some_function (arg: Foo) -> Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.generate_swift();

        let expected = r#"
func some_function(_ arg: Renamed) -> Renamed {
    __swift_bridge__$some_function(arg)
}
"#;

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
func some_function(_ arg: Foo) {
    __swift_bridge__$some_function(Unmanaged.passRetained(arg).toOpaque())
}
"#;

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
func some_function() -> Foo {
    Unmanaged<Foo>.fromOpaque(__swift_bridge__$some_function().ptr).takeRetainedValue()
}
"#;

        assert_generated_contains_expected(&generated, &expected);
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
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function () {
    someFunctionSwiftName()
}
"#;

        assert_generated_contains_expected(&generated, &expected);
    }
}
