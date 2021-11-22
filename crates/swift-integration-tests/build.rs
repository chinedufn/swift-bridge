use std::path::PathBuf;

fn main() {
    let out_dir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../SwiftRustIntegrationTestRunner/Generated"
    );
    let out_dir = PathBuf::from(out_dir);

    let bridges = vec![
        "./src/expose_opaque_rust_struct.rs",
        "./src/import_opaque_swift_class.rs",
        "./src/bool.rs",
        "./src/option.rs",
        "./src/string.rs",
        "./src/vec.rs",
    ];
    for bridge in bridges.iter() {
        let path = PathBuf::from(bridge);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, "swift-integration-tests")
}
