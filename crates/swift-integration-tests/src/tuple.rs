#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type TupleTestOpaqueRustType;
        #[swift_bridge(init)]
        fn new(val: i32) -> TupleTestOpaqueRustType;
        fn val(&self) -> i32;
        fn rust_reflect_tuple_primitives(tuple: (i16, u32)) -> (i16, u32);
        fn rust_reflect_tuple_opaque_rust_and_string_and_primitive(
            tuple: (TupleTestOpaqueRustType, String, u8),
        ) -> (TupleTestOpaqueRustType, String, u8);
    }
}

pub struct TupleTestOpaqueRustType(i32);

impl TupleTestOpaqueRustType {
    fn new(val: i32) -> Self {
        TupleTestOpaqueRustType(val)
    }
    fn val(&self) -> i32 {
        self.0
    }
}

fn rust_reflect_tuple_primitives(tuple: (i16, u32)) -> (i16, u32) {
    tuple
}

fn rust_reflect_tuple_opaque_rust_and_string_and_primitive(
    tuple: (TupleTestOpaqueRustType, String, u8),
) -> (TupleTestOpaqueRustType, String, u8) {
    tuple
}
