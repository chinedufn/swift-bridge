#[swift_bridge::bridge]
mod ffi {
    enum TupleTestEnum {
        NamedField { data: i32 },
        UnnamedFields(u8, String),
        NoFields,
    }
    #[swift_bridge(swift_repr = "struct")]
    struct TupleTestStruct {
        field: u8,
    }
    extern "Rust" {
        type TupleTestOpaqueRustType;
        #[swift_bridge(init)]
        fn new(val: i32) -> TupleTestOpaqueRustType;
        fn val(&self) -> i32;
        fn rust_reflect_tuple_primitives(tuple: (i16, u32)) -> (i16, u32);
        fn rust_reflect_tuple_opaque_rust_and_string_and_primitive(
            tuple: (TupleTestOpaqueRustType, String, u8),
        ) -> (TupleTestOpaqueRustType, String, u8);
        fn rust_reflect_tuple_f64_and_usize_and_bool(
            tuple: (f64, usize, bool),
        ) -> (f64, usize, bool);
    }
    extern "Swift" {
        fn swift_reflect_tuple_primitives(arg: (i32, u32)) -> (i32, u32);
        fn swift_reflect_opaque_and_primitive_tuple(
            arg: (TupleTestOpaqueRustType, i32),
        ) -> (TupleTestOpaqueRustType, i32);
        fn swift_reflect_struct_and_enum_and_string(
            arg: (TupleTestStruct, TupleTestEnum, String),
        ) -> (TupleTestStruct, TupleTestEnum, String);
    }
    extern "Rust" {
        fn test_rust_calls_swift_tuples();
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

fn rust_reflect_tuple_f64_and_usize_and_bool(tuple: (f64, usize, bool)) -> (f64, usize, bool) {
    tuple
}

fn test_rust_calls_swift_tuples() {
    let val = ffi::swift_reflect_tuple_primitives((-123, 123));
    assert_eq!(val.0, -123);
    assert_eq!(val.1, 123);

    let val = ffi::swift_reflect_opaque_and_primitive_tuple((TupleTestOpaqueRustType(123), -123));
    assert_eq!(val.0 .0, 123);
    assert_eq!(val.1, -123);

    let val = ffi::swift_reflect_struct_and_enum_and_string((
        ffi::TupleTestStruct { field: 123 },
        ffi::TupleTestEnum::NamedField { data: -123 },
        "hello, world".to_string(),
    ));
    assert_eq!(val.0.field, 123);
    assert!(matches!(
        val.1,
        ffi::TupleTestEnum::NamedField { data: -123 }
    ));
    assert_eq!(val.2, "hello, world".to_string());
}
