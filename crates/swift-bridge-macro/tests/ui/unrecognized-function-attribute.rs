//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=unrecognized-function-attribute.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(InvalidAttribute)]
        fn some_function();
    }
}

pub struct SomeType;

fn main() {}
