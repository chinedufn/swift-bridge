use std::path::PathBuf;

fn main() {
    if let Ok(out_dir) = std::env::var("SWIFT_BRIDGE_OUT_DIR") {
        let out_dir = PathBuf::from(out_dir);

        let generated = swift_bridge_build::bridge(vec![
            //
            manifest_dir().join("src/std_bridge/string.rs"),
        ]);
        let generated_swift = generated.concat_swift();
        let generated_c = generated.concat_c();

        let core_swift_out = out_dir.join("SwiftBridgeCore.swift");
        let mut swift = core_swift();
        swift += &generated_swift;

        std::fs::write(core_swift_out, swift).unwrap();

        let core_c_header_out = out_dir.join("SwiftBridgeCore.h");
        let mut c_header = core_c_header().to_string();
        c_header += &generated_c;

        std::fs::write(core_c_header_out, c_header).unwrap();
    }
}

fn core_swift() -> String {
    let mut core_swift = "".to_string();

    core_swift += include_str!("src/std_bridge/string.swift");

    core_swift
}

fn core_c_header() -> &'static str {
    r#"
typedef struct RustStr { uint8_t* const start; uintptr_t len; } RustStr;
"#
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
