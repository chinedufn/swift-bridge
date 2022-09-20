//! See also: crates/swift-integration-tests/src/result.rs

use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Test code generation for Rust function that accepts and returns a Result<T, E>
/// where T and E are Strings.
mod extern_rust_fn_result_string {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function (arg: Result<String, String>);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: swift_bridge::result::ResultPtrAndPtr
            ) {
                super::some_function(
                    if arg.is_ok {
                        std::result::Result::Ok(unsafe { Box::from_raw(arg.ok_or_err as *mut swift_bridge::string::RustString).0 })
                    } else {
                        std::result::Result::Err(unsafe { Box::from_raw(arg.ok_or_err as *mut swift_bridge::string::RustString).0 })
                    }
                )
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function<GenericIntoRustString: IntoRustString>(_ arg: RustResult<GenericIntoRustString, GenericIntoRustString>) {
    __swift_bridge__$some_function({ switch arg { case .Ok(let ok): return __private__ResultPtrAndPtr(is_ok: true, ok_or_err: { let rustString = ok.intoRustString(); rustString.isOwned = false; return rustString.ptr }()) case .Err(let err): return __private__ResultPtrAndPtr(is_ok: false, ok_or_err: { let rustString = err.intoRustString(); rustString.isOwned = false; return rustString.ptr }()) } }())
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
void __swift_bridge__$some_function(struct __private__ResultPtrAndPtr arg);
    "#,
    );

    #[test]
    fn extern_rust_fn_result_string() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that accepts and returns a Result<T, E>
/// where T and E are opaque Rust types.
mod extern_rust_fn_result_opaque_rust {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type SomeType;

                    fn some_function (arg: Result<SomeType, SomeType>);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: swift_bridge::result::ResultPtrAndPtr
            ) {
                super::some_function(
                    if arg.is_ok {
                        std::result::Result::Ok(unsafe { *Box::from_raw(arg.ok_or_err as *mut super::SomeType) })
                    } else {
                        std::result::Result::Err(unsafe { *Box::from_raw(arg.ok_or_err as *mut super::SomeType) })
                    }
                )
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: RustResult<SomeType, SomeType>) {
    __swift_bridge__$some_function({ switch arg { case .Ok(let ok): return __private__ResultPtrAndPtr(is_ok: true, ok_or_err: {ok.isOwned = false; return ok.ptr;}()) case .Err(let err): return __private__ResultPtrAndPtr(is_ok: false, ok_or_err: {err.isOwned = false; return err.ptr;}()) } }())
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
void __swift_bridge__$some_function(struct __private__ResultPtrAndPtr arg);
    "#,
    );

    #[test]
    fn extern_rust_fn_result_opaque_rust() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}
