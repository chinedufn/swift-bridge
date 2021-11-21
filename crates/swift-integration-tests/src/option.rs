#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn create_rust_option_u8_some() -> Option<u8>;
        fn create_rust_option_u8_none() -> Option<u8>;

        fn run_option_tests();
    }

    extern "Swift" {
        fn create_swift_option_u8_some() -> Option<u8>;
        fn create_swift_option_u8_none() -> Option<u8>;
    }
}

fn run_option_tests() {
    assert_eq!(ffi::create_swift_option_u8_some(), Some(55));
    assert_eq!(ffi::create_swift_option_u8_none(), None);
}

fn create_rust_option_u8_some() -> Option<u8> {
    Some(70)
}

fn create_rust_option_u8_none() -> Option<u8> {
    None
}
