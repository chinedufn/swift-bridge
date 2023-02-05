//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=unrecognized-argument-attribute.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn some_function(
            #[swift_bridge(InvalidArgumentAttribute)] some_value: isize
        );
    }
}

fn main() {}
