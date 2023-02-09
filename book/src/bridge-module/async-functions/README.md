# Async Functions

`swift-bridge` supports async/await between Swift and Rust.

## Async Rust Functions

```rust
#[swift_bridge::bridge]
mod ffi {
    struct MyStruct;
    
    extern "Rust" {
        async fn some_async_function(list: Vec<u8>) -> MyStruct;
    }
}

async fn some_async_function(list: Vec<u8>) -> ffi::MyStruct {
    ffi::MyStruct
}
```

## Return `Result<OpaqueType, OpaqueType>` from async functions

`swift-bridge` allows async functions to return `Result<OpaqueType, OpaqueType>`.

```rust

#[swift_bridge::bridge]
mod ffi {    
    extern "Rust" {
        type User;
        type ApiError;

        async fn user_count() -> u32;
        async fn load_user(url: &str) -> Result<User, ApiError>;
    }
}
```

```
// Swift

let totalUsers = await user_count()

do {
    let user = try await load_user("https://example.com/users/5")
} catch let error as ApiError {
    // ... error handling ...
}
```