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

/// Test code generation for Rust function that accepts a Result<T, E> where T and E are
/// opaque Rust types.
mod extern_rust_fn_arg_result_opaque_rust {
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
    fn extern_rust_fn_arg_result_opaque_rust() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that accepts a Result<T, E> where T and E are
/// opaque Rust types.
mod extern_rust_fn_return_result_opaque_rust {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type SomeType;

                    fn some_function () -> Result<SomeType, SomeType>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> swift_bridge::result::ResultPtrAndPtr {
                match super::some_function() {
                    Ok(ok) => {
                        swift_bridge::result::ResultPtrAndPtr {
                            is_ok: true,
                            ok_or_err: Box::into_raw(Box::new({
                                let val: super::SomeType = ok;
                                val
                            })) as *mut super::SomeType as *mut std::ffi::c_void
                        }
                    }
                    Err(err) => {
                        swift_bridge::result::ResultPtrAndPtr {
                            is_ok: false,
                            ok_or_err: Box::into_raw(Box::new({
                                let val: super::SomeType = err;
                                val
                            })) as *mut super::SomeType as *mut std::ffi::c_void
                        }
                    }
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> SomeType {
    try { let val = __swift_bridge__$some_function(); if val.is_ok { return SomeType(ptr: val.ok_or_err!) } else { throw SomeType(ptr: val.ok_or_err!) } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
struct __private__ResultPtrAndPtr __swift_bridge__$some_function(void);
    "#,
    );

    #[test]
    fn extern_rust_fn_return_result_opaque_rust() {
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
/// where T and E are opaque Swift types.
mod extern_rust_fn_result_opaque_swift {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Swift" {
                    type SomeType;
                }

                extern "Rust" {
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
                        std::result::Result::Ok(unsafe { SomeType(arg.ok_or_err) })
                    } else {
                        std::result::Result::Err(unsafe { SomeType(arg.ok_or_err) })
                    }
                )
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: RustResult<SomeType, SomeType>) {
    __swift_bridge__$some_function({ switch arg { case .Ok(let ok): return __private__ResultPtrAndPtr(is_ok: true, ok_or_err: Unmanaged.passRetained(ok).toOpaque()) case .Err(let err): return __private__ResultPtrAndPtr(is_ok: false, ok_or_err: Unmanaged.passRetained(err).toOpaque()) } }())
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
    fn extern_rust_fn_result_opaque_swift() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that accepts a Result<(), E> where E is an
/// opaque Rust type.
mod extern_rust_fn_return_result_null_and_opaque_rust {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type SomeType;

                    fn some_function () -> Result<(), SomeType>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> *mut super::SomeType {
                match super::some_function() {
                    Ok(ok) => std::ptr::null_mut(),
                    Err(err) => Box::into_raw(Box::new({
                        let val: super::SomeType = err;
                        val
                    })) as *mut super::SomeType
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> () {
    try { let val = __swift_bridge__$some_function(); if val != nil { throw SomeType(ptr: val!) } else { return } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
void* __swift_bridge__$some_function(void);
    "#,
    );

    #[test]
    fn extern_rust_fn_return_result_opaque_rust() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

// Test code generation for Rust function that accepts a Result<T, E> where T is a UnitStruct and E is an
/// opaque Rust type.
mod extern_rust_fn_return_result_unit_and_opaque_rust {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                struct UnitType;
                extern "Rust" {
                    type SomeType;

                    fn some_function () -> Result<UnitType, SomeType>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> *mut super::SomeType {
                match super::some_function() {
                    Ok(ok) => std::ptr::null_mut(),
                    Err(err) => Box::into_raw(Box::new({
                        let val: super::SomeType = err;
                        val
                    })) as *mut super::SomeType
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> UnitType {
    try { let val = __swift_bridge__$some_function(); if val != nil { throw SomeType(ptr: val!) } else { return UnitType() } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
void* __swift_bridge__$some_function(void);
    "#,
    );

    #[test]
    fn extern_rust_fn_return_result_unit_and_opaque_rust() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that returns a Result<T, E> where T is a opaque Rust type and
/// E is a transparent enum type.
mod extern_rust_fn_return_result_opaque_rust_type_and_transparent_enum_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type SomeOkType;
                }
                enum SomeErrEnum {
                    Variant1,
                    Variant2(i32),
                }
                extern "Rust" {
                    fn some_function() -> Result<SomeOkType, SomeErrEnum>;
                }
            }
        }
    }

    // In Rust 1.79.0 dead_code warnings are issued for wrapped data in enums in spite of the enum
    // having `#[repr(C)]`. `#[allow(unused)]` can be removed following resolution and release of this
    // issue: https://github.com/rust-lang/rust/issues/126706
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[repr(C)]
            pub enum ResultSomeOkTypeAndSomeErrEnum{
                #[allow(unused)]
                Ok(*mut super::SomeOkType),
                #[allow(unused)]
                Err(__swift_bridge__SomeErrEnum),
            }


            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> ResultSomeOkTypeAndSomeErrEnum{
                match super::some_function() {
                    Ok(ok) => ResultSomeOkTypeAndSomeErrEnum::Ok(Box::into_raw(Box::new({
                        let val: super::SomeOkType = ok;
                        val
                    })) as *mut super::SomeOkType),
                    Err(err) => ResultSomeOkTypeAndSomeErrEnum::Err(err.into_ffi_repr()),
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> SomeOkType {
    try { let val = __swift_bridge__$some_function(); switch val.tag { case __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$ResultOk: return SomeOkType(ptr: val.payload.ok) case __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$ResultErr: throw val.payload.err.intoSwiftRepr() default: fatalError() } }()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef enum __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$Tag {__swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$ResultOk, __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$ResultErr} __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$Tag;
union __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$Fields {void* ok; struct __swift_bridge__$SomeErrEnum err;};
typedef struct __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum{__swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$Tag tag; union __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum$Fields payload;} __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum;        
"#,
            r#"struct __swift_bridge__$ResultSomeOkTypeAndSomeErrEnum __swift_bridge__$some_function(void)"#,
        ])
    }

    #[test]
    fn extern_rust_result_transparent_enum_type_and_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that returns a Result<T, E> where T is a transparent enum type and
/// E is a opaque Rust type.
mod extern_rust_fn_return_result_transparent_enum_type_and_opaque_rust_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                enum SomeOkEnum {
                    Variant1,
                    Variant2(i32),
                }
                extern "Rust" {
                    type SomeErrType;
                }
                extern "Rust" {
                    fn some_function() -> Result<SomeOkEnum, SomeErrType>;
                }
            }
        }
    }

    // In Rust 1.79.0 dead_code warnings are issued for wrapped data in enums in spite of the enum
    // having `#[repr(C)]`. `#[allow(unused)]` can be removed following resolution and release of this
    // issue: https://github.com/rust-lang/rust/issues/126706
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[repr(C)]
            pub enum ResultSomeOkEnumAndSomeErrType{
                #[allow(unused)]
                Ok(__swift_bridge__SomeOkEnum),
                #[allow(unused)]
                Err(*mut super::SomeErrType),
            }


            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> ResultSomeOkEnumAndSomeErrType{
                match super::some_function() {
                    Ok(ok) => ResultSomeOkEnumAndSomeErrType::Ok(ok.into_ffi_repr()),
                    Err(err) => ResultSomeOkEnumAndSomeErrType::Err(Box::into_raw(Box::new({
                        let val: super::SomeErrType = err;
                        val
                    })) as *mut super::SomeErrType),
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> SomeOkEnum {
    try { let val = __swift_bridge__$some_function(); switch val.tag { case __swift_bridge__$ResultSomeOkEnumAndSomeErrType$ResultOk: return val.payload.ok.intoSwiftRepr() case __swift_bridge__$ResultSomeOkEnumAndSomeErrType$ResultErr: throw SomeErrType(ptr: val.payload.err) default: fatalError() } }()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef enum __swift_bridge__$ResultSomeOkEnumAndSomeErrType$Tag {__swift_bridge__$ResultSomeOkEnumAndSomeErrType$ResultOk, __swift_bridge__$ResultSomeOkEnumAndSomeErrType$ResultErr} __swift_bridge__$ResultSomeOkEnumAndSomeErrType$Tag;
union __swift_bridge__$ResultSomeOkEnumAndSomeErrType$Fields {struct __swift_bridge__$SomeOkEnum ok; void* err;};
typedef struct __swift_bridge__$ResultSomeOkEnumAndSomeErrType{__swift_bridge__$ResultSomeOkEnumAndSomeErrType$Tag tag; union __swift_bridge__$ResultSomeOkEnumAndSomeErrType$Fields payload;} __swift_bridge__$ResultSomeOkEnumAndSomeErrType;        
"#,
            r#"struct __swift_bridge__$ResultSomeOkEnumAndSomeErrType __swift_bridge__$some_function(void)"#,
        ])
    }

    #[test]
    fn extern_rust_result_transparent_enum_type_and_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that returns a Result<T, E> where T is () and
/// E is a transparent enum type.
mod extern_rust_fn_return_result_unit_type_and_transparent_enum_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                enum SomeErrEnum {
                    Variant1,
                    Variant2(i32),
                }
                extern "Rust" {
                    fn some_function() -> Result<(), SomeErrEnum>;
                }
            }
        }
    }

    // In Rust 1.79.0 dead_code warnings are issued for wrapped data in enums in spite of the enum
    // having `#[repr(C)]`. `#[allow(unused)]` can be removed following resolution and release of this
    // issue: https://github.com/rust-lang/rust/issues/126706
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[repr(C)]
            pub enum ResultVoidAndSomeErrEnum{
                #[allow(unused)]
                Ok,
                #[allow(unused)]
                Err(__swift_bridge__SomeErrEnum),
            }

            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> ResultVoidAndSomeErrEnum{
                match super::some_function() {
                    Ok(ok) => ResultVoidAndSomeErrEnum::Ok,
                    Err(err) => ResultVoidAndSomeErrEnum::Err(err.into_ffi_repr()),
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> () {
    try { let val = __swift_bridge__$some_function(); switch val.tag { case __swift_bridge__$ResultVoidAndSomeErrEnum$ResultOk: return case __swift_bridge__$ResultVoidAndSomeErrEnum$ResultErr: throw val.payload.err.intoSwiftRepr() default: fatalError() } }()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef enum __swift_bridge__$ResultVoidAndSomeErrEnum$Tag {__swift_bridge__$ResultVoidAndSomeErrEnum$ResultOk, __swift_bridge__$ResultVoidAndSomeErrEnum$ResultErr} __swift_bridge__$ResultVoidAndSomeErrEnum$Tag;
union __swift_bridge__$ResultVoidAndSomeErrEnum$Fields {struct __swift_bridge__$SomeErrEnum err;};
typedef struct __swift_bridge__$ResultVoidAndSomeErrEnum{__swift_bridge__$ResultVoidAndSomeErrEnum$Tag tag; union __swift_bridge__$ResultVoidAndSomeErrEnum$Fields payload;} __swift_bridge__$ResultVoidAndSomeErrEnum;        
"#,
            r#"struct __swift_bridge__$ResultVoidAndSomeErrEnum __swift_bridge__$some_function(void)"#,
        ])
    }

    #[test]
    fn extern_rust_result_transparent_enum_type_and_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that returns a Result<T, E> where T is a tuple type and
/// E is a transparent enum type.
mod extern_rust_fn_return_result_tuple_type_and_transparent_enum_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                enum SomeErrEnum {
                    Variant1,
                    Variant2,
                }
                extern "Rust" {
                    fn some_function() -> Result<(i32, u32), SomeErrEnum>;
                }
            }
        }
    }

