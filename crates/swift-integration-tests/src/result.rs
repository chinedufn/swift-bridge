//! See also: crates/swift-bridge-ir/src/codegen/codegen_tests/result_codegen_tests.rs

#[swift_bridge::bridge]
mod ffi {
    struct UnitStruct;

    extern "Rust" {
        fn rust_func_reflect_result_opaque_rust(
            arg: Result<ResultTestOpaqueRustType, ResultTestOpaqueRustType>,
        ) -> Result<ResultTestOpaqueRustType, ResultTestOpaqueRustType>;

        fn rust_func_takes_result_string(arg: Result<String, String>);
        fn rust_func_takes_result_opaque_swift(
            arg: Result<ResultTestOpaqueSwiftType, ResultTestOpaqueSwiftType>,
        );

        fn rust_func_return_result_null_opaque_rust(
            succeed: bool,
        ) -> Result<(), ResultTestOpaqueRustType>;

        fn rust_func_return_result_unit_struct_opaque_rust(
            succeed: bool,
        ) -> Result<UnitStruct, ResultTestOpaqueRustType>;
    }

    extern "Rust" {
        type ResultTestOpaqueRustType;

        #[swift_bridge(init)]
        fn new(val: u32) -> ResultTestOpaqueRustType;
        fn val(&self) -> u32;
    }

    extern "Swift" {
        type ResultTestOpaqueSwiftType;

        fn val(&self) -> u32;
    }

    enum ResultTransparentEnum {
        NamedField { data: i32 },
        UnnamedFields(u8, String),
        NoFields,
    }

    extern "Rust" {
        fn rust_func_return_result_transparent_enum_opaque_rust(
            succeed: bool,
        ) -> Result<ResultTestOpaqueRustType, ResultTransparentEnum>;
    }

    extern "Rust" {
        fn rust_func_return_result_unit_type_enum_opaque_rust(
            succeed: bool,
        ) -> Result<(), ResultTransparentEnum>;
    }
}

fn rust_func_takes_result_string(arg: Result<String, String>) {
    match arg {
        Ok(ok) => {
            assert_eq!(ok, "Success Message")
        }
        Err(err) => {
            assert_eq!(err, "Error Message")
        }
    }
}

fn rust_func_reflect_result_opaque_rust(
    arg: Result<ResultTestOpaqueRustType, ResultTestOpaqueRustType>,
) -> Result<ResultTestOpaqueRustType, ResultTestOpaqueRustType> {
    match arg {
        Ok(ok) => {
            assert_eq!(ok.val, 111);
            Ok(ok)
        }
        Err(err) => {
            assert_eq!(err.val, 222);
            Err(err)
        }
    }
}

fn rust_func_takes_result_opaque_swift(
    arg: Result<ffi::ResultTestOpaqueSwiftType, ffi::ResultTestOpaqueSwiftType>,
) {
    match arg {
        Ok(ok) => {
            assert_eq!(ok.val(), 555)
        }
        Err(err) => {
            assert_eq!(err.val(), 666)
        }
    }
}

fn rust_func_return_result_null_opaque_rust(succeed: bool) -> Result<(), ResultTestOpaqueRustType> {
    if succeed {
        Ok(())
    } else {
        Err(ResultTestOpaqueRustType { val: 222 })
    }
}

fn rust_func_return_result_unit_struct_opaque_rust(
    succeed: bool,
) -> Result<ffi::UnitStruct, ResultTestOpaqueRustType> {
    if succeed {
        Ok(ffi::UnitStruct)
    } else {
        Err(ResultTestOpaqueRustType { val: 222 })
    }
}

pub struct ResultTestOpaqueRustType {
    val: u32,
}
impl ResultTestOpaqueRustType {
    fn new(val: u32) -> Self {
        Self { val }
    }

    fn val(&self) -> u32 {
        self.val
    }
}

fn rust_func_return_result_transparent_enum_opaque_rust(
    succeed: bool,
) -> Result<ResultTestOpaqueRustType, ffi::ResultTransparentEnum> {
    if succeed {
        Ok(ResultTestOpaqueRustType::new(123))
    } else {
        Err(ffi::ResultTransparentEnum::NamedField { data: 123 })
    }
}


fn rust_func_return_result_unit_type_enum_opaque_rust(
    succeed: bool,
) -> Result<(), ffi::ResultTransparentEnum> {
    if succeed {
        Ok(())
    } else {
        Err(ffi::ResultTransparentEnum::NamedField { data: 123 })
    }
}