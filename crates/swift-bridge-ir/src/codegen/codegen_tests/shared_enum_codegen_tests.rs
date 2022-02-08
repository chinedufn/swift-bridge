use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we generate the correct to_ffi_repr() and to_rust_repr() implementations for an
/// enum where none of the variants contain any data.
mod generates_struct_to_and_from_ffi_conversions_no_data {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum SomeEnum {
                    Variant1,
                    Variant2,
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            mod ffi {
                pub enum SomeEnum {
                    Variant1,
                    Variant2
                }

                #[repr(C)]
                #[doc(hidden)]
                pub enum __swift_bridge__SomeEnum {
                    Variant1,
                    Variant2
                }

                impl swift_bridge::SharedEnum for SomeEnum {
                    type FfiRepr = __swift_bridge__SomeEnum;
                }

                impl SomeEnum {
                    #[doc(hidden)]
                    #[inline(always)]
                    pub fn into_ffi_repr(self) -> __swift_bridge__SomeEnum {
                        match self {
                            SomeEnum::Variant1 => __swift_bridge__SomeEnum::Variant1,
                            SomeEnum::Variant2 => __swift_bridge__SomeEnum::Variant2
                        }
                    }
                }

                impl __swift_bridge__SomeEnum {
                    #[doc(hidden)]
                    #[inline(always)]
                    pub fn into_rust_repr(self) -> SomeEnum {
                        match self {
                            __swift_bridge__SomeEnum::Variant1 => SomeEnum::Variant1,
                            __swift_bridge__SomeEnum::Variant2 => SomeEnum::Variant2
                        }
                    }
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public enum SomeEnum {
    case Variant1
    case Variant2
}
extension SomeEnum {
    func intoFfiRepr() -> __swift_bridge__$SomeEnum {
        switch self {
            case SomeEnum.Variant1:
                return __swift_bridge__$SomeEnum(tag: __swift_bridge__$SomeEnum$Variant1)
            case SomeEnum.Variant2:
                return __swift_bridge__$SomeEnum(tag: __swift_bridge__$SomeEnum$Variant2)
        }
    }
}
extension __swift_bridge__$SomeEnum {
    func intoSwiftRepr() -> SomeEnum {
        switch self.tag {
            case __swift_bridge__$SomeEnum$Variant1:
                return SomeEnum.Variant1
            case __swift_bridge__$SomeEnum$Variant2:
                return SomeEnum.Variant2
            default:
                fatalError("Unreachable")
        }
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim(
            r#"
typedef enum __swift_bridge__$SomeEnumTag { __swift_bridge__$SomeEnum$Variant1, __swift_bridge__$SomeEnum$Variant2, } __swift_bridge__$SomeEnumTag;
typedef struct __swift_bridge__$SomeEnum { __swift_bridge__$SomeEnumTag tag; } __swift_bridge__$SomeEnum;
    "#,
        )
    }

    #[test]
    fn generates_struct_to_and_from_ffi_conversions_no_data() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the correct code for a function that has an enum as an argument and
/// returns an enum.
mod using_enum_in_extern_rust_fn {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum SomeEnum {
                    Variant1,
                    Variant2,
                }

                extern "Rust" {
                    fn some_function(arg: SomeEnum) -> SomeEnum;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub extern "C" fn __swift_bridge__some_function(arg: __swift_bridge__SomeEnum) -> __swift_bridge__SomeEnum {
                super::some_function(arg.into_rust_repr()).into_ffi_repr()
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: SomeEnum) -> SomeEnum {
    __swift_bridge__$some_function(arg.intoFfiRepr()).intoSwiftRepr()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
struct __swift_bridge__$SomeEnum __swift_bridge__$some_function(struct __swift_bridge__$SomeEnum arg);
    "#,
        )
    }

    #[test]
    fn using_enum_in_extern_rust_fn() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
