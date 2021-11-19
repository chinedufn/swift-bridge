use std::path::PathBuf;

fn main() {
    if let Ok(out_dir) = std::env::var("SWIFT_BRIDGE_OUT_DIR") {
        let out_dir = PathBuf::from(out_dir);

        swift_bridge_build::parse_bridges(
            vec![
                manifest_dir().join("src/std_bridge/string.rs"),
                //
            ],
            &out_dir,
        );

        let core_out = out_dir.join("SwiftBridgeCore.swift");
        std::fs::write(core_out, core_swift()).unwrap();
    }
}

fn core_swift() -> String {
    let mut core_swift = "".to_string();

    core_swift += include_str!("src/std_bridge/string.swift");

    core_swift
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
