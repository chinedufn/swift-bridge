/// We declare an enum and rename it using the `swift_name` attribute.
/// We then use them as function arg and return types.
///
/// Related: crates/swift-bridge-ir/src/codegen/codegen_tests/transparent_enum_codegen_tests.rs
///   - shared_enum_swift_name_attribute
#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_name = "EnumRename")]
    enum EnumName {
        Variant1,
        Variant2,
    }

    extern "Rust" {
        fn extern_rust_enum_rename(arg: EnumName) -> EnumName;
    }

    extern "Swift" {
        fn extern_swift_enum_rename(arg: EnumName) -> EnumName;
    }
}

use ffi::EnumName;

fn extern_rust_enum_rename(arg: EnumName) -> EnumName {
    arg
}
