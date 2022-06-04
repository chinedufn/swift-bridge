#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn hello_rust() -> String;
    }

    #[swift_bridge(swift_repr = "struct")]
    struct SomeStruct {
        field: u8
    }

    #[swift_bridge(swift_repr = "struct")]
    struct UnnamedStruct(u8);
}

fn hello_rust() -> String {
    String::from("Hello, From Rust!")
}
