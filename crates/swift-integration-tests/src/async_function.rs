#[swift_bridge::bridge]
mod ffi {
    struct AsyncRustFnReturnStruct;

    extern "Rust" {
        async fn rust_async_return_null();
        async fn rust_async_return_u8() -> u8;
        async fn rust_async_return_struct() -> AsyncRustFnReturnStruct;
    }
}

async fn rust_async_return_null() {}

async fn rust_async_return_u8() -> u8 {
    123
}

async fn rust_async_return_struct() -> ffi::AsyncRustFnReturnStruct {
    ffi::AsyncRustFnReturnStruct
}
