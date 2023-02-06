//! See also: crates/swift-bridge-ir/src/codegen/codegen_tests/result_codegen_tests.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_func_reflect_result_opaque_rust(
            arg: Result<ResultTestOpaqueRustType, ResultTestOpaqueRustType>,
        ) -> Result<ResultTestOpaqueRustType, ResultTestOpaqueRustType>;

        fn rust_func_takes_result_string(arg: Result<String, String>);
        fn rust_func_takes_result_opaque_swift(
            arg: Result<ResultTestOpaqueSwiftType, ResultTestOpaqueSwiftType>,
        );

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
