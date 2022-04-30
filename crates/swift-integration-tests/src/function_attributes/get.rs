#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeTypeGet;

        #[swift_bridge(get(my_u8))]
        fn my_u8(&self) -> u8;

        #[swift_bridge(get(&my_string))]
        fn my_string_reference(&self) -> &str;
    }
}

pub struct SomeTypeGet {
    my_u8: u8,
    my_string: String,
}
