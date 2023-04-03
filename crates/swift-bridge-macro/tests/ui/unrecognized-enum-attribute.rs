//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=unrecognized-enum-attribute.rs

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(InvalidAttribute)]
    enum SomeEnum {}
}

fn main() {}
