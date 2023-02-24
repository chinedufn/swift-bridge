#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct AsyncRustFnReturnStruct {
        field: u8,
    }

    extern "Rust" {
        async fn rust_async_return_null();
        async fn rust_async_reflect_u8(arg: u8) -> u8;
        async fn rust_async_reflect_string(string: String) -> String;
        async fn rust_async_return_struct() -> AsyncRustFnReturnStruct;
        async fn rust_async_func_reflect_result_opaque_rust(
            arg: Result<AsyncResultOpaqueRustType1, AsyncResultOpaqueRustType2>,
        ) -> Result<AsyncResultOpaqueRustType1, AsyncResultOpaqueRustType2>;
    }

    extern "Rust" {
        type TestRustAsyncSelf;

        #[swift_bridg(init)]
        fn new() -> TestRustAsyncSelf;
        async fn reflect_u16(&self, arg: u16) -> u16;
    }

    extern "Rust" {
        type AsyncResultOpaqueRustType1;

        #[swift_bridge(init)]
        fn new(val: u32) -> AsyncResultOpaqueRustType1;
        fn val(&self) -> u32;
    }

    extern "Rust" {
        type AsyncResultOpaqueRustType2;

        #[swift_bridge(init)]
        fn new(val: u32) -> AsyncResultOpaqueRustType2;
        fn val(&self) -> u32;
    }
}

async fn rust_async_return_null() {}

async fn rust_async_reflect_u8(arg: u8) -> u8 {
    arg
}

async fn rust_async_reflect_string(string: String) -> String {
    string
}

async fn rust_async_return_struct() -> ffi::AsyncRustFnReturnStruct {
    ffi::AsyncRustFnReturnStruct { field: 123 }
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

pub struct AsyncResultOpaqueRustType1(u32);

impl AsyncResultOpaqueRustType1 {
    fn new(val: u32) -> Self {
        Self(val)
    }

    fn val(&self) -> u32 {
        self.0
    }
}

pub struct AsyncResultOpaqueRustType2(u32);

impl AsyncResultOpaqueRustType2 {
    fn new(val: u32) -> Self {
        Self(val)
    }

    fn val(&self) -> u32 {
        self.0
    }
}

async fn rust_async_func_reflect_result_opaque_rust(
    arg: Result<AsyncResultOpaqueRustType1, AsyncResultOpaqueRustType2>,
) -> Result<AsyncResultOpaqueRustType1, AsyncResultOpaqueRustType2> {
    match arg {
        Ok(ok) => {
            assert_eq!(ok.0, 10);
            Ok(ok)
        }
        Err(err) => {
            assert_eq!(err.0, 100);
            Err(err)
        }
    }
}
