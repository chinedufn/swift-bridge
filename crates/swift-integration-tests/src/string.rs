use swift_bridge::string::SwiftString;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn run_string_tests();

        fn create_string(str: &str) -> String;
    }
}

fn run_string_tests() {
    let string = SwiftString::new_with_str("hello");

    assert_eq!(string.len(), 5);

    assert_eq!(string.to_str(), "hello");
}

fn create_string(str: &str) -> String {
    str.to_string()
}
