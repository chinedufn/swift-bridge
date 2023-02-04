use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we can add argument labels.
mod argument_label {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn some_function(
                        #[swift_bridge(label = "argumentLabel1")] parameter_name1: i32,
                        #[swift_bridge(label = "argumentLabel2")] parameter_name2: u32,
                    );
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            fn __swift_bridge__some_function(parameter_name1: i32, parameter_name2: u32) {
                super::some_function(parameter_name1, parameter_name2)
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function(argumentLabel1 parameter_name1: Int32, argumentLabel2 parameter_name2: UInt32) {
    __swift_bridge__$some_function(parameter_name1, parameter_name2)
}
            
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(int32_t parameter_name1, uint32_t parameter_name2);
"#,
        )
    }

    #[test]
    fn argument_label() {

        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we can add argument labels.
mod argument_one_label {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn some_function(
                        #[swift_bridge(label = "argumentLabel1")] parameter_name1: i32,
                        parameter_name2: u32,
                    );
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            fn __swift_bridge__some_function(parameter_name1: i32, parameter_name2: u32) {
                super::some_function(parameter_name1, parameter_name2)
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function(argumentLabel1 parameter_name1: Int32, _ parameter_name2: UInt32) {
    __swift_bridge__$some_function(parameter_name1, parameter_name2)
}
            
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(int32_t parameter_name1, uint32_t parameter_name2);
"#,
        )
    }

    #[test]
    fn argument_label() {

        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}