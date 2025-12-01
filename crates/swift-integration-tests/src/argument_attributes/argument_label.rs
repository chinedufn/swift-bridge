#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn test_argument_label(
            #[swift_bridge(label = "someArg")] some_arg: i32,
            another_arg: i32,
        ) -> i32;
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

#[cfg(test)]
mod tests {
    use super::ffi::*;

    #[test]
    fn test_swift_func_with_unlabeled_params() {
        let result = swift_func_with_unlabeled_params(10, 20);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_swift_func_with_custom_labels() {
        let result = swift_func_with_custom_labels(5, 3);
        assert_eq!(result, 8);
    }
}
