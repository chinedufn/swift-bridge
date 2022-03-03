#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn hello_rust() -> String;
    }
}

fn hello_rust() -> String {
    String::from("Hello Rust!")
}