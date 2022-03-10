use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use swift_bridge_build::{create_package, CreatePackageConfig};
use swift_bridge_build::ApplePlatform as Platform;

#[test]
fn gen_package() {
    // Delete previous files/folders
    let _ = fs::remove_dir_all("tests/test-swift-packages/generated");
    let _ = fs::remove_dir_all("tests/test-swift-packages/MySwiftPackage");
    let _ = fs::remove_dir_all("tests/test-project/.build");
    
    // Build Rust
    if let Err(err) = fs::create_dir("tests/test-swift-packages/generated") {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("{}", err);
        }
    }
    
    let output = Command::new("sh")
        .current_dir("tests/test-swift-packages")
        .arg("build.sh")
        .output()
        .expect("Couldn't execute build script");
    println!("==Build Script==\n{}", std::str::from_utf8(&*output.stdout).unwrap());
    println!("{}", std::str::from_utf8(&*output.stderr).unwrap());
    
    // Generate package
    create_package(CreatePackageConfig {
        bridge_dir: &Path::new("tests/test-swift-packages/generated"),
        paths: HashMap::from([
            (Platform::MacOS, &Path::new("tests/test-swift-packages/target/x86_64-apple-darwin/debug/libtest_swift_packages.a") as _),
        ]),
        out_dir: &Path::new("tests/test-swift-packages/MySwiftPackage"),
        package_name: "MySwiftPackage"
    });
    
    // Test package (macOS executable)
    let output = Command::new("swift")
        .current_dir("tests/test-project")
        .arg("run")
        .output()
        .expect("Failed to execute `swift run`");
    
    println!("{}", std::str::from_utf8(&*output.stderr).unwrap());
    assert_eq!("Hello, From Rust!\n", std::str::from_utf8(&*output.stdout).unwrap());
}
