#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_trimmed_generated_contains_trimmed_expected, parse_ok};
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
    if case let val? = foo() { _set_option_return(true); return val; } else { _set_option_return(false); return 123; }
} 
"#;

        assert_trimmed_generated_contains_trimmed_expected(generated.trim(), expected.trim());
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
func foo() -> Optional<UInt8> {
    let val = __swift_bridge__$foo(); if _get_option_return() { return val; } else { return nil; }
} 
"#;

        assert_eq!(generated.trim(), expected.trim());
    }
}
