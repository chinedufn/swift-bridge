use std::collections::HashMap;
use std::path::Path;

use swift_bridge_build::ApplePlatform as Platform;
use swift_bridge_build::{create_package, CreatePackageConfig};

// TODO: paths
fn main() {
    // Generate package
    create_package(CreatePackageConfig {
        bridge_dir: &Path::new("swift-package-rust-library-fixture/generated"),
        paths: HashMap::from([
            (Platform::MacOS, &Path::new("swift-package-rust-library-fixture/target/x86_64-apple-darwin/debug/libtest_swift_packages.a") as _),
        ]),
        out_dir: &Path::new("swift-package-rust-library-fixture/MySwiftPackage"),
        package_name: "MySwiftPackage"
    });
}
