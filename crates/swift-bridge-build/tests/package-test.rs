use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use swift_bridge_build::{generate_package, GeneratePackageConfig};
use swift_bridge_build::ApplePlatform as Platform;

#[test]
fn gen_package() {
    // Build Rust
    Command::new("sh")
        .current_dir("tests/sample_project")
        .arg("build.sh")
        .spawn()
        .expect("Couldn't execute build script");
    // Generate package
    generate_package(GeneratePackageConfig {
        bridge_dir: &Path::new("tests/sample_project/generated"),
        paths: HashMap::from([
            (Platform::iOS, &Path::new("tests/sample_project/target/x86_64-apple-ios/debug/libsample_project.a") as &dyn AsRef<Path>),
            (Platform::macOS, &Path::new("tests/sample_project/target/x86_64-apple-darwin/debug/libsample_project.a") as &dyn AsRef<Path>),
            (Platform::Simulator, &Path::new("tests/sample_project/target/aarch64-apple-ios/debug/libsample_project.a") as &dyn AsRef<Path>),
        ]),
        out_dir: &Path::new("tests/sample_project/MySwiftPackage"),
        package_name: "MySwiftPackage"
    });
    
    // Test package (macOS executable)
    let output = Command::new("swift")
        .current_dir("tests/test_project")
        .arg("run")
        .output()
        .expect("Failed to execute `swift run`");
    
    println!("{}", std::str::from_utf8(&*output.stderr).unwrap());
    assert_eq!("Hello Rust!\n", std::str::from_utf8(&*output.stdout).unwrap());
}
