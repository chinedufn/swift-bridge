use crate::{ParsedExternFn, TypeMethod, SWIFT_BRIDGE_PREFIX};
use proc_macro2::Span;
use syn::ForeignItemType;

pub(crate) struct ExternRustSection {
    pub types: Vec<ExternRustSectionType>,
    pub freestanding_fns: Vec<ParsedExternFn>,
}

pub(crate) struct ExternRustSectionType {
    /// `type Foo`
    pub ty: ForeignItemType,
    /// fn bar (&self);
    /// fn buzz (self: &Foo) -> u8;
    /// ... etc
    pub methods: Vec<TypeMethod>,
}

impl ExternRustSectionType {
    pub fn new(ty: ForeignItemType) -> Self {
        ExternRustSectionType {
            ty,
            methods: vec![],
        }
    }
}

impl ExternRustSection {
    /// Gererate the corresponding Swift code for an `extern "Rust"` module.
    pub fn generate_swift(&self) -> String {
        let mut swift = "".to_string();

        for freestanding in &self.freestanding_fns {
            let fn_name = freestanding.func.sig.ident.to_string();

            let params = freestanding.to_swift_param_names_and_types();
            let ret = freestanding.to_swift_return();
            let call_args = freestanding.to_rust_call_args();

            let call_fn = format!("{}({})", fn_name, call_args);

            let func = format!(
                r#"
func {fn_name} ({params}){ret} {{
    {prefix}${call_fn}
}} 
"#,
                fn_name = fn_name,
                params = params,
                prefix = SWIFT_BRIDGE_PREFIX,
                ret = ret,
                call_fn = call_fn,
            );
            swift += &func;
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
func foo (bar: UInt8) {
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
    private var ptr: PtrToRust
    
    init() {
        fatalError("No swift_bridge(constructor) attribute provided.")
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
    private var ptr: PtrToRust
    
    init() {
        ptr = __swift_bridge__$Foo$new()
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we generated a Swift class with an init method with params.
    #[test]
    fn class_with_init_and_params() {
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
    private var ptr: PtrToRust
    
    init(val: UInt8) {
        ptr = __swift_bridge__$Foo$new(val)
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }

    /// Verify that we can generate a class method.
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
    private var ptr: PtrToRust
    
    init(val: UInt8) {
        fatalError("No swift_bridge(constructor) attribute provided.")
    }
    
    bar() {
        __swift_bridge__$Foo$bar()
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
    private var ptr: PtrToRust
    
    init(val: UInt8) {
        fatalError("No swift_bridge(constructor) attribute provided.")
    }
    
    class bar() {
        __swift_bridge__$Foo$bar()
    }
}
"#;

        assert_eq!(generated.trim(), expected.trim());
    }
}
