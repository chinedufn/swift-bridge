#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_negate_bool(start: bool) -> bool;

        fn run_bool_test();
    }

    extern "Swift" {
        fn swiftNegateBool(start: bool) -> bool;
    }
}

fn run_bool_test() {
    assert_eq!(ffi::swiftNegateBool(true), false);
    assert_eq!(ffi::swiftNegateBool(false), true);
}

fn rust_negate_bool(start: bool) -> bool {
    !start
}
