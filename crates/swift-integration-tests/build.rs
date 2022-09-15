use std::path::PathBuf;

fn main() {
    let out_dir = "../../SwiftRustIntegrationTestRunner/Generated";
    let out_dir = PathBuf::from(out_dir);

    let mut bridges = vec![];
    read_files_recursive(PathBuf::from("src"), &mut bridges);

    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}

fn read_files_recursive(dir: PathBuf, files: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            read_files_recursive(path, files);
        } else {
            files.push(path)
        }
    }
}
