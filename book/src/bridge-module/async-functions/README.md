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
        type OpaqueType1;
        type OpaqueType2;
        async fn some_function() -> Result<OpaqueType1, OpaqueType2>;
    }
}
```