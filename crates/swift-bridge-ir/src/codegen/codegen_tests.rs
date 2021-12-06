//! Tests for codegen for different scenarios.
//!
//! Tests are grouped into modules, each of which takes a set of tokens and then tests that the
//! generated Rust, Swift and C code matches what we expect.
//!
//! This entire module is conditionally compiled with `#[cfg(test)]`, so there is no need to
//! conditionally compile it's submodules.
//!
//! We previously kept out Rust, Swift and C codegen tests in separate files, but then moved to
//! this approach to make it easier to reason about our codegen.
//!
//! There are a bunch of tests in generate_rust_tokens.rs generate_swift.rs and
//! generate_c_header.rs that were written before this module was created. They should be
//! re-organized into this module over time.

#![cfg(test)]

use crate::test_utils::{
    assert_tokens_contain, assert_tokens_eq, assert_trimmed_generated_contains_trimmed_expected,
    assert_trimmed_generated_equals_trimmed_expected, parse_ok,
};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

/// Test code generation for freestanding Swift function that takes an opaque Rust type argument.
mod extern_swift_freestanding_fn_with_owned_opaque_rust_type_arg {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    type MyType;
                }

                extern "Swift" {
                    fn some_function (arg: MyType);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub fn some_function (arg: super::MyType) {
                unsafe { __swift_bridge__some_function( Box::into_raw(Box::new(arg)) as *mut super::MyType ) }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$some_function"]
                fn __swift_bridge__some_function (arg: *mut super::MyType);
            }
        })
    }

    const EXPECTED_SWIFT: ExpectedSwiftCode = ExpectedSwiftCode::ContainsAfterTrim(
        r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function (_ arg: UnsafeMutableRawPointer) {
    some_function(arg: MyType(ptr: arg, isOwned: true))
}
"#,
    );

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
typedef struct MyType MyType;
"#,
    );

    #[test]
    fn extern_swift_freestanding_fn_with_owned_opaque_rust_type_arg() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

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
func __swift_bridge__some_function (_ arg: __private__PointerToSwiftType) {
    some_function(arg: Unmanaged<MyType>.fromOpaque(arg.ptr).takeRetainedValue())
}
"#,
    );

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(r#""#);

    #[test]
    fn extern_swift_freestanding_fn_with_owned_opaque_swift_type_arg() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT_CODE,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that takes an owned String argument.
mod extern_rust_fn_with_owned_string_argument {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    fn some_function (arg: String);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: *mut swift_bridge::string::RustString
            ) {
                super::some_function(unsafe { Box::from_raw(arg).0 })
            }
        })
    }

    const EXPECTED_SWIFT_CODE: ExpectedSwiftCode = ExpectedSwiftCode::ContainsAfterTrim(
        r#"
func some_function(_ arg: RustString) {
    __swift_bridge__$some_function({arg.isOwned = false; return arg.ptr}())
}
"#,
    );

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
void __swift_bridge__$some_function(void* arg);
    "#,
    );

    #[test]
    fn extern_rust_fn_with_owned_string_argument() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT_CODE,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that returns an owned String argument.
mod extern_rust_fn_returns_string_argument {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> String;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function() -> *mut swift_bridge::string::RustString {
                swift_bridge::string::RustString(super::some_function()).box_into_raw()
            }
        })
    }

    const EXPECTED_SWIFT_CODE: ExpectedSwiftCode = ExpectedSwiftCode::ContainsAfterTrim(
        r#"
func some_function() -> RustString {
    RustString(ptr: __swift_bridge__$some_function(), isOwned: true)
}
"#,
    );

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
void* __swift_bridge__$some_function(void);
    "#,
    );

    #[test]
    fn extern_rust_fn_returns_string_argument() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT_CODE,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that returns a Vec<T> where T is an opaque Rust type.