    // In Rust 1.79.0 dead_code warnings are issued for wrapped data in enums in spite of the enum
    // having `#[repr(C)]`. `#[allow(unused)]` can be removed following resolution and release of this
    // issue: https://github.com/rust-lang/rust/issues/126706
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                #[repr(C)]
                pub enum ResultTupleI32U32AndSomeErrEnum{
                    #[allow(unused)]
                    Ok(__swift_bridge__tuple_I32U32),
                    #[allow(unused)]
                    Err(__swift_bridge__SomeErrEnum),
                }
            },
            quote! {
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn __swift_bridge__some_function() -> ResultTupleI32U32AndSomeErrEnum{
                    match super::some_function() {
                        Ok(ok) => ResultTupleI32U32AndSomeErrEnum::Ok({let val = ok; __swift_bridge__tuple_I32U32(val.0, val.1)}),
                        Err(err) => ResultTupleI32U32AndSomeErrEnum::Err(err.into_ffi_repr()),
                    }
                }
            },
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct __swift_bridge__tuple_I32U32(i32, u32);
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> (Int32, UInt32) {
    try { let val = __swift_bridge__$some_function(); switch val.tag { case __swift_bridge__$ResultTupleI32U32AndSomeErrEnum$ResultOk: return { let val = val.payload.ok; return (val._0, val._1); }() case __swift_bridge__$ResultTupleI32U32AndSomeErrEnum$ResultErr: throw val.payload.err.intoSwiftRepr() default: fatalError() } }()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef enum __swift_bridge__$ResultTupleI32U32AndSomeErrEnum$Tag {__swift_bridge__$ResultTupleI32U32AndSomeErrEnum$ResultOk, __swift_bridge__$ResultTupleI32U32AndSomeErrEnum$ResultErr} __swift_bridge__$ResultTupleI32U32AndSomeErrEnum$Tag;
union __swift_bridge__$ResultTupleI32U32AndSomeErrEnum$Fields {struct __swift_bridge__$tuple$I32U32 ok; struct __swift_bridge__$SomeErrEnum err;};
typedef struct __swift_bridge__$ResultTupleI32U32AndSomeErrEnum{__swift_bridge__$ResultTupleI32U32AndSomeErrEnum$Tag tag; union __swift_bridge__$ResultTupleI32U32AndSomeErrEnum$Fields payload;} __swift_bridge__$ResultTupleI32U32AndSomeErrEnum;        
"#,
            r#"struct __swift_bridge__$ResultTupleI32U32AndSomeErrEnum __swift_bridge__$some_function(void)"#,
            r#"typedef struct __swift_bridge__$tuple$I32U32 { int32_t _0; uint32_t _1; } __swift_bridge__$tuple$I32U32;"#,
        ])
    }

    #[test]
    fn extern_rust_fn_return_result_tuple_type_and_transparent_enum_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that returns a Result<T, E> where T is () and
