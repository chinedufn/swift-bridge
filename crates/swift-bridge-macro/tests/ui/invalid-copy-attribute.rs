//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=invalid-copy-attribute.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Copy(1))]
        type DoesNotImplementCopy;

        #[swift_bridge(Copy(10))]
        type IncorrectCopySize;

    }
}

pub struct DoesNotImplementCopy(u8);

#[derive(Copy, Clone)]
pub struct IncorrectCopySize([u8; 9]);

fn main() {}
