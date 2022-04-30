//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=unrecognized-opaque-type-attribute.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(InvalidAttribute)]
        type SomeType;
    }
}

pub struct SomeType;

fn main() {}
