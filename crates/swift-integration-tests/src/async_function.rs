#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        async fn rust_async_u8() -> u8;
    }
}

async fn rust_async_u8() -> u8 {
    123
}
