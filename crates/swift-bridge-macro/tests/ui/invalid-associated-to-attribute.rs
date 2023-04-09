//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=invalid-associated-to-attribute.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeType;

        #[swift_bridge(associated_to = SomeType)]
        fn immutable_method(&self);

        #[swift_bridge(associated_to = SomeType)]
        fn mutable_method(&mut self);

        #[swift_bridge(associated_to = SomeType)]
        fn owned_method(self);
    }
}

pub struct SomeType;

impl SomeType {
    fn immutable_method(&self) {

    }
    fn mutable_method(&mut self) {

    }
    fn owned_method() {

    }
}

fn main() {}