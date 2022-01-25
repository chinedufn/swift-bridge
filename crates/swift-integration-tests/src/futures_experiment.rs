#[no_mangle]
pub extern "C" fn async_rust_fn(
    callback_wrapper: *mut std::ffi::c_void,
    callback: extern "C" fn(*mut std::ffi::c_void, i32) -> (),
) {
    let callback_wrapper = swift_bridge::async_runtime::SwiftCallbackWrapper(callback_wrapper);
    let task = async move {
        let val = async_rust_fn_to_expose().await;

        let callback_wrapper = callback_wrapper;
        let callback_wrapper = callback_wrapper.0;
        (callback)(callback_wrapper, val)
    };
    swift_bridge::async_runtime::ASYNC_RUNTIME.spawn_task(Box::pin(task));
}

async fn async_rust_fn_to_expose() -> i32 {
    5
}
