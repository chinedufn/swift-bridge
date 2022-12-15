#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn run_string_tests();

        fn create_string(str: &str) -> String;
    }

    extern "Swift" {
        fn create_swift_string() -> String;
    }
}

fn run_string_tests() {
    let string = ffi::create_swift_string();
    assert_eq!(string.len(), 5);
    assert_eq!(&string, "hello");
}

fn create_string(str: &str) -> String {
    str.to_string()
}
