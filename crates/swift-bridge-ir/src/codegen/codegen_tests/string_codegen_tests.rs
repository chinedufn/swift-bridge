use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Test code generation for Rust function that takes an owned String argument.
mod extern_rust_fn_with_owned_string_argument {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    fn some_function (arg: String);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: *mut swift_bridge::string::RustString
            ) {
                super::some_function(unsafe { Box::from_raw(arg).0 })
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: RustString) {
    __swift_bridge__$some_function({arg.isOwned = false; return arg.ptr}())
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim(
            r#"
void __swift_bridge__$some_function(void* arg);
    "#,
        )
    }

    #[test]
    fn extern_rust_fn_with_owned_string_argument() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that takes an &str argument.
mod extern_rust_fn_with_str_argument {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    fn some_function (arg: &str);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: swift_bridge::string::RustStr
            ) {
                super::some_function(arg.to_str())
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: String) {
    arg.utf8CString.withUnsafeBufferPointer({argPtr in
        __swift_bridge__$some_function(RustStr(start: UnsafeMutableRawPointer(mutating: argPtr.baseAddress!).assumingMemoryBound(to: UInt8.self), len: UInt(arg.count)))
    })
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim(
            r#"
void __swift_bridge__$some_function(struct RustStr arg);
    "#,
        )
    }

    #[test]
    fn extern_rust_fn_with_str_argument() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that returns an owned String argument.
mod extern_rust_fn_returns_string {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> String;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> *mut swift_bridge::string::RustString {
                swift_bridge::string::RustString(super::some_function()).box_into_raw()
            }
        })
    }

    const EXPECTED_SWIFT_CODE: ExpectedSwiftCode = ExpectedSwiftCode::ContainsAfterTrim(
        r#"
func some_function() -> RustString {
    RustString(ptr: __swift_bridge__$some_function(), isOwned: true)
}
"#,
    );

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
void* __swift_bridge__$some_function(void);
    "#,
    );

    #[test]
    fn extern_rust_fn_returns_string() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT_CODE,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that returns a &str.
mod extern_rust_fn_return_str {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> &str;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> swift_bridge::string::RustStr {
                swift_bridge::string::RustStr::from_str(super::some_function())
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function() -> RustStr {
    __swift_bridge__$some_function()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim(
            r#"
struct RustStr __swift_bridge__$some_function(void);
    "#,
        )
    }

    #[test]
    fn extern_rust_fn_return_str() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
