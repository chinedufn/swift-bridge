fn main() {
    let out_dir = "./SwiftUIExample/Generated/";

    let bridges = vec!["src/bridge.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}
