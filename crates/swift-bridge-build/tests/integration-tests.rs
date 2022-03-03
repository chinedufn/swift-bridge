use std::collections::HashMap;
use std::path::Path;
use swift_bridge_build::{generate_package, GeneratePackageConfig, ApplePlatform};

use swift_bridge_build::ApplePlatform as Platform;

#[test]
fn gen_package() {
    generate_package(GeneratePackageConfig {
        bridge_dir: &Path::new("tests/sample_project/generated"),
        paths: HashMap::from([
            (Platform::iOS, &Path::new("tests/sample_project/target/x86_64-apple-ios/debug/libsample_project.a") as &dyn AsRef<Path>),
            (Platform::macOS, &Path::new("tests/sample_project/target/x86_64-apple-darwin/debug/libsample_project.a") as &dyn AsRef<Path>),
            (Platform::Simulator, &Path::new("tests/sample_project/target/aarch64-apple-ios/debug/libsample_project.a") as &dyn AsRef<Path>),
        ]),
        out_dir: &Path::new("tests/sample_project/package")
    });
}
