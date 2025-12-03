//! See also: crates/swift-bridge-ir/src/codegen/codegen_tests/option_codegen_tests.rs

use ffi::OptTestOpaqueSwiftType;

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct StructWithOptionFields {
        u8: Option<u8>,
        i8: Option<i8>,
        u16: Option<u16>,
        i16: Option<i16>,
        u32: Option<u32>,
        i32: Option<i32>,
        u64: Option<u64>,
        i64: Option<i64>,
        usize: Option<usize>,
        isize: Option<isize>,
        f32: Option<f32>,
        f64: Option<f64>,
        boolean: Option<bool>,
        string: Option<String>,
        // TODO: Support more types:
        // str: Option<&'static str>,
    }

    // An enum where none of the variants have data.
    enum OptionEnumWithNoData {
        Variant1,
        Variant2,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct OptionStruct {
        field: u8,
    }

    extern "Rust" {
        type OptTestOpaqueRustType;
        type OptTestOpaqueRefRustType;

        #[swift_bridge(init)]
        fn new(field: u8) -> OptTestOpaqueRustType;
        fn field(self: &OptTestOpaqueRustType) -> u8;

        #[swift_bridge(associated_to = OptTestOpaqueRefRustType)]
        fn new(field: u8) -> OptTestOpaqueRefRustType;
        fn field_ref(self: &OptTestOpaqueRefRustType) -> Option<&OptTestOpaqueRustType>;
    }

    extern "Rust" {
        #[swift_bridge(Copy(1))]
        type OptTestOpaqueRustCopyType;
        fn new_opaque_rust_copy_type(field: u8) -> OptTestOpaqueRustCopyType;
    }

    extern "Rust" {
        #[swift_bridge(declare_generic)]
        type OptTestGenericOpaqueRustType<A>;
        type OptTestGenericOpaqueRustType<u8>;
        fn new_generic_opaque_rust_type(field: u8) -> OptTestGenericOpaqueRustType<u8>;
    }

    extern "Rust" {
        #[swift_bridge(Copy(1))]
        type OptTestGenericOpaqueRustCopyType<u8>;
        fn new_generic_opaque_rust_copy_type(field: u8) -> OptTestGenericOpaqueRustCopyType<u8>;
    }

    extern "Rust" {
        fn rust_reflect_option_u8(arg: Option<u8>) -> Option<u8>;
        fn rust_reflect_option_i8(arg: Option<i8>) -> Option<i8>;
        fn rust_reflect_option_u16(arg: Option<u16>) -> Option<u16>;
        fn rust_reflect_option_i16(arg: Option<i16>) -> Option<i16>;
        fn rust_reflect_option_u32(arg: Option<u32>) -> Option<u32>;
        fn rust_reflect_option_i32(arg: Option<i32>) -> Option<i32>;
        fn rust_reflect_option_u64(arg: Option<u64>) -> Option<u64>;
        fn rust_reflect_option_i64(arg: Option<i64>) -> Option<i64>;
        fn rust_reflect_option_usize(arg: Option<usize>) -> Option<usize>;
        fn rust_reflect_option_isize(arg: Option<isize>) -> Option<isize>;
        fn rust_reflect_option_f32(arg: Option<f32>) -> Option<f32>;
        fn rust_reflect_option_f64(arg: Option<f64>) -> Option<f64>;
        fn rust_reflect_option_bool(arg: Option<bool>) -> Option<bool>;

        fn rust_reflect_option_string(arg: Option<String>) -> Option<String>;
        fn rust_create_option_static_str() -> Option<&'static str>;
        fn rust_reflect_option_str(arg: Option<&str>) -> Option<&str>;

        fn rust_reflect_option_vector_rust_type(arg: Option<Vec<u16>>) -> Option<Vec<u16>>;

        fn rust_reflect_option_opaque_rust_type(
            arg: Option<OptTestOpaqueRustType>,
        ) -> Option<OptTestOpaqueRustType>;
        fn rust_reflect_option_opaque_swift_type(
            arg: Option<OptTestOpaqueSwiftType>,
        ) -> Option<OptTestOpaqueSwiftType>;

        fn rust_reflect_option_ref_opaque_rust_type(
            arg: Option<&OptTestOpaqueRustType>,
        ) -> Option<&OptTestOpaqueRustType>;

        fn rust_reflect_option_opaque_rust_copy_type(
            arg: Option<OptTestOpaqueRustCopyType>,
        ) -> Option<OptTestOpaqueRustCopyType>;

        fn rust_reflect_option_generic_opaque_rust_type(
            arg: Option<OptTestGenericOpaqueRustType<u8>>,
        ) -> Option<OptTestGenericOpaqueRustType<u8>>;

        fn rust_reflect_option_generic_opaque_rust_copy_type(
            arg: Option<OptTestGenericOpaqueRustCopyType<u8>>,
        ) -> Option<OptTestGenericOpaqueRustCopyType<u8>>;

        fn rust_reflect_struct_with_option_fields(
            arg: StructWithOptionFields,
        ) -> StructWithOptionFields;

        fn rust_reflect_option_enum_with_no_data(
            arg: Option<OptionEnumWithNoData>,
        ) -> Option<OptionEnumWithNoData>;

        fn rust_reflect_option_struct_with_no_data(
            arg: Option<OptionStruct>,
        ) -> Option<OptionStruct>;

        fn test_rust_calls_swift_option_primitive();
    }

    extern "Swift" {
        type OptTestOpaqueSwiftType;

        fn swift_reflect_option_u8(arg: Option<u8>) -> Option<u8>;
        fn swift_reflect_option_i8(arg: Option<i8>) -> Option<i8>;
        fn swift_reflect_option_u16(arg: Option<u16>) -> Option<u16>;
        fn swift_reflect_option_i16(arg: Option<i16>) -> Option<i16>;
        fn swift_reflect_option_u32(arg: Option<u32>) -> Option<u32>;
        fn swift_reflect_option_i32(arg: Option<i32>) -> Option<i32>;
        fn swift_reflect_option_u64(arg: Option<u64>) -> Option<u64>;
        fn swift_reflect_option_i64(arg: Option<i64>) -> Option<i64>;
        fn swift_reflect_option_usize(arg: Option<usize>) -> Option<usize>;
        fn swift_reflect_option_isize(arg: Option<isize>) -> Option<isize>;
        fn swift_reflect_option_f32(arg: Option<f32>) -> Option<f32>;
        fn swift_reflect_option_f64(arg: Option<f64>) -> Option<f64>;
        fn swift_reflect_option_bool(arg: Option<bool>) -> Option<bool>;

        fn swift_reflect_option_string(arg: Option<String>) -> Option<String>;
        // TODO: Change to `swift_reflect_option_str` once we support Swift returning `-> &str`
        fn swift_arg_option_str(arg: Option<&str>) -> bool;
        // fn swift_reflect_option_str(arg: Option<&str>) -> Option<&str>;
    }

    extern "Rust" {
        #[swift_bridge(Equatable)]
        type FailableInitType;

        #[swift_bridge(init)]
        fn new(success: bool) -> Option<FailableInitType>;
        fn count(&self) -> i32;
    }
}

