#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        // Here we declare functions that have an argument with a type that does not match that
        // of the real implementation below.
        //
        // Non FFI safe types such as `&str` are passed over the FFI boundary using different types
        // such as `swift_bridge::string::RustStr`, so our tests confirm that our codegen preserves
        // the span of the original argument type.

        #[swift_bridge(rust_name = "some_function")]
        fn fn1(arg: &str);
        // TODO: Add more types.
        // #[swift(bridge(rust_name = function))]
        // fn fn2(arg: String);
        // #[swift(bridge(rust_name = function))]
        // fn fn3(arg: Option<u8>);
    }
}

fn some_function(_arg: u16) {}

fn main() {}
