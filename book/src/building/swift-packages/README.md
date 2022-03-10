# Bundling Rust code as a Swift package

In this chapter we'll walk through bundling your Rust library into a Swift Package.

Note that Swift Packages that contain native libraries only work on Apple hardware. 

You should avoid bundling your Rust code into a Swift Package if you plan to target Linux, Windows or any other non-Apple target.

## Project setup

```bash
mkdir rust-swift-package && cd rust-swift-package
```

### Rust project setup

```bash
cargo new my-rust-lib --lib
cd my_rust_lib
```

```toml
# my-rust-lib/Cargo.toml

[lib]
crate-type = ["staticlib"]

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```

In `src/lib.rs`, add the following:

```rust
// my-rust-lib/src/lib.rs
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn hello_rust() -> String;
    }
}

fn hello_rust() -> String {
    String::from("Hello Rust!")
}
```

Add a new `build.rs` file (`touch build.rs`):
```rust
// my-rust-lib/build.rs

use std::path::PathBuf;
use swift_bridge_build::{GeneratePackageConfig, ApplePlatform};

fn main() {
    let out_dir = PathBuf::from("./generated");
    
    let bridges = vec!["src/lib.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }
    
    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}
```

Create a new folder called *generated* `mkdir generated`.

Build the project for the desired platforms:

```bash
export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
```

We can now take our generated files and turn them into a Swift Package. This can be achieved using  the `API` or the `CLI` to package the bridging code and the Rust libraries into a Swift Package.

#### API

```rust
use std::path::Path;
use std::collections::HashMap;
use swift_bridge_build::{CreatePackageConfig, ApplePlatform};
fn main() {
    swift_bridge_build::create_package(GeneratePackageConfig {
        bridge_dir: &Path::new("./generated"),
        paths: HashMap::from([
            (ApplePlatform::IOS, &"target/x86_64-apple-ios/debug/libmy_rust_lib.a" as _),
            (ApplePlatform::Simulator, &"target/aarch64-apple-ios/debug/libmy_rust_lib.a" as _),
            (ApplePlatform::MacOS, &"target/x86_64-apple-darwin/debug/libmy_rust_lib.a" as _),
        ]),
        out_dir: &Path::new("MySwiftPackage"),
        package_name: "MySwiftPackage"
    });
}
```

#### CLI
*Not yet implemented*

## Using the Swift Package

We now have a Swift Package (in the `MySwiftPackage` directory) which we can include in other projects using the Swift Package Manager.

### Example: MacOS executable
Here is an example of an executable project located in `rust-swift-project/testPackage`.

```swift
// testPackage/Package.swift

// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
    name: "testPackage",
    dependencies: [
        .package(path: "../MySwiftPackage")
    ],
    targets: [
        .executableTarget(
            name: "testPackage",
            dependencies: [
                .product(name: "MySwiftPackage", package: "MySwiftPackage")
            ])
    ]
)
```

```swift
// testPackage/Sources/testPackage/main.swift
import MySwiftPackage

print(hello_rust().toString())
```

```
$ swift run
Hello Rust!
```

### Example: iOS app

To add the package to an iOS app in XCode, go to the target's general panel, click the `+` button in the `Frameworks, Libraries, and Embedded Content` section. Then, click `Add Other` and choose `Add Package Dependency`.

Import and use it in the same way as the executable.
