#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn reflect_tuple_primitive_types(tuple: (i16, u32)) -> (i16, u32);
        fn reflect_tuple_string_and_primitive_type(tuple: (String, i32)) -> (String, i32);
    }
}

fn reflect_tuple_primitive_types(tuple: (i16, u32)) -> (i16, u32) {
    tuple
}

fn reflect_tuple_string_and_primitive_type(tuple: (String, i32)) -> (String, i32) {
    tuple
}