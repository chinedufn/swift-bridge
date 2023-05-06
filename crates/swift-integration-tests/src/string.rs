#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn run_string_tests();

        fn create_string(str: &str) -> String;
    }

    extern "Swift" {
        fn create_swift_string() -> String;
        fn reflect_swift_string(arg: String) -> String;
    }
}

fn run_string_tests() {
    let string = ffi::create_swift_string();
    assert_eq!(string.len(), 5);
    assert_eq!(&string, "hello");

    let foo = "foo";
    let string = ffi::reflect_swift_string(foo.to_string());
    assert_eq!(string.len(), 3);
    assert_eq!(&string, foo);
}

fn create_string(str: &str) -> String {
    str.to_string()
}
