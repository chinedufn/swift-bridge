use std::path::PathBuf;

fn main() {
    let out_dir = "../../SwiftRustIntegrationTestRunner/Generated";
    let out_dir = PathBuf::from(out_dir);

    let bridges = vec![
        "src/expose_opaque_rust_type.rs",
        "src/import_opaque_swift_class.rs",
        "src/bool.rs",
        "src/option.rs",
        "src/pointer.rs",
        "src/string.rs",
        "src/vec.rs",
        "src/slice.rs",
        "src/shared_types/shared_struct.rs",
        "src/shared_types/shared_enum.rs",
        "src/rust_function_uses_opaque_swift_type.rs",
        "src/swift_function_uses_opaque_rust_type.rs",
        "src/conditional_compilation.rs",
        "src/opaque_type_attributes/already_declared.rs",
        "src/function_attributes/identifiable.rs",
        "src/struct_attributes/already_declared.rs",
        "src/struct_attributes/swift_name.rs",
    ];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}
