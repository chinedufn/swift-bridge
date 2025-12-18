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

        async fn rust_async_reflect_string(string: String) -> String;
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
        UnnamedFields(i32, String),
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

// =============================================================================
// Tests for Rust calling async Swift functions
// =============================================================================

#[swift_bridge::bridge]
mod ffi_swift_async {
    extern "Rust" {
        // Rust functions that call Swift async functions - called from Swift tests
        fn rust_calls_swift_async_void() -> bool;
        fn rust_calls_swift_async_return_u32() -> u32;
        fn rust_calls_swift_async_return_string() -> String;
        fn rust_calls_swift_async_throws_ok() -> u32;
        fn rust_calls_swift_async_throws_err() -> u32;
    }

    // Shared enum error type for async throws tests - can be created on both Rust and Swift sides
    enum SwiftAsyncError {
        ErrorWithValue(u32),
    }

    extern "Swift" {
        async fn swift_async_void();
        async fn swift_async_return_u32() -> u32;
        async fn swift_async_return_string() -> String;
        async fn swift_async_throws(succeed: bool) -> Result<u32, SwiftAsyncError>;
    }
}

// Use tokio runtime to block on async Swift calls
fn rust_calls_swift_async_void() -> bool {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        ffi_swift_async::swift_async_void().await;
        true
    })
}

fn rust_calls_swift_async_return_u32() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { ffi_swift_async::swift_async_return_u32().await })
}

fn rust_calls_swift_async_return_string() -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { ffi_swift_async::swift_async_return_string().await })
}

fn rust_calls_swift_async_throws_ok() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match ffi_swift_async::swift_async_throws(true).await {
            Ok(val) => val,
            Err(_) => panic!("Expected Ok, got Err"),
        }
    })
}

fn rust_calls_swift_async_throws_err() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match ffi_swift_async::swift_async_throws(false).await {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(ffi_swift_async::SwiftAsyncError::ErrorWithValue(val)) => val,
        }
    })
}

// =============================================================================
// Tests for Rust calling async Swift functions that return Result<(), E>
// =============================================================================

#[swift_bridge::bridge]
mod ffi_swift_async_result_void {
    extern "Rust" {
        // Rust functions that call Swift async functions returning Result<(), E>
        fn rust_calls_swift_async_throws_void_ok() -> bool;
        fn rust_calls_swift_async_throws_void_err() -> u32;
    }

    // Shared enum error type for async throws tests
    enum SwiftAsyncVoidError {
        ErrorWithValue(u32),
    }

    extern "Swift" {
        async fn swift_async_throws_void(succeed: bool) -> Result<(), SwiftAsyncVoidError>;
    }
}

fn rust_calls_swift_async_throws_void_ok() -> bool {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match ffi_swift_async_result_void::swift_async_throws_void(true).await {
            Ok(()) => true,
            Err(_) => panic!("Expected Ok(()), got Err"),
        }
    })
}

fn rust_calls_swift_async_throws_void_err() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match ffi_swift_async_result_void::swift_async_throws_void(false).await {
            Ok(()) => panic!("Expected Err, got Ok"),
            Err(ffi_swift_async_result_void::SwiftAsyncVoidError::ErrorWithValue(val)) => val,
        }
    })
}

// =============================================================================
// Tests for Rust calling async Swift methods with &self
// =============================================================================

#[swift_bridge::bridge]
mod ffi_swift_async_method {
    extern "Rust" {
        // Rust functions that call Swift async methods - called from Swift tests
        fn rust_calls_swift_async_method_return_u32() -> u32;
        fn rust_calls_swift_async_method_with_args() -> u32;
        fn rust_calls_swift_async_method_throws_ok() -> u32;
        fn rust_calls_swift_async_method_throws_err() -> u32;
        fn rust_calls_swift_async_method_throws_void_ok() -> bool;
        fn rust_calls_swift_async_method_throws_void_err() -> u32;
    }

    // Shared enum error type for async method throws tests
    enum SwiftAsyncMethodError {
        ErrorWithValue(u32),
    }

    extern "Swift" {
        type AsyncSwiftType;

        #[swift_bridge(init)]
        fn new(value: u32) -> AsyncSwiftType;

        // Simple async method returning a value
        async fn get_value(&self) -> u32;

        // Async method with arguments
        async fn add_to_value(&self, amount: u32) -> u32;

        // Async method returning Result<T, E>
        async fn maybe_get_value(&self, succeed: bool) -> Result<u32, SwiftAsyncMethodError>;

        // Async method returning Result<(), E>
        async fn maybe_succeed(&self, succeed: bool) -> Result<(), SwiftAsyncMethodError>;
    }
}

fn rust_calls_swift_async_method_return_u32() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let obj = ffi_swift_async_method::AsyncSwiftType::new(42);
        obj.get_value().await
    })
}

fn rust_calls_swift_async_method_with_args() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let obj = ffi_swift_async_method::AsyncSwiftType::new(100);
        obj.add_to_value(50).await
    })
}

fn rust_calls_swift_async_method_throws_ok() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let obj = ffi_swift_async_method::AsyncSwiftType::new(999);
        match obj.maybe_get_value(true).await {
            Ok(val) => val,
            Err(_) => panic!("Expected Ok, got Err"),
        }
    })
}

fn rust_calls_swift_async_method_throws_err() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let obj = ffi_swift_async_method::AsyncSwiftType::new(123);
        match obj.maybe_get_value(false).await {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(ffi_swift_async_method::SwiftAsyncMethodError::ErrorWithValue(val)) => val,
        }
    })
}

fn rust_calls_swift_async_method_throws_void_ok() -> bool {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let obj = ffi_swift_async_method::AsyncSwiftType::new(0);
        match obj.maybe_succeed(true).await {
            Ok(()) => true,
            Err(_) => panic!("Expected Ok(()), got Err"),
        }
    })
}

fn rust_calls_swift_async_method_throws_void_err() -> u32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let obj = ffi_swift_async_method::AsyncSwiftType::new(0);
        match obj.maybe_succeed(false).await {
            Ok(()) => panic!("Expected Err, got Ok"),
            Err(ffi_swift_async_method::SwiftAsyncMethodError::ErrorWithValue(val)) => val,
        }
    })
}
