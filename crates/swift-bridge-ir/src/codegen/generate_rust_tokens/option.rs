#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_tokens_contain, parse_ok};
    use quote::{quote, ToTokens};

    /// Verify that we can return an Option<u8> from a function in an extern "Rust" block.
    /// Our integration tests can more thoroughly test other primitive types.
    #[test]
    fn extern_rust_freestanding_function_return_primitive_option() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;
                    fn some_function () -> Option<u8>;
                }
            }
        };
        let module = parse_ok(start);
        let tokens = module.to_token_stream();

        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function () -> u8 {
                if let Some(val) = super::some_function() {
                    swift_bridge::option::_set_option_return(true);
                    val
                } else {
                    swift_bridge::option::_set_option_return(false);
                    <u8 as swift_bridge::option::FfiOptional>::unused_value()
                }
            }
        };

        assert_tokens_contain(&tokens, &expected);
    }

    /// Verify that we can return an Option<u8> from a function in an extern "Swift" block.
    /// Our integration tests can more thoroughly test other primitive types.
    #[test]
    fn extern_swift_freestanding_function_return_primitive_option() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    fn some_function () -> Option<u8>;
                }
            }
        };
        let module = parse_ok(start);
        let tokens = module.to_token_stream();

        let expected = quote! {
            pub fn some_function() -> Option<u8> {
                let value = unsafe { __swift_bridge__some_function() };
                if swift_bridge::option::_get_option_return() {
                    Some(value)
                } else {
                    None
                }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$some_function"]
                fn __swift_bridge__some_function () -> u8;
            }
        };

        assert_tokens_contain(&tokens, &expected);
    }
}
