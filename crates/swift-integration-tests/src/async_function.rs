// This is a temporary workaround until https://github.com/chinedufn/swift-bridge/issues/270
// is closed. When tests are compiled they have `-D warnings` (deny warnings) enabled, so
// tests won't even compile unless this warning is ignored.
#![allow(dead_code)]

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct AsyncRustFnReturnStruct {
        field: u8,
    }

    extern "Rust" {
        async fn rust_async_return_null();
        async fn rust_async_reflect_u8(arg: u8) -> u8;

        async fn rust_async_return_struct() -> AsyncRustFnReturnStruct;
        async fn rust_async_func_reflect_result_opaque_rust(
            arg: Result<AsyncResultOpaqueRustType1, AsyncResultOpaqueRustType2>,
        ) -> Result<AsyncResultOpaqueRustType1, AsyncResultOpaqueRustType2>;
        async fn rust_async_func_return_result_null_opaque_rust(
            succeed: bool,
        ) -> Result<(), AsyncResultOpaqueRustType2>;

        // TODO: this is broken because RustString is not Sendable.
        // Work around making String and other opaque types Sendable is tracked
        // here: https://github.com/chinedufn/swift-bridge/issues/150
        // async fn rust_async_reflect_string(string: String) -> String;
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

    enum AsyncResultOkEnum {
        NoFields,
        UnnamedFields(i32, u32),
        NamedFields { value: u8 },
    }

    enum AsyncResultErrEnum {
        UnnamedFields(String, i32),
        NamedFields { value: u32 },
    }

    extern "Rust" {
        async fn rust_async_func_return_result_transparent_enum_and_transparent_enum(
            succeed: bool,
        ) -> Result<AsyncResultOkEnum, AsyncResultErrEnum>;
        async fn rust_async_func_return_result_opaque_rust_and_transparent_enum(
            succeed: bool,
        ) -> Result<AsyncResultOpaqueRustType1, AsyncResultErrEnum>;
        async fn rust_async_func_return_result_transparent_enum_and_opaque_rust(
            succeed: bool,
        ) -> Result<AsyncResultOkEnum, AsyncResultOpaqueRustType1>;
        async fn rust_async_func_return_result_null_and_transparent_enum(
            succeed: bool,
        ) -> Result<(), AsyncResultErrEnum>;
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

async fn rust_async_func_return_result_transparent_enum_and_transparent_enum(
    succeed: bool,
) -> Result<ffi::AsyncResultOkEnum, ffi::AsyncResultErrEnum> {
    if succeed {
        Ok(ffi::AsyncResultOkEnum::UnnamedFields(
            123,
            "hello".to_string(),
        ))
    } else {
        Err(ffi::AsyncResultErrEnum::NamedFields { value: 100 })
    }
}

async fn rust_async_func_return_result_opaque_rust_and_transparent_enum(
    succeed: bool,
) -> Result<AsyncResultOpaqueRustType1, ffi::AsyncResultErrEnum> {
    if succeed {
        Ok(AsyncResultOpaqueRustType1(10))
    } else {
        Err(ffi::AsyncResultErrEnum::NamedFields { value: 1000 })
    }
}

async fn rust_async_func_return_result_transparent_enum_and_opaque_rust(
    succeed: bool,
) -> Result<ffi::AsyncResultOkEnum, AsyncResultOpaqueRustType1> {
    if succeed {
        Ok(ffi::AsyncResultOkEnum::NoFields)
    } else {
        Err(AsyncResultOpaqueRustType1(1000))
    }
}

async fn rust_async_func_return_result_null_and_transparent_enum(
    succeed: bool,
) -> Result<(), ffi::AsyncResultErrEnum> {
    if succeed {
        Ok(())
    } else {
        Err(ffi::AsyncResultErrEnum::UnnamedFields(
            "foo".to_string(),
            123,
        ))
    }
}

async fn rust_async_func_return_result_null_opaque_rust(
    succeed: bool,
) -> Result<(), AsyncResultOpaqueRustType2> {
    if succeed {
        Ok(())
    } else {
        Err(AsyncResultOpaqueRustType2(111))
    }
}
