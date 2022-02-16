use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we properly handle a `#[swift_bridge(already_declared)]` attribute on an opaque
/// Rust type.
mod extern_rust_already_declared_type_attribute {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(already_declared)]
                    type SomeType;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::DoesNotContain(quote! {
            SomeType
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::DoesNotContainAfterTrim(
            r#"
SomeType
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::DoesNotContainAfterTrim(
            r#"
SomeType
    "#,
        )
    }

    #[test]
    fn extern_rust_already_declared_type_attribute() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate associated functions and methods for already declared types.
mod extern_rust_already_declared_type_still_generates_methods {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(already_declared)]
                    type SomeType;

                    fn some_function(self);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            fn __swift_bridge__SomeType_some_function
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(r#"func some_function"#)
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"void __swift_bridge__$SomeType$some_function(void* self);"#,
    );

    #[test]
    fn extern_rust_already_declared_type_still_generates_methods() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Verify that we do not re-declare an already defined struct.
mod already_declared_struct {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                #[swift_bridge(already_declared, swift_repr = "struct")]
                struct FfiSomeType;

                extern "Rust" {
                    fn some_function(arg: FfiSomeType) -> FfiSomeType;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::DoesNotContain(quote! {
            struct FfiSomeType
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::DoesNotContainAfterTrim("struct FfiSomeType")
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim(
            r#"
struct __swift_bridge__$FfiSomeType __swift_bridge__$some_function(struct __swift_bridge__$FfiSomeType arg);
"#,
        )
    }

    #[test]
    fn already_declared_struct() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
