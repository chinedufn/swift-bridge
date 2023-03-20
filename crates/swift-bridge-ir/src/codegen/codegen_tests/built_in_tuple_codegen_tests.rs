use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we can use a (primitive type, primitive type) as Rust function arg and return type.
mod extern_rust_tuple_primitives {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn some_function(arg: (i32, u8)) -> (i32, u8);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                pub extern "C" fn __swift_bridge__some_function (arg: __swift_bridge__tuple_I32U8) -> __swift_bridge__tuple_I32U8 {
                    { let val = super::some_function((arg.0, arg.1)); __swift_bridge__tuple_I32U8(val.0, val.1) }
                }
            },
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct __swift_bridge__tuple_I32U8(i32, u8);
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
public func some_function(_ arg: (Int32, UInt8)) -> (Int32, UInt8) {
    { let val = __swift_bridge__$some_function(__swift_bridge__$tuple$I32U8(_0: arg.0, _1: arg.1)); return (val._0, val._1); }()
}
"#,
        ])
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef struct __swift_bridge__$tuple$I32U8 { int32_t _0; uint8_t _1; } __swift_bridge__$tuple$I32U8;
"#,
            r#"
struct __swift_bridge__$tuple$I32U8 __swift_bridge__$some_function(struct __swift_bridge__$tuple$I32U8 arg);
"#,
        ])
    }

    #[test]
    fn extern_rust_tuple_primitives() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we can use a (String, primitive type) as Rust function arg and return type.
mod extern_rust_tuple_string_primitive {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn some_function(arg1: (String, u32)) -> (String, u32);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                pub extern "C" fn __swift_bridge__some_function (arg1: __swift_bridge__tuple_StringU32) -> __swift_bridge__tuple_StringU32 {
                    { let val = super::some_function((unsafe { Box::from_raw(arg1.0).0 }, arg1.1)); __swift_bridge__tuple_StringU32(swift_bridge::string::RustString(val.0).box_into_raw(), val.1) }
                }
            },
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct __swift_bridge__tuple_StringU32(*mut swift_bridge::string::RustString, u32);
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
public func some_function<GenericIntoRustString: IntoRustString>(_ arg1: (GenericIntoRustString, UInt32)) -> (RustString, UInt32) {
    { let val = __swift_bridge__$some_function(__swift_bridge__$tuple$StringU32(_0: { let rustString = arg1.0.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), _1: arg1.1)); return (RustString(ptr: val._0), val._1); }()
}
"#,
        ])
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef struct __swift_bridge__$tuple$StringU32 { void* _0; uint32_t _1; } __swift_bridge__$tuple$StringU32;
"#,
            r#"
struct __swift_bridge__$tuple$StringU32 __swift_bridge__$some_function(struct __swift_bridge__$tuple$StringU32 arg1);
"#,
        ])
    }

    #[test]
    fn extern_rust_tuple_string_primitive() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we can use a (OpaqueRustType, primitive type) as Rust function arg and return type.
mod extern_rust_tuple_opaque_rust_primitive {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type SomeType;
                    fn some_function(arg1: (SomeType, u32)) -> (SomeType, u32);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                pub extern "C" fn __swift_bridge__some_function (arg1: __swift_bridge__tuple_SomeTypeU32) -> __swift_bridge__tuple_SomeTypeU32 {
                    { let val = super::some_function((unsafe { * Box::from_raw(arg1.0) }, arg1.1));
                    __swift_bridge__tuple_SomeTypeU32(Box::into_raw(Box::new({
                        let val: super::SomeType = val.0;
                        val
                    })) as *mut super::SomeType, val.1) }
                }
            },
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct __swift_bridge__tuple_SomeTypeU32(*mut super::SomeType, u32);
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
public func some_function(_ arg1: (SomeType, UInt32)) -> (SomeType, UInt32) {
    { let val = __swift_bridge__$some_function(__swift_bridge__$tuple$SomeTypeU32(_0: {arg1.0.isOwned = false; return arg1.0.ptr;}(), _1: arg1.1)); return (SomeType(ptr: val._0), val._1); }()
}
"#,
        ])
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef struct __swift_bridge__$tuple$SomeTypeU32 { void* _0; uint32_t _1; } __swift_bridge__$tuple$SomeTypeU32;
"#,
            r#"
struct __swift_bridge__$tuple$SomeTypeU32 __swift_bridge__$some_function(struct __swift_bridge__$tuple$SomeTypeU32 arg1);
"#,
        ])
    }

    #[test]
    fn extern_rust_tuple_opaque_rust_primitive() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