/// E is a transparent struct type.
mod extern_rust_fn_return_result_unit_type_and_transparent_struct_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                struct SomeErrStruct {
                    inner: String,
                }
                extern "Rust" {
                    fn some_function() -> Result<(), SomeErrStruct>;
                }
            }
        }
    }

    // In Rust 1.79.0 dead_code warnings are issued for wrapped data in enums in spite of the enum
    // having `#[repr(C)]`. `#[allow(unused)]` can be removed following resolution and release of this
    // issue: https://github.com/rust-lang/rust/issues/126706
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[repr(C)]
            pub enum ResultVoidAndSomeErrStruct{
                #[allow(unused)]
                Ok,
                #[allow(unused)]
                Err(__swift_bridge__SomeErrStruct),
            }

            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> ResultVoidAndSomeErrStruct{
                match super::some_function() {
                    Ok(ok) => ResultVoidAndSomeErrStruct::Ok,
                    Err(err) => ResultVoidAndSomeErrStruct::Err(err.into_ffi_repr()),
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() throws -> () {
    try { let val = __swift_bridge__$some_function(); switch val.tag { case __swift_bridge__$ResultVoidAndSomeErrStruct$ResultOk: return case __swift_bridge__$ResultVoidAndSomeErrStruct$ResultErr: throw val.payload.err.intoSwiftRepr() default: fatalError() } }()
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef enum __swift_bridge__$ResultVoidAndSomeErrStruct$Tag {__swift_bridge__$ResultVoidAndSomeErrStruct$ResultOk, __swift_bridge__$ResultVoidAndSomeErrStruct$ResultErr} __swift_bridge__$ResultVoidAndSomeErrStruct$Tag;
union __swift_bridge__$ResultVoidAndSomeErrStruct$Fields {struct __swift_bridge__$SomeErrStruct err;};
typedef struct __swift_bridge__$ResultVoidAndSomeErrStruct{__swift_bridge__$ResultVoidAndSomeErrStruct$Tag tag; union __swift_bridge__$ResultVoidAndSomeErrStruct$Fields payload;} __swift_bridge__$ResultVoidAndSomeErrStruct;
"#,
            r#"struct __swift_bridge__$ResultVoidAndSomeErrStruct __swift_bridge__$some_function(void)"#,
        ])
    }

    #[test]
    fn extern_rust_result_transparent_struct_type_and_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
