use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use swift_bridge_build::{generate_package, GeneratePackageConfig};
use swift_bridge_build::ApplePlatform as Platform;

#[test]
fn gen_package() {
    // Build Rust
    if let Err(err) = fs::create_dir("tests/sample-project/generated") {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("{}", err);
        }
    }
    
    let output = Command::new("sh")
        .current_dir("tests/sample-project")
        .arg("build.sh")
        .output()
        .expect("Couldn't execute build script");
    println!("{}", std::str::from_utf8(&*output.stdout).unwrap());
    
    // Generate package
    generate_package(GeneratePackageConfig {
        bridge_dir: &Path::new("tests/sample-project/generated"),
        paths: HashMap::from([
            (Platform::macOS, &Path::new("tests/sample-project/target/x86_64-apple-darwin/debug/libsample_project.a") as &dyn AsRef<Path>),
        ]),
        out_dir: &Path::new("tests/sample-project/MySwiftPackage"),
        package_name: "MySwiftPackage"
    });
    
    // Test package (macOS executable)
    let output = Command::new("swift")
        .current_dir("tests/test-project")
        .arg("run")
        .output()
        .expect("Failed to execute `swift run`");
    
    println!("{}", std::str::from_utf8(&*output.stderr).unwrap());
    assert_eq!("Hello Rust!\n", std::str::from_utf8(&*output.stdout).unwrap());
}
