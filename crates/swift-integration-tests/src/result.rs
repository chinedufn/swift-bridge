enum ResultErrorType {
    Error1,
    Error2,
}

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_reflect_okay_result_i8(val: i8) -> Result<i8, ResultErrorType>;
    }
}

pub fn rust_reflect_okay_result_i8(val: i8) -> Result<i8, ResultErrorType> {
    Ok(val)
}
//pub fn rust_reflect_option_bool(arg: Option<bool>) -> Option<bool> { arg }
