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
                callback: extern "C" fn(*mut std::ffi::c_void) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                    fut.await;

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
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(()))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<(), Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<(), Never>) -> ()

    public init(cb: @escaping (Result<(), Never>) -> ()) {
        self.cb = cb
    }
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

/// Verify that we generate the correct code for extern "Rust" async functions that takes a u32 arg.
mod extern_rust_async_function_u32_arg {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    async fn some_function(arg: u32);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void) -> (),
                arg: u32
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function(arg);
                let task = async move {
                    fut.await;

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
func some_function(_ arg: UInt32) async {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(()))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<(), Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete, arg)
    })
}
class CbWrapper$some_function {
    var cb: (Result<(), Never>) -> ()

    public init(cb: @escaping (Result<(), Never>) -> ()) {
        self.cb = cb
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
#include <stdint.h>
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper), uint32_t arg);
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
                callback: extern "C" fn(*mut std::ffi::c_void, u8) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                    let val = fut.await;

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
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: UInt8) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(rustFnRetVal))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<UInt8, Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<UInt8, Never>) -> ()

    public init(cb: @escaping (Result<UInt8, Never>) -> ()) {
        self.cb = cb
    }
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

/// Verify that we generate the correct code for extern "Rust" async functions that returns a u8.
mod extern_rust_async_function_returns_string {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    async fn some_function() -> String;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void, *mut swift_bridge::string::RustString) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                    let val = swift_bridge::string::RustString(fut.await).box_into_raw();

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
func some_function() async -> RustString {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: UnsafeMutableRawPointer?) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(RustString(ptr: rustFnRetVal!)))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<RustString, Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<RustString, Never>) -> ()

    public init(cb: @escaping (Result<RustString, Never>) -> ()) {
        self.cb = cb
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper, void* ret));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function_returns_string() {
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
                struct SomeStruct {
                    field: u8
                }

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
                callback: extern "C" fn(*mut std::ffi::c_void, __swift_bridge__SomeStruct) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                    let val = fut.await.into_ffi_repr();

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
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$SomeStruct) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(rustFnRetVal.intoSwiftRepr()))
    }

    return await withCheckedContinuation({ (continuation: CheckedContinuation<SomeStruct, Never>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<SomeStruct, Never>) -> ()

    public init(cb: @escaping (Result<SomeStruct, Never>) -> ()) {
        self.cb = cb
    }
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

/// Verify that we generate the correct code for extern "Rust" async method.
mod extern_rust_async_method {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type SomeType;
                    async fn some_method(&self);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__SomeType_some_method(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void) -> (),
                this: *mut super::SomeType
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = (unsafe {&*this}).some_method();
                let task = async move {
                    fut.await;

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
extension SomeTypeRef {
    public func some_method() async {
        func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?) {
            let wrapper = Unmanaged<CbWrapper$SomeType$some_method>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
            wrapper.cb(.success(()))
        }

        return await withCheckedContinuation({ (continuation: CheckedContinuation<(), Never>) in
            let callback = { rustFnRetVal in
                continuation.resume(with: rustFnRetVal)
            }

            let wrapper = CbWrapper$SomeType$some_method(cb: callback)
            let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

            __swift_bridge__$SomeType$some_method(wrapperPtr, onComplete, ptr)
        })
    }
    class CbWrapper$SomeType$some_method {
        var cb: (Result<(), Never>) -> ()
    
        public init(cb: @escaping (Result<(), Never>) -> ()) {
            self.cb = cb
        }
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$SomeType$some_method(void* callback_wrapper, void __swift_bridge__$SomeType$some_method$async(void* callback_wrapper), void* self);
    "#,
        )
    }

    #[test]
    fn extern_rust_async_method() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the correct code for extern "Rust" async functions that returns a Result<OpaqueRustType, OpaqueRustType>.
mod extern_rust_async_function_returns_result_opaque {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type OkType;
                    type ErrorType;
                    async fn some_function() -> Result<OkType, ErrorType>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void, swift_bridge :: result :: ResultPtrAndPtr) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                let val = match fut.await {
                    Ok(ok) => {
                        swift_bridge::result::ResultPtrAndPtr {
                            is_ok: true,
                            ok_or_err: Box::into_raw(Box::new({
                                let val: super::OkType = ok;
                                val
                            })) as *mut super::OkType as *mut std::ffi::c_void
                        }
                    }
                    Err(err) => {
                        swift_bridge::result::ResultPtrAndPtr {
                            is_ok: false,
                            ok_or_err: Box::into_raw(Box::new({
                                let val: super::ErrorType = err;
                                val
                            })) as *mut super::ErrorType as *mut std::ffi::c_void
                        }
                    }
                };
                    let callback_wrapper = callback_wrapper;
                    let callback_wrapper = callback_wrapper.0;

                    (callback)(callback_wrapper, val)
                };
                swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
            }
        })
    }

    // TODO: Replace `Error` with the concrete error type `ErrorType`.
    // As of Feb 2023 using the concrete error type leads to a compile time error.
    // This seems like a bug in the Swift compiler.

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() async throws -> OkType {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __private__ResultPtrAndPtr) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        if rustFnRetVal.is_ok {
            wrapper.cb(.success(OkType(ptr: rustFnRetVal.ok_or_err!)))
        } else {
            wrapper.cb(.failure(ErrorType(ptr: rustFnRetVal.ok_or_err!)))
        }
    }

    return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<OkType, Error>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<OkType, Error>) -> ()

    public init(cb: @escaping (Result<OkType, Error>) -> ()) {
        self.cb = cb
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper, struct __private__ResultPtrAndPtr ret));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function_returns_result_opaque() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the correct code for extern "Rust" async functions that returns a Result<TransparentEnum, TransparentEnum>.
mod extern_rust_async_function_returns_result_transparent_enum {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum OkEnum {
                    OkVariant1,
                    OkVariant2,
                }
                enum ErrEnum {
                    ErrVariant1,
                    ErrVariant2,
                }
                extern "Rust" {
                    async fn some_function() -> Result<OkEnum, ErrEnum>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void, ResultOkEnumAndErrEnum) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                let val = match fut.await {
                    Ok(ok) => ResultOkEnumAndErrEnum::Ok(ok.into_ffi_repr()),
                    Err(err) => ResultOkEnumAndErrEnum::Err(err.into_ffi_repr()),
                };
                    let callback_wrapper = callback_wrapper;
                    let callback_wrapper = callback_wrapper.0;