mod extern_rust_fn_return_vec_of_opaque_rust_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type MyRustType;

                    fn some_function () -> Vec<MyRustType>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            const _: () = {
                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$new"]
                pub extern "C" fn _new() -> *mut Vec<super::MyRustType> {
                    Box::into_raw(Box::new(Vec::new()))
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$drop"]
                pub extern "C" fn _drop(vec: *mut Vec<super::MyRustType>) {
                    let vec = unsafe { Box::from_raw(vec) };
                    drop(vec)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$len"]
                pub extern "C" fn _len(vec: *const Vec<super::MyRustType>) -> usize {
                    unsafe { &*vec }.len()
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$get"]
                pub extern "C" fn _get(vec: *const Vec<super::MyRustType>, index: usize) -> *const super::MyRustType {
                    let vec = unsafe { & *vec };
                    if let Some(val) = vec.get(index) {
                        swift_bridge::option::_set_option_return(true);
                        val as *const super::MyRustType
                    } else {
                        swift_bridge::option::_set_option_return(false);
                        std::ptr::null()
                    }
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$push"]
                pub extern "C" fn _push(vec: *mut Vec<super::MyRustType>, val: *mut super::MyRustType) {
                    unsafe { &mut *vec }.push(unsafe { *Box::from_raw(val) })
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$pop"]
                pub extern "C" fn _pop(vec: *mut Vec<super::MyRustType>) -> *mut super::MyRustType {
                    let vec = unsafe { &mut *vec };
                    if let Some(val) = vec.pop() {
                        swift_bridge::option::_set_option_return(true);
                        Box::into_raw(Box::new(val))
                    } else {
                        swift_bridge::option::_set_option_return(false);
                        std::ptr::null::<super::MyRustType>() as *mut super::MyRustType
                    }
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$as_ptr"]
                pub extern "C" fn _as_ptr(vec: *const Vec<super::MyRustType>) -> *const super::MyRustType {
                    unsafe { & *vec }.as_ptr()
                }
            };
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
func some_function() -> RustVec<MyRustType> {
    RustVec(ptr: __swift_bridge__$some_function(), isOwned: true)
}"#,
            //
            r#"
extension MyRustType: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_MyRustType$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_MyRustType$drop(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: MyRustType) {
        __swift_bridge__$Vec_MyRustType$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_MyRustType$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (MyRustType(ptr: pointer!, isOwned: true) as! Self)
        }
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_MyRustType$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return (MyRustType(ptr: pointer!, isOwned: false) as! Self)
        }
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_MyRustType$len(vecPtr)
    }
}
"#,
        ])
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
#include <stdint.h>
typedef struct MyRustType MyRustType;
void __swift_bridge__$MyRustType$_free(void* self);

void* __swift_bridge__$Vec_MyRustType$new(void);
void __swift_bridge__$Vec_MyRustType$drop(void* vec_ptr);
void __swift_bridge__$Vec_MyRustType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_MyRustType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_MyRustType$get(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_MyRustType$len(void* vec_ptr);
void* __swift_bridge__$Vec_MyRustType$as_ptr(void* vec_ptr);

void* __swift_bridge__$some_function(void);
"#,
    );

    #[test]
    fn extern_rust_fn_return_vec_of_opaque_rust_type() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

struct CodegenTest {
    bridge_module_tokens: TokenStream,
    // Gets turned into a Vec<String> and compared to a Vec<String> of the generated Rust tokens.
    expected_rust_tokens: ExpectedRustTokens,
    // Gets trimmed and compared to the generated Swift code.
    expected_swift_code: ExpectedSwiftCode,
    // Gets trimmed and compared to the generated C header.
    expected_c_header: ExpectedCHeader,
}

enum ExpectedRustTokens {
    /// The generated Rust token stream matches the provided stream.
    #[allow(unused)]
    Exact(TokenStream),
    /// The generated Rust tokens stream contains the provided stream.
    Contains(TokenStream),
}

enum ExpectedSwiftCode {
    #[allow(unused)]
    ExactAfterTrim(&'static str),
    ContainsAfterTrim(&'static str),
    ContainsManyAfterTrim(Vec<&'static str>),
}

enum ExpectedCHeader {
    ExactAfterTrim(&'static str),
    ContainsAfterTrim(&'static str),
}

impl CodegenTest {
    fn test(self) {
        let module = parse_ok(self.bridge_module_tokens);
        let generated_tokens = module.to_token_stream();

        match self.expected_rust_tokens {
            ExpectedRustTokens::Exact(expected_tokens) => {
                assert_tokens_eq(&generated_tokens, &expected_tokens);
            }
            ExpectedRustTokens::Contains(expected_contained_tokens) => {
                assert_tokens_contain(&generated_tokens, &expected_contained_tokens);
            }
        };

        match self.expected_swift_code {
            ExpectedSwiftCode::ExactAfterTrim(expected_swift) => {
                assert_trimmed_generated_equals_trimmed_expected(
                    &module.generate_swift(),
                    expected_swift,
                );
            }
            ExpectedSwiftCode::ContainsAfterTrim(expected_contained_swift) => {
                assert_trimmed_generated_contains_trimmed_expected(
                    &module.generate_swift(),
                    expected_contained_swift,
                );
            }
            ExpectedSwiftCode::ContainsManyAfterTrim(many) => {
                let generated = module.generate_swift();

                for expected_contained_swift in many {
                    assert_trimmed_generated_contains_trimmed_expected(
                        &generated,
                        expected_contained_swift,
                    );
                }
            }
        };

        match self.expected_c_header {
            ExpectedCHeader::ExactAfterTrim(expected) => {
                assert_trimmed_generated_equals_trimmed_expected(
                    &module.generate_c_header_inner(),
                    expected,
                );
            }
            ExpectedCHeader::ContainsAfterTrim(expected) => {
                assert_trimmed_generated_contains_trimmed_expected(
                    &module.generate_c_header_inner(),
                    expected,
                );
            }
        };
    }
}
