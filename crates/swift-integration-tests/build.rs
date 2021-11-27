use std::path::PathBuf;

fn main() {
    let out_dir = "../../SwiftRustIntegrationTestRunner/Generated";
    let out_dir = PathBuf::from(out_dir);

    let bridges = vec![
        "src/expose_opaque_rust_struct.rs",
        "src/import_opaque_swift_class.rs",
        "src/bool.rs",
        "src/option.rs",
        "src/pointer.rs",
        // TODO: Write the test
        // "src/shared_types.rs",
        "src/string.rs",
        "src/vec.rs",
        "src/rust_function_returns_opaque_swift_type.rs",
    ];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}
