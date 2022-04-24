
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {

        //#[swift_bridge(rust_name = "some_function")]
        fn fn1() -> Result<i8,i8>;
    }
}

fn fn1() -> Option<String>{None}

fn main() {}
