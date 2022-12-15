//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=invalid-module-item.rs

#[swift_bridge::bridge]
mod ffi {
    use std;
    fn foo() {}
}

fn main() {}
