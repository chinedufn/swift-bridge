use std::collections::HashMap;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn some_function(arg: String) -> String; 
    }
}
fn some_function(arg: String) -> String {
    arg
}