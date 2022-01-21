/// We declare a few shared structs that are renamed using the `swift_name` attribute.
/// We then use them as function arg and return types.
///
/// Related: crates/swift-bridge-ir/src/codegen/codegen_tests/shared_struct_codegen_tests.rs
///   - shared_struct_swift_name_attribute
#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_name = "StructRename1")]
    struct StructName1;

    #[swift_bridge(swift_name = "StructRename2", swift_repr = "struct")]
    struct StructName2 {
        field: u8,
    }

    #[swift_bridge(swift_name = "StructRename3", swift_repr = "struct")]
    struct StructName3(u8);

    extern "Rust" {
        fn extern_rust_struct_rename_1(arg: StructName1) -> StructName1;
        fn extern_rust_struct_rename_2(arg: StructName2) -> StructName2;
        fn extern_rust_struct_rename_3(arg: StructName3) -> StructName3;
    }

    extern "Swift" {
        fn extern_swift_struct_rename_1(arg: StructName1) -> StructName1;
        fn extern_swift_struct_rename_2(arg: StructName2) -> StructName2;
        fn extern_swift_struct_rename_3(arg: StructName3) -> StructName3;
    }
}

use ffi::{StructName1, StructName2, StructName3};

fn extern_rust_struct_rename_1(arg: StructName1) -> StructName1 {
    arg
}
fn extern_rust_struct_rename_2(arg: StructName2) -> StructName2 {
    arg
}
fn extern_rust_struct_rename_3(arg: StructName3) -> StructName3 {
    arg
}
