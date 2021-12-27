#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeType;

        // The `get_another_type` function returns "AnotherType".
        // Yet here we are trying to return "SomeType".
        // So, if this compiles it means that our `into_return_type` macro is working.
        #[swift_bridge(into_return_type)]
        fn get_another_type() -> SomeType;
    }
}

pub struct SomeType;

struct AnotherType;

impl Into<SomeType> for AnotherType {
    fn into(self) -> SomeType {
        SomeType
    }
}

fn get_another_type() -> AnotherType {
    AnotherType
}
