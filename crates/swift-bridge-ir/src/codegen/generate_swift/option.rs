#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_generated_contains_expected, parse_ok};
    use quote::quote;

    /// Verify that we generate correct code for an extern "Swift" block that returns an Option<T>
    /// where T is a primitive.
    #[test]
    fn freestanding_swift_function_return_option_primitive() {
        let tokens = quote! {
            mod foo {
                extern "Swift" {
                    fn foo () -> Option<u8>;
                }
            }
        };
        let module = parse_ok(tokens);
        let generated = module.generate_swift();

        let expected = r#"
@_cdecl("__swift_bridge__$foo")
func __swift_bridge__foo () -> UInt8 {
    if case let val? = foo() { return markReturnTypeSome(val); } else { return markReturnTypeNone(); }
} 
"#;

        assert_generated_contains_expected(generated.trim(), expected.trim());
    }

    /// Verify that we generate correct code for an extern "Rust" block that returns an Option<T>
    /// where T is a primitive.
    #[test]
    fn freestanding_rust_function_return_option_primitive() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo () -> Option<u8>;
                }
            }
        };
        let module = parse_ok(tokens);
        let generated = module.generate_swift();

        let expected = r#"
func foo() -> Option<UInt8> {
    let val = __swift_bridge__$foo(); if _get_option_return() { return val; } else { return nil; }
} 
"#;

        assert_eq!(generated.trim(), expected.trim());
    }
}
