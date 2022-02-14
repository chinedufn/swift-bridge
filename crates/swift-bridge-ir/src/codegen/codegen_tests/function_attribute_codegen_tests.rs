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

/// Verify that we explicitly case to the struct  when generating `into_return_type` code for a
/// Rust function that returns a shared struct.
/// This explicit conversion into the struct avoids a type inference issues when converting the
/// shared struct to its FFI representation so that we can return it.
mod into_return_type_attribute_for_shared_struct {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                #[swift_bridge(swift_name = "StructRename1")]
                struct StructName1;

                extern "Rust" {
                    #[swift_bridge(into_return_type)]
                    fn some_function() -> StructName1;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub extern "C" fn __swift_bridge__some_function() -> __swift_bridge__StructName1 {
                 { let val: StructName1 = super::some_function().into(); val }.into_ffi_repr()
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::SkipTest
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::SkipTest
    }

    #[test]
    fn shared_struct_swift_name_attribute() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we can use `return_with` to convert a return type.
mod return_with {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(return_with = path::to::convert_fn)]
                    fn some_function() -> u32;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub extern "C" fn __swift_bridge__some_function() -> u32 {
                super::path::to::convert_fn(super::some_function())
            }
        })
    }

    #[test]
    fn return_with() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: ExpectedSwiftCode::SkipTest,
            expected_c_header: ExpectedCHeader::SkipTest,
        }
        .test();
    }
}

/// Verify that we can annotate that a function should serve as the Identifiable protocol extension.
mod protocol_identifiable {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type SomeType;
                    type AnotherType;

                    #[swift_bridge(Identifiable)]
                    fn some_function(self: &SomeType) -> i16;

                    #[swift_bridge(Identifiable)]
                    fn id(self: &AnotherType) -> u32;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::SkipTest
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        // We add the class declarations to our assertions to ensure that our extensions come
        // directory after our class declarations.
        // This helps when an end-user that is looking at the generated code more easily see
        // which protocols a class implements.
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
public class SomeTypeRef {
    var ptr: UnsafeMutableRawPointer

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension SomeTypeRef: Identifiable {
    public var id: Int16 {
        return self.some_function()
    }
}"#,
            r#"
public class AnotherTypeRef {
    var ptr: UnsafeMutableRawPointer

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AnotherTypeRef: Identifiable {}"#,
        ])
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::SkipTest
    }

    #[test]
    fn protocol_identifiable() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
