#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn test_argument_label(
            #[swift_bridge(label = "someArg")] some_arg: i32,
            another_arg: i32,
        ) -> i32;

        fn rust_calls_swift_with_unlabeled_params(a: i32, b: i32) -> i32;

        fn rust_calls_swift_with_custom_labels(a: i32, b: i32) -> i32;
    }

    extern "Swift" {
        // Swift function: func swift_func_with_unlabeled_params(_ a: Int32, _ b: Int32) -> Int32
        fn swift_func_with_unlabeled_params(
            #[swift_bridge(label = "_")] a: i32,
            #[swift_bridge(label = "_")] b: i32,
        ) -> i32;

        // Swift function: func swift_func_with_custom_labels(first a: Int32, second b: Int32) -> Int32
        fn swift_func_with_custom_labels(
            #[swift_bridge(label = "first")] a: i32,
            #[swift_bridge(label = "second")] b: i32,
        ) -> i32;
    }
}

fn test_argument_label(some_arg: i32, another_arg: i32) -> i32 {
    some_arg + another_arg
}

fn rust_calls_swift_with_unlabeled_params(a: i32, b: i32) -> i32 {
    ffi::swift_func_with_unlabeled_params(a, b)
}

fn rust_calls_swift_with_custom_labels(a: i32, b: i32) -> i32 {
    ffi::swift_func_with_custom_labels(a, b)
}