fn test_rust_calls_swift_option_primitive() {
    assert_eq!(ffi::swift_reflect_option_u8(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_u8(None), None);

    assert_eq!(ffi::swift_reflect_option_i8(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_i8(None), None);

    assert_eq!(ffi::swift_reflect_option_u16(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_u16(None), None);

    assert_eq!(ffi::swift_reflect_option_i16(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_i16(None), None);

    assert_eq!(ffi::swift_reflect_option_u32(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_u32(None), None);

    assert_eq!(ffi::swift_reflect_option_i32(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_i32(None), None);

    assert_eq!(ffi::swift_reflect_option_u64(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_u64(None), None);

    assert_eq!(ffi::swift_reflect_option_i64(Some(55)), Some(55));
    assert_eq!(ffi::swift_reflect_option_i64(None), None);

    assert_eq!(ffi::swift_reflect_option_f32(Some(55.)), Some(55.));
    assert_eq!(ffi::swift_reflect_option_f32(None), None);

    assert_eq!(ffi::swift_reflect_option_f64(Some(55.)), Some(55.));
    assert_eq!(ffi::swift_reflect_option_f64(None), None);

    assert_eq!(ffi::swift_reflect_option_bool(Some(true)), Some(true));
    assert_eq!(ffi::swift_reflect_option_bool(Some(false)), Some(false));
    assert_eq!(ffi::swift_reflect_option_bool(None), None);

    assert_eq!(ffi::swift_reflect_option_string(None), None);
    assert_eq!(
        ffi::swift_reflect_option_string(Some("hello".to_string())),
        Some("hello".to_string())
    );

    // TODO: Change to `swift_reflect_option_str` once we support Swift returning `-> &str`
    assert_eq!(ffi::swift_arg_option_str(None), false);
    assert_eq!(
        ffi::swift_arg_option_str(Some("this is an option str")),
        true
    );
    // assert_eq!(ffi::swift_reflect_option_str(None), None);
    // assert_eq!(ffi::swift_reflect_option_str(Some("a str")), Some("a str"));
}

pub struct OptTestOpaqueRustType {
    field: u8,
}
impl OptTestOpaqueRustType {
    fn new(field: u8) -> Self {
        Self { field }
    }

    fn field(&self) -> u8 {
        self.field
    }
}

pub struct OptTestOpaqueRefRustType {
    field: Option<OptTestOpaqueRustType>,
}

impl OptTestOpaqueRefRustType {
    fn new(field: u8) -> Self {
        Self {
            field: Some(OptTestOpaqueRustType::new(field)),
        }
    }

    fn field_ref(&self) -> Option<&OptTestOpaqueRustType> {
        self.field.as_ref()
    }
}

#[derive(Copy, Clone)]
pub struct OptTestOpaqueRustCopyType {
    #[allow(unused)]
    field: u8,
}
fn new_opaque_rust_copy_type(field: u8) -> OptTestOpaqueRustCopyType {
    OptTestOpaqueRustCopyType { field }
}

pub struct OptTestGenericOpaqueRustType<T> {
    #[allow(unused)]
    field: T,
}
fn new_generic_opaque_rust_type<T>(field: T) -> OptTestGenericOpaqueRustType<T> {
    OptTestGenericOpaqueRustType { field }
}

#[derive(Copy, Clone)]
pub struct OptTestGenericOpaqueRustCopyType<T> {
    #[allow(unused)]
    field: T,
}
fn new_generic_opaque_rust_copy_type<T>(field: T) -> OptTestGenericOpaqueRustCopyType<T> {
    OptTestGenericOpaqueRustCopyType { field }
}

use self::reflect_primitives::*;
#[rustfmt::skip]
mod reflect_primitives {
    pub fn rust_reflect_option_u8(arg: Option<u8>) -> Option<u8> { arg }
    pub fn rust_reflect_option_i8(arg: Option<i8>) -> Option<i8> { arg }
    pub fn rust_reflect_option_u16(arg: Option<u16>) -> Option<u16> { arg }
    pub fn rust_reflect_option_i16(arg: Option<i16>) -> Option<i16> { arg }
    pub fn rust_reflect_option_u32(arg: Option<u32>) -> Option<u32> { arg }
    pub fn rust_reflect_option_i32(arg: Option<i32>) -> Option<i32> { arg }
    pub fn rust_reflect_option_u64(arg: Option<u64>) -> Option<u64> { arg }
    pub fn rust_reflect_option_i64(arg: Option<i64>) -> Option<i64> { arg }
    pub fn rust_reflect_option_usize(arg: Option<usize>) -> Option<usize> { arg }
    pub fn rust_reflect_option_isize(arg: Option<isize>) -> Option<isize> { arg }
    pub fn rust_reflect_option_f32(arg: Option<f32>) -> Option<f32> { arg }
    pub fn rust_reflect_option_f64(arg: Option<f64>) -> Option<f64> { arg }
    pub fn rust_reflect_option_bool(arg: Option<bool>) -> Option<bool> { arg }
}

fn rust_reflect_option_string(arg: Option<String>) -> Option<String> {
    arg
}

fn rust_create_option_static_str() -> Option<&'static str> {
    Some("hello")
}
fn rust_reflect_option_str(arg: Option<&str>) -> Option<&str> {
    arg
}

fn rust_reflect_option_vector_rust_type(arg: Option<Vec<u16>>) -> Option<Vec<u16>> {
    arg
}

fn rust_reflect_option_opaque_rust_type(
    arg: Option<OptTestOpaqueRustType>,
) -> Option<OptTestOpaqueRustType> {
    arg
}

fn rust_reflect_option_ref_opaque_rust_type(
    arg: Option<&OptTestOpaqueRustType>,
) -> Option<&OptTestOpaqueRustType> {
    arg
}
pub fn rust_reflect_option_opaque_swift_type(
    arg: Option<OptTestOpaqueSwiftType>,
) -> Option<OptTestOpaqueSwiftType> {
    arg
}

fn rust_reflect_option_opaque_rust_copy_type(
    arg: Option<OptTestOpaqueRustCopyType>,
) -> Option<OptTestOpaqueRustCopyType> {
    arg
}

fn rust_reflect_option_generic_opaque_rust_type(
    arg: Option<OptTestGenericOpaqueRustType<u8>>,
) -> Option<OptTestGenericOpaqueRustType<u8>> {
    arg
}

fn rust_reflect_option_generic_opaque_rust_copy_type(
    arg: Option<OptTestGenericOpaqueRustCopyType<u8>>,
) -> Option<OptTestGenericOpaqueRustCopyType<u8>> {
    arg
}

fn rust_reflect_struct_with_option_fields(
    arg: ffi::StructWithOptionFields,
) -> ffi::StructWithOptionFields {
    arg
}
fn rust_reflect_option_enum_with_no_data(
    arg: Option<ffi::OptionEnumWithNoData>,
) -> Option<ffi::OptionEnumWithNoData> {
    arg
}

fn rust_reflect_option_struct_with_no_data(
    arg: Option<ffi::OptionStruct>,
) -> Option<ffi::OptionStruct> {
    arg
}

#[derive(PartialEq)]
struct FailableInitType;

impl FailableInitType {
    fn new(success: bool) -> Option<FailableInitType> {
        if success {
            Some(FailableInitType)
        } else {
            None
        }
    }

    fn count(&self) -> i32 {
        132
    }
}
