# Creating Swift Packages

In this chapter we'll walk through bundling your Rust library into a Swift Package.

> Swift Packages that contain binary dependencies are only available on Apple platforms.
>
> You cannot bundle your Rust code into a Swift Package if you plan to target Linux,
> Windows or any other non-Apple target.
>
> Instead, use a building approach from one of the other [building chapters](../README.md).

## Project setup

```bash
mkdir my-rust-lib && cd my-rust-lib
```

```toml
# Cargo.toml

[lib]
crate-type = ["staticlib"]

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```

In `src/lib.rs`, add the following:

```rust
// src/lib.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn hello_rust() -> String;
    }
}

fn hello_rust() -> String {
    String::from("Hello from Rust!")
}
```

Create a new `build.rs` file with the following contents:

```sh
touch build.rs
```

```rust
// build.rs

use std::path::PathBuf;

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

Create a new bash script for building our Rust native libraries along with a folder
that we'll write our parsed bridges too.

```
touch build-rust.sh
chmod +x build-rust.sh
mkdir generated
```

```bash
# build-rust.sh

#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"
# Build the project for the desired platforms:
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-darwin
mkdir -p ./target/universal-macos/debug

lipo \
    ./target/aarch64-apple-darwin/debug/libmy_rust_lib.a \
    ./target/x86_64-apple-darwin/debug/libmy_rust_lib.a -create -output \
    ./target/universal-macos/debug/libmy_rust_lib.a

cargo build --target aarch64-apple-ios

cargo build --target x86_64-apple-ios
cargo build --target aarch64-apple-ios-sim
mkdir -p ./target/universal-ios/debug

lipo \
    ./target/aarch64-apple-ios-sim/debug/libmy_rust_lib.a \
    ./target/x86_64-apple-ios/debug/libmy_rust_lib.a -create -output \
    ./target/universal-ios/debug/libmy_rust_lib.a
```

Install Rust toolchains for the desired platforms:

```bash
rustup target add x86_64-apple-darwin aarch64-apple-darwin aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

Run the script to build our Rust libraries:

```sh
./build-rust.sh
```

We can now use the `API` or the `CLI` to package the generated bridging code and the Rust libraries into a Swift Package.

#### API

Here's an example of using the API to package up our generated bridging code and our Rust libraries into a Swift Package.

```rust
use std::path::PathBuf;
use std::collections::HashMap;
use swift_bridge_build::{CreatePackageConfig, ApplePlatform};
fn main() {
    swift_bridge_build::create_package(CreatePackageConfig {
        bridge_dir: PathBuf::from("./generated"),
        paths: HashMap::from([
            (ApplePlatform::IOS, "target/aarch64-apple-ios/debug/libmy_rust_lib.a".into()),
            (ApplePlatform::Simulator, "target/universal-ios/debug/libmy_rust_lib.a".into()),
            (ApplePlatform::MacOS, "target/universal-macos/debug/libmy_rust_lib.a".into()),
        ]),
        out_dir: PathBuf::from("MySwiftPackage"),
        package_name: PathBuf::from("MySwiftPackage")
    });
}
```

#### CLI

You can use the `swift-bridge` CLI's `create-package` command in order to create a Swift Package.

First, install the CLI.

```bash
cargo install -f swift-bridge-cli
swift-bridge-cli --help
```

Then, run the following to package up your generated bridges and your Rust libraries into a Swift Package.

```bash
swift-bridge-cli create-package \
  --bridges-dir ./generated \
  --out-dir MySwiftPackage \
  --ios target/aarch64-apple-ios/debug/libmy_rust_lib.a \
  --simulator target/universal-ios/debug/libmy_rust_lib.a \
  --macos target/universal-macos/debug/libmy_rust_lib.a \
  --name MySwiftPackage
```

## Using the Swift Package

We now have a Swift Package (in the `MySwiftPackage` directory) which we can include in other projects using the Swift Package Manager.

### Using the package in an Xcode project

To add the package to an iOS app in XCode, make sure you are in a `.xcworkspace`, and add the package to the project: go to the project's package dependencies panel, click on `+` -> `Add Local` -> Select the `MySwiftPackage` directory.

Then, add go to the target's general panel and click the `+` button in the `Frameworks, Libraries, and Embedded Content` section. Then select Workspace -> MySwiftPackage -> MySwiftPackage.

Import and use it in the same way as the executable.

### Using the package in an executable Swift project

Here is an example of an executable Swift project that depends on our newly created `MySwiftPackage`.

```
mkdir SwiftProject
touch SwiftProject/Package.swift
mkdir -p SwiftProject/Sources/SwiftProject
touch SwiftProject/Sources/SwiftProject/main.swift
```

Add these contents to `SwiftProject/Package.swift`.

```swift
// SwiftProject/Package.swift

// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
    name: "SwiftProject",
    dependencies: [
        .package(path: "../MySwiftPackage")
    ],
    targets: [
        .executableTarget(
            name: "SwiftProject",
            dependencies: [
                .product(name: "MySwiftPackage", package: "MySwiftPackage")
            ])
    ]
)
```

And then add this to our `SwiftProject/Sources/SwiftProject/main.swift` file.

```swift
// SwiftProject/Sources/SwiftProject/main.swift

import MySwiftPackage

print(hello_rust().toString())
```

And now you can run your Swift project that depends on your Rust based Swift Package:

```
cd SwiftProject
swift run
# You should see "Hello from Rust!" in your terminal.
```
