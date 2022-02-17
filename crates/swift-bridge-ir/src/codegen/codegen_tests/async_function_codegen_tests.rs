use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we generate the correct code for extern "Rust" async functions that do not return
/// a value.
mod extern_rust_async_function_no_return {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    async fn some_function();
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void) -> ()
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let task = async move {
                    super::some_function().await;

                    let callback_wrapper = callback_wrapper;
                    let callback_wrapper = callback_wrapper.0;

                    (callback)(callback_wrapper)
                };
                swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function() async {
    class CbWrapper {
        var cb: (Result<(), Never>) -> ()

        init(cb: @escaping (Result<(), Never>) -> ()) {
            self.cb = cb
        }
    }

    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?) {
        let wrapper = Unmanaged<CbWrapper>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(()))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<(), Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the correct code for extern "Rust" async functions that returns a u8.
mod extern_rust_async_function_returns_u8 {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    async fn some_function() -> u8;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void, u8) -> ()
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let task = async move {
                    let val = super::some_function().await;

                    let callback_wrapper = callback_wrapper;
                    let callback_wrapper = callback_wrapper.0;

                    (callback)(callback_wrapper, val)
                };
                swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function() async -> UInt8 {
    class CbWrapper {
        var cb: (Result<UInt8, Never>) -> ()

        init(cb: @escaping (Result<UInt8, Never>) -> ()) {
            self.cb = cb
        }
    }

    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: UInt8) {
        let wrapper = Unmanaged<CbWrapper>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(rustFnRetVal))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<UInt8, Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
#include <stdint.h>
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper, uint8_t ret));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function_returns_u8() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the correct code for extern "Rust" async functions that returns a struct.
mod extern_rust_async_function_returns_struct {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                struct SomeStruct;

                extern "Rust" {
                    async fn some_function() -> SomeStruct;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void, __swift_bridge__SomeStruct) -> ()
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let task = async move {
                    let val = super::some_function().await.into_ffi_repr();

                    let callback_wrapper = callback_wrapper;
                    let callback_wrapper = callback_wrapper.0;

                    (callback)(callback_wrapper, val)
                };
                swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function() async -> SomeStruct {
    class CbWrapper {
        var cb: (Result<SomeStruct, Never>) -> ()

        init(cb: @escaping (Result<SomeStruct, Never>) -> ()) {
            self.cb = cb
        }
    }

    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$SomeStruct) {
        let wrapper = Unmanaged<CbWrapper>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(rustFnRetVal.intoSwiftRepr()))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<SomeStruct, Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper, struct __swift_bridge__$SomeStruct ret));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function_returns_struct() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