                    (callback)(callback_wrapper, val)
                };
                swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
            }
        })
    }

    // TODO: Replace `Error` with the concrete error type `ErrorType`.
    // As of Feb 2023 using the concrete error type leads to a compile time error.
    // This seems like a bug in the Swift compiler.

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() async throws -> OkEnum {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultOkEnumAndErrEnum) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        switch rustFnRetVal.tag { case __swift_bridge__$ResultOkEnumAndErrEnum$ResultOk: wrapper.cb(.success(rustFnRetVal.payload.ok.intoSwiftRepr())) case __swift_bridge__$ResultOkEnumAndErrEnum$ResultErr: wrapper.cb(.failure(rustFnRetVal.payload.err.intoSwiftRepr())) default: fatalError() }
    }

    return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<OkEnum, Error>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<OkEnum, Error>) -> ()

    public init(cb: @escaping (Result<OkEnum, Error>) -> ()) {
        self.cb = cb
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper, struct __swift_bridge__$ResultOkEnumAndErrEnum ret));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function_returns_result_transparent_enum() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the correct code for extern "Rust" async functions that returns a Result<OpaqueRustType, TransparentEnum>.
mod extern_rust_async_function_returns_result_opaque_rust_transparent_enum {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum ErrEnum {
                    ErrVariant1,
                    ErrVariant2,
                }
                extern "Rust" {
                    type SomeType;
                    async fn some_function() -> Result<SomeType, ErrEnum>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void, ResultSomeTypeAndErrEnum) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                let val = match fut.await {
                    Ok(ok) => ResultSomeTypeAndErrEnum::Ok(Box::into_raw(Box::new({
                        let val: super::SomeType = ok;
                        val
                    })) as *mut super::SomeType),
                    Err(err) => ResultSomeTypeAndErrEnum::Err(err.into_ffi_repr()),
                };
                    let callback_wrapper = callback_wrapper;
                    let callback_wrapper = callback_wrapper.0;

