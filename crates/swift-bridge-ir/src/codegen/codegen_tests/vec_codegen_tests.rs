//! See also: crates/swift-integration-tests/src/vec.rs

use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we emit Rust, Swift and C header code that allows an extern "Rust" type be used
/// within a Vec<T>.
mod extern_rust_type_vec_support {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type MyRustType;
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
                        val as *const super::MyRustType
                    } else {
                        std::ptr::null()
                    }
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_MyRustType$get_mut"]
                pub extern "C" fn _get_mut(vec: *mut Vec<super::MyRustType>, index: usize) -> *mut super::MyRustType {
                    let vec = unsafe { &mut *vec };
                    if let Some(val) = vec.get_mut(index) {
                        val as *mut super::MyRustType
                    } else {
                        std::ptr::null::<super::MyRustType>() as *mut super::MyRustType
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
                        Box::into_raw(Box::new(val))
                    } else {
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
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
extension MyRustType: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_MyRustType$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_MyRustType$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: MyRustType) {
        __swift_bridge__$Vec_MyRustType$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_MyRustType$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (MyRustType(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MyRustTypeRef> {
        let pointer = __swift_bridge__$Vec_MyRustType$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return MyRustTypeRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MyRustTypeRefMut> {
        let pointer = __swift_bridge__$Vec_MyRustType$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return MyRustTypeRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_MyRustType$len(vecPtr)
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ExactAfterTrim(
            r#"
typedef struct MyRustType MyRustType;
void __swift_bridge__$MyRustType$_free(void* self);

void* __swift_bridge__$Vec_MyRustType$new(void);
void __swift_bridge__$Vec_MyRustType$drop(void* vec_ptr);
void __swift_bridge__$Vec_MyRustType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_MyRustType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_MyRustType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_MyRustType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_MyRustType$len(void* vec_ptr);
void* __swift_bridge__$Vec_MyRustType$as_ptr(void* vec_ptr);
"#,
        )
    }

    #[test]
    fn extern_rust_fn_return_vec_of_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
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
                    fn some_function() -> Vec<MyRustType>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub extern "C" fn __swift_bridge__some_function() -> *mut Vec<super::MyRustType> {
                Box::into_raw(Box::new(super::some_function()))
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function() -> RustVec<MyRustType> {
    RustVec(ptr: __swift_bridge__$some_function())
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
    fn extern_rust_fn_return_vec_of_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for Rust function that has an argument
/// Vec<T> where T is an opaque Rust type.
mod extern_rust_fn_arg_vec_of_opaque_rust_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type MyRustType;
                    fn some_function(arg: Vec<MyRustType>);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub extern "C" fn __swift_bridge__some_function(
                arg: *mut Vec<super::MyRustType>
            ) {
                super::some_function(unsafe { * Box::from_raw(arg) })
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: RustVec<MyRustType>) {
    __swift_bridge__$some_function({ let val = arg; val.isOwned = false; return val.ptr }())
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
    fn extern_rust_fn_return_vec_of_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
