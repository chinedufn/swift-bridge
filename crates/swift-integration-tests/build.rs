use std::path::PathBuf;

fn main() {
    let out_dir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../SwiftRustIntegrationTestRunner/Generated"
    );
    let out_dir = PathBuf::from(out_dir);

    swift_bridge_build::parse_bridges(vec![
        "./src/expose_opaque_rust_struct.rs",
        "./src/import_opaque_swift_class.rs",
        "./src/bool.rs",
        "./src/string.rs",
        "./src/vec.rs",
    ])
    .write_all_concatenated(out_dir, "swift-integration-tests")
}
