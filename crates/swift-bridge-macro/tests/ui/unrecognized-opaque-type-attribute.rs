//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=unrecognized-opaque-type-attribute.rs

// <!-- ANCHOR: mdbook-ui-test-example -->
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(InvalidAttribute)]
        type SomeType;
    }
}

pub struct SomeType;
// <!-- ANCHOR_END: mdbook-ui-test-example -->

fn main() {}
