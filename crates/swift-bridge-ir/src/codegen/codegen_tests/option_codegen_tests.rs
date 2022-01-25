//! See also: crates/swift-integration-tests/src/option.rs

use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Test code generation for Rust function that accepts and returns an Option<T> where T is a
/// primitive.
mod extern_rust_fn_option_primitive {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function (arg: Option<u8>) -> Option<f32>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: swift_bridge::option::OptionU8
            ) -> swift_bridge::option::OptionF32 {
                if let Some(val) = super::some_function(
                    if arg.is_some {
                        Some(arg.val)
                    } else {
                        None
                    }
                ) {
                    swift_bridge::option::OptionF32 { val, is_some: true}
                } else {
                    swift_bridge::option::OptionF32 { val: 123.4, is_some: false}
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: Optional<UInt8>) -> Optional<Float> {
    { let val = __swift_bridge__$some_function({ let val = arg; return __private__OptionU8(val: val ?? 123, is_some: val != nil); }()); if val.is_some { return val.val } else { return nil } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
struct __private__OptionF32 __swift_bridge__$some_function(struct __private__OptionU8 arg);
    "#,
    );

    #[test]
    fn extern_rust_fn_return_option_string() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that accepts and returns Option<String>.
mod extern_rust_fn_option_string {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function (arg: Option<String>) -> Option<String>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: *mut swift_bridge::string::RustString
            ) -> *mut swift_bridge::string::RustString {
                if let Some(val) = super::some_function(
                    if arg.is_null() {
                        None
                    } else {
                        Some(unsafe { Box::from_raw(arg).0 })
                    }
                ) {
                    swift_bridge::string::RustString(val).box_into_raw()
                } else {
                    std::ptr::null::<swift_bridge::string::RustString>() as *mut swift_bridge::string::RustString
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function<GenericIntoRustString: IntoRustString>(_ arg: Optional<GenericIntoRustString>) -> Optional<RustString> {
    { let val = __swift_bridge__$some_function({ if let rustString = optionalStringIntoRustString(arg) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()); if val != nil { return RustString(ptr: val!) } else { return nil } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
void* __swift_bridge__$some_function(void* arg);
    "#,
    );

    #[test]
    fn extern_rust_fn_return_option_string() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that returns Option<&str>
mod extern_rust_fn_return_option_str {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function () -> Option<&str>;
                    fn another_function () -> Option<&'static str>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        let no_lifetime = quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> swift_bridge::string::RustStr {
                if let Some(val) = super::some_function() {
                    swift_bridge::string::RustStr::from_str(val)
                } else {
                    swift_bridge::string::RustStr { start: std::ptr::null::<u8>(), len: 0}
                }
            }
        };
        let lifetime = quote! {
            #[export_name = "__swift_bridge__$another_function"]
            pub extern "C" fn __swift_bridge__another_function() -> swift_bridge::string::RustStr {
                if let Some(val) = super::another_function() {
                    swift_bridge::string::RustStr::from_str(val)
                } else {
                    swift_bridge::string::RustStr { start: std::ptr::null::<u8>(), len: 0}
                }
            }
        };

        ExpectedRustTokens::ContainsMany(vec![no_lifetime, lifetime])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        let no_lifetime = r#"
func some_function() -> Optional<RustStr> {
    { let val = __swift_bridge__$some_function(); if val.start != nil { return val; } else { return nil; } }()
}
"#;
        let lifetime = r#"
func another_function() -> Optional<RustStr> {
    { let val = __swift_bridge__$another_function(); if val.start != nil { return val; } else { return nil; } }()
}
        "#;

        ExpectedSwiftCode::ContainsManyAfterTrim(vec![no_lifetime, lifetime])
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
struct RustStr __swift_bridge__$some_function(void);
struct RustStr __swift_bridge__$another_function(void);
    "#,
    );

    #[test]
    fn extern_rust_fn_return_option_str() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that takes an Option<&str> arg
mod extern_rust_fn_arg_option_str {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function (arg: Option<&str>);
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
                super::some_function(
                    if arg.start.is_null() {
                        None
                    } else {
                        Some(arg.to_str())
                    }
                )
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function<GenericToRustStr: ToRustStr>(_ arg: Optional<GenericToRustStr>) {
    optionalRustStrToRustStr(arg, { argAsRustStr in
        __swift_bridge__$some_function(argAsRustStr)
    })
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
void __swift_bridge__$some_function(struct RustStr arg);
    "#,
    );

    #[test]
    fn extern_rust_fn_return_option_str() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that returns an Option<OpaqueRustType>
mod extern_rust_fn_return_option_opaque_rust_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type SomeType;
                    fn some_function () -> Option<SomeType>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> *mut super::SomeType {
                if let Some(val) = super::some_function() {
                    Box::into_raw(Box::new(val))
                } else {
                    std::ptr::null_mut()
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function() -> Optional<SomeType> {
    { let val = __swift_bridge__$some_function(); if val != nil { return SomeType(ptr: val!) } else { return nil } }()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void* __swift_bridge__$some_function(void);
    "#,
        )
    }

    #[test]
    fn extern_rust_fn_return_option_str() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that takes an Option<OpaqueRustType> argument.
mod extern_rust_fn_with_option_opaque_rust_type_arg {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type SomeType;
                    fn some_function (arg: Option<SomeType>);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: *mut super::SomeType
            ) {
                super::some_function(
                    if arg.is_null() {
                        None
                    } else {
                        Some( unsafe { * Box::from_raw(arg) } )
                    }
                )
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: Optional<SomeType>) {
    __swift_bridge__$some_function({ if let val = arg { val.isOwned = false; return val.ptr } else { return nil } }())
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* arg);
    "#,
        )
    }

    #[test]
    fn extern_rust_fn_return_option_str() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
