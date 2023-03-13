use std::collections::HashMap;

#[swift_bridge::bridge]
mod ffi {

    extern "Rust" {
        fn some_function(arg: (String, i32))  -> (String, i32);
        fn foo(arg: String) -> String;
    }
}

fn some_function(arg: (String, i32))  -> (String, i32) {
    arg
}

fn foo(arg: String) -> String {
    arg
}
