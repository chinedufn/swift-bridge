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
