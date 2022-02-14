//! If this file compiles then we know that our return value was converted.

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(return_with = some_module::convert_str_to_u32)]
        fn get_str_value_return_with() -> u32;
    }
}

fn get_str_value_return_with() -> &'static str {
    "123"
}

mod some_module {
    pub fn convert_str_to_u32(val: &str) -> u32 {
        val.parse().unwrap()
    }
}
