//! Support for calling async Swift functions from Rust.
//!
//! This module provides the runtime support for async Swift function calls.
//! When Rust calls an async Swift function, it creates a oneshot channel,
//! passes the sender to Swift as an opaque pointer along with a callback,
//! and awaits the receiver. Swift executes the async function in a Task
//! and calls the callback when complete, which sends the result through
//! the channel.

use tokio::sync::oneshot;

/// Wrapper holding the oneshot sender, passed to Swift as an opaque pointer.
///
/// This struct is generic over the result type T, which allows it to work
/// with any return type from async Swift functions.
pub struct SwiftAsyncCallback<T> {
    sender: Option<oneshot::Sender<T>>,
}

/// Create a oneshot channel for an async Swift callback.
///
/// Returns a tuple of:
/// - A future that resolves when Swift calls the callback
/// - A raw pointer to the callback wrapper that should be passed to Swift
///
/// The wrapper pointer must be passed to `complete_swift_async` exactly once
/// to avoid memory leaks.
///
/// # Example (generated code pattern)
///
/// ```ignore
/// pub async fn fetch_data() -> String {
///     extern "C" fn callback(
///         wrapper: *mut std::ffi::c_void,
///         result: *mut swift_bridge::string::RustString
///     ) {
///         unsafe {
///             swift_bridge::async_swift_support::complete_swift_async(wrapper, result);
///         }
///     }
///
///     let (future, wrapper_ptr) = swift_bridge::async_swift_support::create_swift_async_call();
///     unsafe { __swift_bridge__fetch_data(wrapper_ptr, callback) };
///
///     let ffi_result = future.await;
///     // Convert FFI result to Rust type...
/// }
/// ```
pub fn create_swift_async_call<T: Send + 'static>(
) -> (impl std::future::Future<Output = T>, *mut std::ffi::c_void) {
    let (tx, rx) = oneshot::channel::<T>();
    let wrapper = Box::new(SwiftAsyncCallback { sender: Some(tx) });
    let ptr = Box::into_raw(wrapper) as *mut std::ffi::c_void;

    let future = async move {
        rx.await
            .expect("Swift async callback was dropped without sending a result")
    };

    (future, ptr)
}

/// Complete an async Swift call by sending the result back to Rust.
///
/// This function is called from the generated callback that Swift invokes
/// when the async operation completes.
///
/// # Safety
///
/// - `wrapper_ptr` must be a valid pointer obtained from `create_swift_async_call`
/// - This function must be called exactly once per `create_swift_async_call` call
/// - The type parameter T must match the type used in `create_swift_async_call`
pub unsafe fn complete_swift_async<T>(wrapper_ptr: *mut std::ffi::c_void, result: T) {
    let mut wrapper = Box::from_raw(wrapper_ptr as *mut SwiftAsyncCallback<T>);
    if let Some(sender) = wrapper.sender.take() {
        // Ignore send errors - the receiver may have been dropped if the
        // Rust future was cancelled
        let _ = sender.send(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_complete_async_call() {
        let (future, wrapper_ptr) = create_swift_async_call::<u32>();

        // Simulate Swift calling back
        unsafe {
            complete_swift_async(wrapper_ptr, 42u32);
        }

        let result = future.await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_async_call_with_string() {
        let (future, wrapper_ptr) = create_swift_async_call::<String>();

        unsafe {
            complete_swift_async(wrapper_ptr, "hello".to_string());
        }

        let result = future.await;
        assert_eq!(result, "hello");
    }

    #[tokio::test]
    async fn test_async_call_with_result_ok() {
        let (future, wrapper_ptr) = create_swift_async_call::<Result<u32, String>>();

        unsafe {
            complete_swift_async(wrapper_ptr, Ok::<u32, String>(123));
        }

        let result = future.await;
        assert_eq!(result, Ok(123));
    }

    #[tokio::test]
    async fn test_async_call_with_result_err() {
        let (future, wrapper_ptr) = create_swift_async_call::<Result<u32, String>>();

        unsafe {
            complete_swift_async(wrapper_ptr, Err::<u32, String>("error".to_string()));
        }

        let result = future.await;
        assert_eq!(result, Err("error".to_string()));
    }

    #[tokio::test]
    async fn test_async_call_with_unit() {
        let (future, wrapper_ptr) = create_swift_async_call::<()>();

        unsafe {
            complete_swift_async(wrapper_ptr, ());
        }

        future.await;
        // Success if we reach here without panic
    }
}
