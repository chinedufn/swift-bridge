#[swift_bridge::bridge]
mod ffi {
    extern "Swift" {
        // If this compiles then we're successfully using the `rust_name` during code generation.
        #[swift_bridge(swift_name = "testCallSwiftFromRustByNameAttribute")]
        fn test_call_swift_from_rust_by_name_attribute() -> String;
    }

    extern "Rust" {
        #[swift_bridge(swift_name = "testCallRustFromSwiftByNameAttribute")]
        pub fn test_call_rust_from_swift_by_name_attribute() -> String;
    }
}

/// The test on the Swift side will call this function, which in turn will reach into
/// Rust, then the Rust code will call into Swift, and we assert that we got the correct
/// string back from Swift, before returning another string back to Swift.
fn test_call_rust_from_swift_by_name_attribute() -> String {
    assert_eq!(
        ffi::test_call_swift_from_rust_by_name_attribute(),
        "StringFromSwift"
    );
    "StringFromRust".to_string()
}
