//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=opaque-copy-type-mut-ref.rs

// We declare a function that returns a mutable Copy type and a method takes takes a mutable
// reference to a Copy type.
//
// We do not support bridge Copy types mutably since the bytes are copied over the bridge.
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Copy(1))]
        type SomeType;

        fn some_method(&mut self);
        #[swift_bridge(rust_name = "some_method")]
        fn some_method2(self: &mut SomeType);

        fn some_function(arg: &mut SomeType);
    }
}

#[derive(Copy, Clone)]
pub struct SomeType(u8);

impl SomeType {
    fn some_method(&mut self) {}
}
fn some_function(_arg: &mut SomeType) {}

fn main() {}
