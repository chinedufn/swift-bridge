#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn test_argument_label(
            #[swift_bridge(label = "someArg")] some_arg: i32,
            another_arg: i32,
        ) -> i32;
    }
}

fn test_argument_label(some_arg: i32, another_arg: i32) -> i32 {
    some_arg + another_arg
}
