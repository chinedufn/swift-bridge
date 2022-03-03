use std::collections::HashMap;
use std::path::Path;
use swift_bridge_build::{generate_package, GeneratePackageConfig, PackagePlatform};

use swift_bridge_build::PackagePlatform as Platform;

#[test]
fn gen_package() {
    generate_package(GeneratePackageConfig {
        bridge_dir: &Path::new("tests/sample_project/generated"),
        paths: HashMap::from([
            (Platform::iOS, Path::new("tests/sample_project/target/"))
        ]),
        out_dir: &Path::new("tests/sample_project/package")
    })
}
