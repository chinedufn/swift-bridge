#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        // If this compiles then we're successfully using the `rust_name` during code generation.
        #[swift_bridge(rust_name = "another_function")]
        fn some_function();
    }
}

fn another_function() {}
