#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_negate_bool(start: bool) -> bool;

        fn run_bool_test();
    }

    extern "Swift" {
        #[swift_bridge(swift_name = "swiftNegateBool")]
        fn swift_negate_bool(start: bool) -> bool;
    }
}

fn run_bool_test() {
    assert_eq!(ffi::swift_negate_bool(true), false);
    assert_eq!(ffi::swift_negate_bool(false), true);
}

fn rust_negate_bool(start: bool) -> bool {
    !start
}
