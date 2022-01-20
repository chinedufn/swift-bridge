#[swift_bridge::bridge]
mod ffi {
    struct StructWithNoFields;

    // Unused, we're just verifying that this generates Rust and Swift code that compiles
    //  without any Rust warnings (our test suite runs with warnings as errors).
    struct StructWithNoFieldsTuple();
    // Unused, we're just verifying that this generates Rust and Swift code that compiles
    //  without any Rust warnings (our test suite runs with warnings as errors).
    struct StructWithNoFieldsNamed {}

    #[swift_bridge(swift_repr = "struct")]
    struct StructReprStructWithOnePrimitiveField {
        named_field: u8,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct StructReprStructTupleStruct(u8, u32);

    extern "Rust" {
        fn test_rust_calls_swift();

        fn swift_calls_rust_struct_with_no_fields(arg: StructWithNoFields) -> StructWithNoFields;

        fn swift_calls_rust_struct_repr_struct_one_primitive_field(
            arg: StructReprStructWithOnePrimitiveField,
        ) -> StructReprStructWithOnePrimitiveField;

        fn swift_calls_rust_tuple_struct(
            arg: StructReprStructTupleStruct,
        ) -> StructReprStructTupleStruct;
    }

    extern "Swift" {
        fn rust_calls_swift_struct_with_no_fields(arg: StructWithNoFields) -> StructWithNoFields;

        fn rust_calls_struct_repr_struct_one_primitive_field(
            arg: StructReprStructWithOnePrimitiveField,
        ) -> StructReprStructWithOnePrimitiveField;
    }
}

fn test_rust_calls_swift() {
    self::tests::test_rust_calls_swift_struct_with_no_fields();
    self::tests::test_rust_calls_struct_repr_struct_one_primitive_field();
}

fn swift_calls_rust_struct_with_no_fields(arg: ffi::StructWithNoFields) -> ffi::StructWithNoFields {
    arg
}

fn swift_calls_rust_struct_repr_struct_one_primitive_field(
    arg: ffi::StructReprStructWithOnePrimitiveField,
) -> ffi::StructReprStructWithOnePrimitiveField {
    arg
}

fn swift_calls_rust_tuple_struct(
    arg: ffi::StructReprStructTupleStruct,
) -> ffi::StructReprStructTupleStruct {
    arg
}

#[deny(unused)]
mod tests {
    use super::ffi;

    pub(super) fn test_rust_calls_swift_struct_with_no_fields() {
        let _: ffi::StructWithNoFields =
            ffi::rust_calls_swift_struct_with_no_fields(ffi::StructWithNoFields);
    }

    pub(super) fn test_rust_calls_struct_repr_struct_one_primitive_field() {
        let arg = ffi::StructReprStructWithOnePrimitiveField { named_field: 10 };

        let val = ffi::rust_calls_struct_repr_struct_one_primitive_field(arg);

        assert_eq!(val.named_field, 10);
    }
}
