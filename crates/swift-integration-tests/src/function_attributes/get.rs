#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeTypeGet;

        #[swift_bridge(init)]
        fn new() -> SomeTypeGet;

        #[swift_bridge(get(my_u8))]
        fn my_u8(&self) -> u8;

        #[swift_bridge(get(&my_string))]
        fn my_string_reference(&self) -> &str;

        #[swift_bridge(get(my_opt_static_str))]
        fn my_opt_static_str(&self) -> Option<&'static str>;
    }
}

pub struct SomeTypeGet {
    my_u8: u8,
    my_string: String,
    my_opt_static_str: Option<&'static str>,
}

impl SomeTypeGet {
    fn new() -> SomeTypeGet {
        SomeTypeGet {
            my_u8: 123,
            my_string: "Hello".to_string(),
            my_opt_static_str: Some("world"),
        }
    }
}
