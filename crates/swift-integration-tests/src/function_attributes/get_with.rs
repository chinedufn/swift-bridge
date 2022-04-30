use Clone;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeTypeGetWith;

        // Returns ui_to_i16(self.my_u8)
        #[swift_bridge(get_with(my_u8 = u8_to_i16))]
        fn my_u8_converted(&self) -> i16;

        // Returns Clone::clone(&self.my_string)
        #[swift_bridge(get_with(&my_string = Clone::clone))]
        fn my_string_cloned(&self) -> String;

        // Returns string_to_u32(&self.my_string)
        #[swift_bridge(get_with(&my_string = string_to_u32))]
        fn my_string_parsed(&self) -> u32;
    }
}

pub struct SomeTypeGetWith {
    my_u8: u8,
    my_string: String,
}

fn u8_to_i16(num: u8) -> i16 {
    num as i16
}

fn string_to_u32(string: &str) -> u32 {
    string.parse().unwrap()
}
