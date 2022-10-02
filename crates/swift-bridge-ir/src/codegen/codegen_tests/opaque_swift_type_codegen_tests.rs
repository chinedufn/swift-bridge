use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Test code generation for freestanding Swift function that takes an opaque Swift type argument.
mod extern_swift_freestanding_fn_with_owned_opaque_swift_type_arg {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Swift" {
                    type MyType;
                    fn some_function (arg: MyType);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub fn some_function (arg: MyType) {
                unsafe { __swift_bridge__some_function (arg) }
            }

            #[repr(C)]
            pub struct MyType(*mut std::ffi::c_void);

            impl Drop for MyType {
                fn drop (&mut self) {
                    unsafe { __swift_bridge__MyType__free(self.0) }
                }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$some_function"]
                fn __swift_bridge__some_function (arg: MyType);

                #[link_name = "__swift_bridge__$MyType$_free"]
                fn __swift_bridge__MyType__free (this: *mut std::ffi::c_void);
            }
        })
    }

    const EXPECTED_SWIFT_CODE: ExpectedSwiftCode = ExpectedSwiftCode::ContainsAfterTrim(
        r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function (_ arg: UnsafeMutableRawPointer) {
    some_function(arg: Unmanaged<MyType>.fromOpaque(arg).takeRetainedValue())
}
"#,
    );

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(r#""#);

    #[test]
    fn extern_swift_freestanding_fn_with_owned_opaque_swift_type_arg() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT_CODE,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}
