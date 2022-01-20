use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we use the `#[swift_bridge(args_into = (arg1, another_arg))]` attribute.
mod function_args_into_attribute {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                #[swift_bridge(swift_name = "SomeStruct")]
                struct FfiSomeStruct;

                #[swift_bridge(swift_name = "AnotherStruct")]
                struct FfiAnotherStruct(u8);

                extern "Rust" {
                    #[swift_bridge(args_into = (some_arg, another_arg))]
                    fn some_function(some_arg: FfiSomeStruct, another_arg: FfiAnotherStruct, arg3: u8);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub extern "C" fn __swift_bridge__some_function(
                some_arg: __swift_bridge__FfiSomeStruct,
                another_arg: __swift_bridge__FfiAnotherStruct,
                arg3: u8
            ) {
                super::some_function(some_arg.into_rust_repr().into(), another_arg.into_rust_repr().into(), arg3)
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ some_arg: SomeStruct, _ another_arg: AnotherStruct, _ arg3: UInt8) {
    __swift_bridge__$some_function(some_arg.intoFfiRepr(), another_arg.intoFfiRepr(), arg3)
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim(
            r#"
#include <stdint.h>
typedef struct __swift_bridge__$SomeStruct { uint8_t _private; } __swift_bridge__$SomeStruct;
typedef struct __swift_bridge__$AnotherStruct { uint8_t _0; } __swift_bridge__$AnotherStruct;
void __swift_bridge__$some_function(struct __swift_bridge__$SomeStruct some_arg, struct __swift_bridge__$AnotherStruct another_arg, uint8_t arg3);
"#,
        )
    }

    #[test]
    fn function_args_into_attribute() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
