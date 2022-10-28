use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we generate the correct to_ffi_repr() and to_rust_repr() implementations for an
/// enum where none of the variants contain any data.
mod generates_enum_to_and_from_ffi_conversions_no_data {
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
            #[derive(Copy, Clone)]
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
        ExpectedCHeader::ContainsAfterTrim(
            r#"
#include <stdbool.h>
typedef enum __swift_bridge__$SomeEnumTag { __swift_bridge__$SomeEnum$Variant1, __swift_bridge__$SomeEnum$Variant2, } __swift_bridge__$SomeEnumTag;
typedef struct __swift_bridge__$SomeEnum { __swift_bridge__$SomeEnumTag tag; } __swift_bridge__$SomeEnum;
typedef struct __swift_bridge__$Option$SomeEnum { bool is_some; __swift_bridge__$SomeEnum val; } __swift_bridge__$Option$SomeEnum;
"#,
        )
    }

    #[test]
    fn generates_enum_to_and_from_ffi_conversions_no_data() {
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

/// Verify that we can use `Option<Enum>` as Rust function arg and return type.
mod extern_rust_option_enum {
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
                    fn some_function(arg: Option<SomeEnum>) -> Option<SomeEnum>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct __swift_bridge__Option_SomeEnum {
                    is_some: bool,
                    val: std::mem::MaybeUninit<__swift_bridge__SomeEnum>,
                }

                impl __swift_bridge__Option_SomeEnum {
                    #[doc(hidden)]
                    #[inline(always)]
                    pub fn into_rust_repr(self) -> Option<SomeEnum> {
                        if self.is_some {
                            Some(unsafe { self.val.assume_init().into_rust_repr() })
                        } else {
                            None
                        }
                    }

                    #[doc(hidden)]
                    #[inline(always)]
                    pub fn from_rust_repr(val: Option<SomeEnum>) -> __swift_bridge__Option_SomeEnum {
                        if let Some(val) = val {
                            __swift_bridge__Option_SomeEnum {
                                is_some: true,
                                val: std::mem::MaybeUninit::new(val.into_ffi_repr())
                            }
                        } else {
                            __swift_bridge__Option_SomeEnum {
                                is_some: false,
                                val: std::mem::MaybeUninit::uninit()
                            }
                        }
                    }
                }
            },
            quote! {
                pub extern "C" fn __swift_bridge__some_function(arg: __swift_bridge__Option_SomeEnum) -> __swift_bridge__Option_SomeEnum {
                    __swift_bridge__Option_SomeEnum::from_rust_repr(super::some_function(arg.into_rust_repr()))
                }
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
extension __swift_bridge__$Option$SomeEnum {
    @inline(__always)
    func intoSwiftRepr() -> Optional<SomeEnum> {
        if self.is_some {
            return self.val.intoSwiftRepr()
        } else {
            return nil
        }
    }

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<SomeEnum>) -> __swift_bridge__$Option$SomeEnum {
        if let v = val {
            return __swift_bridge__$Option$SomeEnum(is_some: true, val: v.intoFfiRepr())
        } else {
            return __swift_bridge__$Option$SomeEnum(is_some: false, val: __swift_bridge__$SomeEnum())
        }
    }
}
"#,
            r#"
func some_function(_ arg: Optional<SomeEnum>) -> Optional<SomeEnum> {
    __swift_bridge__$some_function(__swift_bridge__$Option$SomeEnum.fromSwiftRepr(arg)).intoSwiftRepr()
}
"#,
        ])
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
#include <stdbool.h>
typedef enum __swift_bridge__$SomeEnumTag { __swift_bridge__$SomeEnum$Variant1, __swift_bridge__$SomeEnum$Variant2, } __swift_bridge__$SomeEnumTag;
typedef struct __swift_bridge__$SomeEnum { __swift_bridge__$SomeEnumTag tag; } __swift_bridge__$SomeEnum;
typedef struct __swift_bridge__$Option$SomeEnum { bool is_some; __swift_bridge__$SomeEnum val; } __swift_bridge__$Option$SomeEnum;
"#,
            r#"
struct __swift_bridge__$Option$SomeEnum __swift_bridge__$some_function(struct __swift_bridge__$Option$SomeEnum arg);
    "#,
        ])
    }

    #[test]
    fn option_enum() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
