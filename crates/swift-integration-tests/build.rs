use std::path::PathBuf;

fn main() {
    let out_dir = concat!("/../../SwiftRustIntegrationTestRunner/Generated");
    let out_dir = PathBuf::from(out_dir);

    let bridges = vec![
        "./src/expose_opaque_rust_struct.rs",
        "./src/import_opaque_swift_class.rs",
        "./src/bool.rs",
        "./src/option.rs",
        "./src/string.rs",
        "./src/vec.rs",
    ];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }

    swift_bridge_build::parse_bridges(bridges).write_all_concatenated(out_dir)
}
