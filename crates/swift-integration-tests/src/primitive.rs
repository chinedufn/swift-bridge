#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn test_rust_calls_swift_primitives();

        fn rust_double_u8(arg: u8) -> u8;
        fn rust_double_i8(arg: i8) -> i8;
        fn rust_double_u16(arg: u16) -> u16;
        fn rust_double_i16(arg: i16) -> i16;
        fn rust_double_u32(arg: u32) -> u32;
        fn rust_double_i32(arg: i32) -> i32;
        fn rust_double_u64(arg: u64) -> u64;
        fn rust_double_i64(arg: i64) -> i64;
        fn rust_double_f32(arg: f32) -> f32;
        fn rust_double_f64(arg: f64) -> f64;
        fn rust_negate_bool(arg: bool) -> bool;
    }

    extern "Swift" {
        fn swift_double_u8(arg: u8) -> u8;
        fn swift_double_i8(arg: i8) -> i8;
        fn swift_double_u16(arg: u16) -> u16;
        fn swift_double_i16(arg: i16) -> i16;
        fn swift_double_u32(arg: u32) -> u32;
        fn swift_double_i32(arg: i32) -> i32;
        fn swift_double_u64(arg: u64) -> u64;
        fn swift_double_i64(arg: i64) -> i64;
        fn swift_double_f32(arg: f32) -> f32;
        fn swift_double_f64(arg: f64) -> f64;
        fn swift_negate_bool(arg: bool) -> bool;
    }
}

fn test_rust_calls_swift_primitives() {
    assert_eq!(ffi::swift_double_u8(5), 10);
    assert_eq!(ffi::swift_double_i8(5), 10);
    assert_eq!(ffi::swift_double_u16(5), 10);
    assert_eq!(ffi::swift_double_i16(5), 10);
    assert_eq!(ffi::swift_double_u32(5), 10);
    assert_eq!(ffi::swift_double_i32(5), 10);
    assert_eq!(ffi::swift_double_u64(5), 10);
    assert_eq!(ffi::swift_double_i64(5), 10);
    assert_eq!(ffi::swift_double_f32(5.), 10.);
    assert_eq!(ffi::swift_double_f64(5.), 10.);
    assert_eq!(ffi::swift_negate_bool(true), false);
    assert_eq!(ffi::swift_negate_bool(false), true);
}

fn rust_double_u8(arg: u8) -> u8 {
    arg * 2
}

fn rust_double_i8(arg: i8) -> i8 {
    arg * 2
}

fn rust_double_u16(arg: u16) -> u16 {
    arg * 2
}

fn rust_double_i16(arg: i16) -> i16 {
    arg * 2
}

fn rust_double_u32(arg: u32) -> u32 {
    arg * 2
}

fn rust_double_i32(arg: i32) -> i32 {
    arg * 2
}

fn rust_double_u64(arg: u64) -> u64 {
    arg * 2
}

fn rust_double_i64(arg: i64) -> i64 {
    arg * 2
}

fn rust_double_f32(arg: f32) -> f32 {
    arg * 2.
}

fn rust_double_f64(arg: f64) -> f64 {
    arg * 2.
}

fn rust_negate_bool(arg: bool) -> bool {
    !arg
}
