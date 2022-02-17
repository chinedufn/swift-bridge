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

Build the project for the desired platforms:

```bash
export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
```

## Creating the XCFramework

Go back to the root of the project and make a new directory `cd ..`.

```bash
mkdir MyFramework && cd $_
```

Copy the generated libraries and the headers to this folder:
```bash
mkdir include
touch include/module.modulemap
cp ../my-rust-lib/generated/SwiftBridgeCore.h ./include
cp ../my-rust-lib/generated/my_rust_lib/my_rust_lib.h ./includ
mkdir ios
cp ../my-rust-lib/target/aarch64-apple-ios/debug/libmy_rust_lib.a ./ios
mkdir macos
cp ../my-rust-lib/target/x86_64-apple-darwin/debug/libmy_rust_lib.a ./macos
mkdir simulator
cp ../my-rust-lib/target/x86_64-apple-ios/debug/libmy_rust_lib.a ./simulator
```

This should result in the follwing folder structure:
```
MyFramework
├── include
│   ├── SwiftBridgeCore.h
│   ├── module.modulemap
│   └── my_rust_lib.h
├── ios
│   └── libmy_rust_lib.a
├── macos
│   └── libmy_rust_lib.a
└── simulator
    └── libmy_rust_lib.a
```

Edit `include/module.modulemap`:

```modulemap
module MyRustLib {
    header "my_rust_lib.h"
    header "SwiftBridgeCore.h"
    export *
}
```

Now it is time to build the xcframework:

```bash
xcodebuild -create-xcframework \
    -library simulator/libmy_rust_lib.a \
    -headers include \
    -library ios/libmy_rust_lib.a \
    -headers include \
    -library macos/libmy_rust_lib.a \
    -headers include \
    -output MyRustLib.xcframework
```

*The order of the `library` tags is important, but we don't currently know why*

## Creating the Swift package

Go back to the root of the project (`cd ..`) and create a new Swift package:

```bash
mkdir MySwiftPackage && cd MySwiftPackage
```

Now either do `mkdir -r Sources/MySwiftPackage` and `touch Package.swift`, or use `swift package init --type library`.

Copy the xcframework and generated swift files to this folder:
```bash
cp -r ../MyFramework/MyRustLib.xcframework ./
cp ../my-rust-lib/generated/SwiftBridgeCore.swift Sources/MySwiftPackage
cp ../my-rust-lib/generated/my_rust_lib/my_rust_lib.swift Sources/MySwiftPackage
```

The folder structure should be:
```
MySwiftPackage
├── Sources
│   └── MySwiftPackage
│       └── SwiftBridgeCore.swift
│       └── my_rust_lib.swift
├── MyRustLib.b.xcframework
└── Package.swift
```

Add the framework as a binary target to `Package.swift`:
```swift
// MySwiftPackage/Package.swift

// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
    name: "MySwiftPackage",
    products: [
        .library(
            name: "MySwiftPackage",
            targets: ["MySwiftPackage"]),
    ],
    dependencies: [

    ],
    targets: [
        .binaryTarget(
            name: "MyRustLib",
            path: "MyRustLib.xcframework"
        ),
        .target(
            name: "MySwiftPackge",
            dependencies: ["MyRustLib"]),
    ]
)
```

We will need to import our rust library in `my_rust_lib.swift` and `SwiftBridgeCore.swift`:

```swift
// MySwiftPackage/Sources/MySwiftPackage/SwiftBridgeCore.swift
import MyRustLib
```

<!--TODO: always make public, or maybe just have an option for making functions public in build.rs?-->
```swift
// MySwiftPackage/Sources/MySwiftPackage/my_rust_lib.swift
import MyRustLib

// Also make the functions here public
public hello_rust() -> RustString {
// ...
```

## Using the Swift Package

We now have a Swift Package which we can include in other projects using the Swift Package Manager.

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
