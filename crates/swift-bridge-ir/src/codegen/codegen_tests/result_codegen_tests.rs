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
            pub extern "C" fn __swift_bridge__some_function() -> swift_bridge::result::ResultVoidAndPtr {
                match super::some_function() {
                    Ok(ok) => {
                        swift_bridge::result::ResultVoidAndPtr {
                            is_ok: true,
                            err: std::ptr::null_mut::<std::ffi::c_void>()
                        }
                    }
                    Err(err) => {
                        swift_bridge::result::ResultVoidAndPtr {
                            is_ok: false,
                            err: Box::into_raw(Box::new({
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
public func some_function() throws -> () {
    try { let val = __swift_bridge__$some_function(); if val.is_ok { return  } else { throw SomeType(ptr: val.err!) } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
struct __private__ResultVoidAndPtr __swift_bridge__$some_function(void);
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
            pub extern "C" fn __swift_bridge__some_function() -> swift_bridge::result::ResultVoidAndPtr {
                match super::some_function() {
                    Ok(ok) => {
                        swift_bridge::result::ResultVoidAndPtr {
                            is_ok: true,
                            err: std::ptr::null_mut::<std::ffi::c_void>()
                        }
                    }
                    Err(err) => {
                        swift_bridge::result::ResultVoidAndPtr {
                            is_ok: false,
                            err: Box::into_raw(Box::new({
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
public func some_function() throws -> UnitType {
    try { let val = __swift_bridge__$some_function(); if val.is_ok { return UnitType() } else { throw SomeType(ptr: val.err!) } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
struct __private__ResultVoidAndPtr __swift_bridge__$some_function(void);
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
