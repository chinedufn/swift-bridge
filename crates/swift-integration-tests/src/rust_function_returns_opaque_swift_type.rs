#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_fn_return_opaque_swift_type() -> SomeSwiftType;
    }

    extern "Swift" {
        type SomeSwiftType;

        #[swift_bridge(init)]
        fn new() -> SomeSwiftType;

        #[swift_bridge(swift_name = "setText")]
        fn set_text(&self, text: &str);
    }
}

fn rust_fn_return_opaque_swift_type() -> ffi::SomeSwiftType {
    let some_swift_type = ffi::SomeSwiftType::new();

    some_swift_type.set_text("I was initialized from Rust");

    some_swift_type
}
