#[swift_bridge::bridge]
mod ffi {
    struct AsyncRustFnReturnStruct;

    extern "Rust" {
        async fn rust_async_return_null();
        async fn rust_async_reflect_u8(arg: u8) -> u8;
        async fn rust_async_return_struct() -> AsyncRustFnReturnStruct;

    }

    extern "Rust" {
        type TestRustAsyncSelf;

        #[swift_bridg(init)]
        fn new() -> TestRustAsyncSelf;
        async fn reflect_u16(&self, arg: u16) -> u16;
    }
}

async fn rust_async_return_null() {}

async fn rust_async_reflect_u8(arg: u8) -> u8 {
    arg
}

async fn rust_async_return_struct() -> ffi::AsyncRustFnReturnStruct {
    ffi::AsyncRustFnReturnStruct
}

pub struct TestRustAsyncSelf;

impl TestRustAsyncSelf {
    fn new() -> Self {
        TestRustAsyncSelf
    }

    async fn reflect_u16(&self, arg: u16) -> u16 {
        arg
    }
}