                    (callback)(callback_wrapper, val)
                };
                swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
            }
        })
    }

    // TODO: Replace `Error` with the concrete error type `ErrorType`.
    // As of Feb 2023 using the concrete error type leads to a compile time error.
    // This seems like a bug in the Swift compiler.

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() async throws -> SomeType {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultSomeTypeAndErrEnum) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        switch rustFnRetVal.tag { case __swift_bridge__$ResultSomeTypeAndErrEnum$ResultOk: wrapper.cb(.success(SomeType(ptr: rustFnRetVal.payload.ok))) case __swift_bridge__$ResultSomeTypeAndErrEnum$ResultErr: wrapper.cb(.failure(rustFnRetVal.payload.err.intoSwiftRepr())) default: fatalError() }
    }

    return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<SomeType, Error>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<SomeType, Error>) -> ()

    public init(cb: @escaping (Result<SomeType, Error>) -> ()) {
        self.cb = cb
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper, struct __swift_bridge__$ResultSomeTypeAndErrEnum ret));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function_returns_result_opaque_rust_transparent_enum() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we generate the correct code for extern "Rust" async functions that returns a Result<(), TransparentEnum>.
mod extern_rust_async_function_returns_result_null_transparent_enum {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum ErrEnum {
                    ErrVariant1,
                    ErrVariant2,
                }
                extern "Rust" {
                    async fn some_function() -> Result<(), ErrEnum>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
             pub extern "C" fn __swift_bridge__some_function(
                callback_wrapper: *mut std::ffi::c_void,
                callback: extern "C" fn(*mut std::ffi::c_void, ResultVoidAndErrEnum) -> (),
            ) {
                let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                let fut = super::some_function();
                let task = async move {
                let val = match fut.await {
                    Ok(ok) => ResultVoidAndErrEnum::Ok,
                    Err(err) => ResultVoidAndErrEnum::Err(err.into_ffi_repr()),
                };
                    let callback_wrapper = callback_wrapper;
                    let callback_wrapper = callback_wrapper.0;

                    (callback)(callback_wrapper, val)
                };
                swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
            }
        })
    }

    // TODO: Replace `Error` with the concrete error type `ErrorType`.
    // As of Feb 2023 using the concrete error type leads to a compile time error.
    // This seems like a bug in the Swift compiler.

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public func some_function() async throws -> () {
    func onComplete(cbWrapperPtr: UnsafeMutableRawPointer?, rustFnRetVal: __swift_bridge__$ResultVoidAndErrEnum) {
        let wrapper = Unmanaged<CbWrapper$some_function>.fromOpaque(cbWrapperPtr!).takeRetainedValue()
        switch rustFnRetVal.tag { case __swift_bridge__$ResultVoidAndErrEnum$ResultOk: wrapper.cb(.success(())) case __swift_bridge__$ResultVoidAndErrEnum$ResultErr: wrapper.cb(.failure(rustFnRetVal.payload.err.intoSwiftRepr())) default: fatalError() }
    }

    return try await withCheckedThrowingContinuation({ (continuation: CheckedContinuation<(), Error>) in
        let callback = { rustFnRetVal in
            continuation.resume(with: rustFnRetVal)
        }

        let wrapper = CbWrapper$some_function(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()

        __swift_bridge__$some_function(wrapperPtr, onComplete)
    })
}
class CbWrapper$some_function {
    var cb: (Result<(), Error>) -> ()

    public init(cb: @escaping (Result<(), Error>) -> ()) {
        self.cb = cb
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* callback_wrapper, void __swift_bridge__$some_function$async(void* callback_wrapper, struct __swift_bridge__$ResultVoidAndErrEnum ret));
    "#,
        )
    }

    #[test]
    fn extern_rust_async_function_returns_result_null_transparent_enum() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
