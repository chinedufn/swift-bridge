use crate::built_in_types::BuiltInType;
use crate::parse::HostLang;
use crate::parsed_extern_fn::ParsedExternFn;
use crate::{BridgedType, SwiftBridgeModule, SWIFT_BRIDGE_PREFIX};
use std::collections::HashMap;

impl SwiftBridgeModule {
    /// Generate the corresponding Swift code for the bridging module.
    pub fn generate_swift(&self) -> String {
        let mut swift = "".to_string();

        let mut associated_funcs_and_methods: HashMap<String, Vec<&ParsedExternFn>> =
            HashMap::new();

        for function in &self.functions {
            if function.host_lang.is_rust() {
                if let Some(ty) = function.associated_type.as_ref() {
                    associated_funcs_and_methods
                        .entry(ty.ident.to_string())
                        .or_default()
                        .push(function);
                    continue;
                }
            }

            let func_definition = match function.host_lang {
                HostLang::Rust => gen_func_swift_calls_rust(function),
                HostLang::Swift => gen_function_exposes_swift_to_rust(function),
            };

            swift += &func_definition;
            swift += "\n";
        }

        for ty in &self.types {
            let generated = match ty.host_lang {
                HostLang::Rust => generate_swift_class(ty, &associated_funcs_and_methods),
                HostLang::Swift => generate_drop_swift_instance_reference_count(ty),
            };

            swift += &generated;
            swift += "\n";
        }

        swift
    }
}

fn generate_swift_class(
    ty: &BridgedType,
    associated_funcs_and_methods: &HashMap<String, Vec<&ParsedExternFn>>,
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

            let func_definition = gen_func_swift_calls_rust(type_method);

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

{initializers}

    init(ptr: UnsafeMutableRawPointer) {{
        self.ptr = ptr
    }}

    deinit {{
        {free_func_call}
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
fn generate_drop_swift_instance_reference_count(ty: &BridgedType) -> String {
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

fn gen_func_swift_calls_rust(function: &ParsedExternFn) -> String {
    let fn_name = function.sig.ident.to_string();
    let params = function.to_swift_param_names_and_types(false);
    let call_args = function.to_swift_call_args(true, false);
    let call_fn = format!("{}({})", fn_name, call_args);

    let type_name_segment = if let Some(ty) = function.associated_type.as_ref() {
        format!("${}", ty.ident.to_string())
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
        function.to_swift_return(false)
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
        built_in.wrap_swift_called_rust_ffi_returned_value(&call_rust)
    } else {
        call_rust
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

fn gen_function_exposes_swift_to_rust(func: &ParsedExternFn) -> String {
    let link_name = func.link_name();
    let prefixed_fn_name = func.prefixed_fn_name();
    let fn_name = &func.sig.ident;

    let params = func.to_swift_param_names_and_types(true);
    let ret = func.to_swift_return(true);

    let args = func.to_swift_call_args(false, true);
    let mut call_fn = format!("{}({})", fn_name, args);

    if let Some(associated_type) = func.associated_type.as_ref() {
        let ty_name = associated_type.ident.to_string();

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

    if let Some(built_in) = BuiltInType::with_return_type(&func.sig.output) {
        match built_in {
            BuiltInType::RefSlice(ref_slice) => {
                // TODO: Move this wrapping logic into the BuiltInType file behind a match statement.
                //  This way all of our type conversions are in one place.
                call_fn = format!(
                    r#"let buffer_pointer = {}
    return __private__RustSlice(start: UnsafeMutablePointer(mutating: buffer_pointer.baseAddress), len: UInt(buffer_pointer.count))"#,
                    call_fn,
                );
            }
            _ => {}
        };
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
func __swift_bridge__foo () -> RustSlice_uint8_t {
    let buffer_pointer = foo()
    return RustSlice_uint8_t(start: UnsafeMutablePointer(mutating: buffer_pointer.baseAddress), len: UInt(buffer_pointer.count))
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
    let slice = __swift_bridge__$foo(); return UnsafeBufferPointer(start: slice.start, count: Int(slice.len));
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

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
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

    init() {
        ptr = __swift_bridge__$Foo$new()
    }

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
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

    init(_ val: UInt8) {
        ptr = __swift_bridge__$Foo$new(val)
    }

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
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

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
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

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
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

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
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

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
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

        assert_generated_contains_expected(generated.trim(), expected.trim());
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
    RustString(ptr: __swift_bridge__$foo())
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    fn assert_generated_contains_expected(generated: &str, expected: &str) {
        assert!(
            generated.trim().contains(&expected.trim()),
            r#"Expected was not contained by generated.
Generated:
{}
Expected:
{}"#,
            generated.trim(),
            expected.trim()
        );
    }
}
