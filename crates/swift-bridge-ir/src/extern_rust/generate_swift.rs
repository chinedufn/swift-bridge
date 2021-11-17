use crate::extern_rust::ExternRustSection;
use crate::SWIFT_BRIDGE_PREFIX;

impl ExternRustSection {
    /// Gererate the corresponding Swift code for an `extern "Rust"` module.
    pub fn generate_swift(&self) -> String {
        let mut swift = "".to_string();

        for freestanding in &self.freestanding_fns {
            // TODO: Normalize with method codegen above
            let fn_name = freestanding.sig.ident.to_string();

            let params = freestanding.to_swift_param_names_and_types();
            let ret = freestanding.to_swift_return();

            let call_args = freestanding.to_swift_call_args();
            let call_fn = format!("{}({})", fn_name, call_args);

            let func = format!(
                r#"
func {fn_name} ({params}){ret} {{
    {prefix}${call_fn}
}} 
"#,
                fn_name = fn_name,
                prefix = SWIFT_BRIDGE_PREFIX,
                params = params,
                ret = ret,
                call_fn = call_fn,
            );
            swift += &func;
        }

        for ty in &self.types {
            let type_name = ty.ty.ident.to_string();

            let mut initializers = vec![];
            let mut instance_methods = vec![];

            let default_init = r#"    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }"#;

            for type_method in &ty.methods {
                // TODO: Normalize with freestanding func codegen above

                let fn_name = type_method.func.sig.ident.to_string();
                let params = type_method.func.to_swift_param_names_and_types();
                let call_args = type_method.func.to_swift_call_args();
                let call_fn = format!("{}({})", fn_name, call_args);

                let maybe_static_class_func =
                    if !type_method.func.is_method() && !type_method.is_initializer {
                        "class "
                    } else {
                        ""
                    };

                let maybe_return = if type_method.is_initializer {
                    "".to_string()
                } else {
                    type_method.func.to_swift_return()
                };

                let (swift_class_func_name, maybe_assign_to_ptr) = if type_method.is_initializer {
                    ("init".to_string(), "ptr = ")
                } else {
                    (format!("func {}", fn_name.as_str()), "")
                };

                let func_definition = format!(
                    r#"    {maybe_static_class_func}{swift_class_func_name}({params}){maybe_ret} {{
        {maybe_assign_to_ptr}{prefix}${type_name}${call_fn}
    }}"#,
                    maybe_static_class_func = maybe_static_class_func,
                    maybe_assign_to_ptr = maybe_assign_to_ptr,
                    swift_class_func_name = swift_class_func_name,
                    params = params,
                    maybe_ret = maybe_return,
                    prefix = SWIFT_BRIDGE_PREFIX,
                    type_name = type_name,
                    call_fn = call_fn
                );

                if type_method.is_initializer {
                    initializers.push(func_definition);
                } else {
                    instance_methods.push(func_definition);
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
    private var ptr: UnsafeMutableRawPointer

{initializers}

    deinit {{
        {free_func_call}
    }}{instance_methods}
}} 
"#,
                type_name = type_name,
                initializers = initializers,
                instance_methods = instance_methods,
                free_func_call = free_func_call
            );

            swift += &class;
        }

        swift
    }
}

#[cfg(test)]
mod tests {
    use crate::SwiftBridgeModule;
    use quote::quote;
    use syn::parse_quote;

    /// Verify that we generated a Swift function to call our freestanding function.
    #[test]
    fn freestanding_function_no_args() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo ();
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
func foo () {
    __swift_bridge__$foo()
} 
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we generated a Swift function to call a freestanding function with one arg.
    #[test]
    fn freestanding_function_one_arg() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo (bar: u8);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
func foo (_ bar: UInt8) {
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
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
func foo () -> UInt32 {
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
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
func foo () -> UnsafeBufferPointer<UInt8> {
    let slice = __swift_bridge__$foo()
    return UnsafeBufferPointer(start: slice.start, len: slice.len)
} 
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we generated a Swift class for a type.
    #[test]
    fn class() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
public class Foo {
    private var ptr: UnsafeMutableRawPointer

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
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
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
public class Foo {
    private var ptr: UnsafeMutableRawPointer

    init() {
        ptr = __swift_bridge__$Foo$new()
    }

    deinit {
        __swift_bridge__$Foo$_free(ptr)
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
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
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
public class Foo {
    private var ptr: UnsafeMutableRawPointer

    init(_ val: UInt8) {
        ptr = __swift_bridge__$Foo$new(val)
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
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
public class Foo {
    private var ptr: UnsafeMutableRawPointer

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
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
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
public class Foo {
    private var ptr: UnsafeMutableRawPointer

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
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

                    fn bar(&self, other: &Foo);
                }
            }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
public class Foo {
    private var ptr: UnsafeMutableRawPointer

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
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
        let generated = module.extern_rust[0].generate_swift();

        let expected = r#"
public class Foo {
    private var ptr: UnsafeMutableRawPointer

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
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
}
