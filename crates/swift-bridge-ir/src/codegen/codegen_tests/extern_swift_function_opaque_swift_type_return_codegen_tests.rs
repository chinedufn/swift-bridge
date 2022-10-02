use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we generate the proper code for extern "Swift" functions that returns an
/// opaque Swift type.
mod test_extern_swift_freestanding_function_owned_opaque_swift_type_return {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Swift" {
                    type SomeType;

                    fn some_function() -> SomeType;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                pub fn some_function () -> SomeType {
                    unsafe { __swift_bridge__some_function() }
                }
            },
            quote! {
                #[link_name = "__swift_bridge__$some_function"]
                fn __swift_bridge__some_function() -> SomeType;
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function () -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(some_function()).toOpaque()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim("")
    }

    #[test]
    fn test_extern_swift_freestanding_function_owned_opaque_swift_type_return() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the proper code for extern "Swift" methods that returns an
/// opaque Swift type.
mod test_extern_swift_method_owned_opaque_swift_type_return {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Swift" {
                    type SomeType;

                    fn some_method(&self) -> SomeType;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                impl SomeType {
                    pub fn some_method (&self) -> SomeType {
                        unsafe { __swift_bridge__SomeType_some_method(swift_bridge::PointerToSwiftType(self.0)) }
                    }
                }
            },
            quote! {
                #[link_name = "__swift_bridge__$SomeType$some_method"]
                fn __swift_bridge__SomeType_some_method(this: swift_bridge::PointerToSwiftType) -> SomeType;
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
@_cdecl("__swift_bridge__$SomeType$some_method")
func __swift_bridge__SomeType_some_method (_ this: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(Unmanaged<SomeType>.fromOpaque(this).takeUnretainedValue().some_method()).toOpaque()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim("")
    }

    #[test]
    fn test_extern_swift_method_owned_opaque_swift_type_return() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
