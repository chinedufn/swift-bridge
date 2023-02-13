//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=incorrect-return-type.rs

pub struct SomeType;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeType;

        #[swift_bridge(rust_name = "some_function")]
        fn fn1() -> SomeType;
        #[swift_bridge(rust_name = "another_function")]
        fn fn2() -> SomeType;
    }
}

fn some_function() -> &'static SomeType {
    &SomeType
}

fn another_function() -> Option<SomeType> {
    None
}

fn main() {}
