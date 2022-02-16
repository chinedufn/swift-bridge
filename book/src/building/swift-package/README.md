# Bundling rust code as a Swift package

It is possible to bundle a rust library into a Swift package. This solution only works when targetting Apple platforms, though.

## Project setup

```bash
mkdir swift_rust_project && cd $_
```

### Rust project setup

```bash
cargo new my_rust_lib --lib
cd my_rust_lib
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

In `lib.rs`, add the following:

```rust
// src/lib.rs
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn hello_rust();
    }
}

fn hello_rust() -> String {
    String::from("Hello Rust!")
}
```

Add a new `build.rs` file:
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

Build the project for the desired platforms:

```bash
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
```

## Creating the XCFramework

Go back to the root of the project and make a new directory.

```bash
mkdir MyFramework && cd $_
```

Copy the generated libraries and the headers to this folder:

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

Add `include/module.modulemap`:

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
    -library simulator/librust.a \
    -headers include \
    -library ios/librust.a \
    -headers include \
    -library macos/librust.a \
    -headers include \
    -output MyRustLib.xcframework
```

*The order of the `library` tags is important*

## Creating the Swift package

Go back to the root of the project and create a new Swift package, copy the xcframework and generated swift files to it:

```
MySwiftPackage
├── Sources
│   └── MySwiftPackage
│       └── SwiftBridgeCore.swift
│       └── my_rust_lib.swift
├── MyRustLib.b.xcframework
└── Package.swift
```

Add the framework as a binary target
```swift
// Package.swift

let package = Package(
    name: "MySwiftPackage",
    products: [
        .library(
            name: "MySwiftPackge",
            targets: ["MySwiftPackge"]),
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

We need to make a few adjustments to the `.swift` files:

```swift
// Sources/MySwiftPackage/SwiftBridgeCore.swift
import MyRustLib

// ...
```

```swift
// Sources/MySwiftPackage/my_rust_lib.swift
import MyRustLib
```

We can choose to either make the functions and classes in `SwiftBridgeCore.swift` public, or to convert the return types of our functions to types recognized by swift, so in this case, do either of the following: 

```swift
// Sources/MySwiftPackage/SwiftBridgeCore.swift
// ...
// Make `toString()` public
extension RustString {
    public func toString() -> String {
        let str = self.as_str()
        let string = str.toString()

        return string
    }
}
// ...
```

or 

```swift
// Sources/MySwiftPackage/my_rust_lib.swift
// ...
// Change return type to `String` and call `.toString`
public func hello_rust() -> String {
    RustString(ptr: __swift_bridge__$hello_rust()).toString()
}
```

## Using the Swift Package

We now have a Swift Package which we can include in other projects using the Swift Package Manager.

### Example: MacOS executable
Here is an example of an executable project located in `swift_rust_project/testPackage`.

```swift
// Package.swift
let package = Package(
    name: "testPackage",
    dependencies: [
        .package(path: "../package")
    ],
    targets: [
        .executableTarget(
            name: "testPackage",
            dependencies: [
                .product(name: "package", package: "package")
            ])
    ]
)
```

```swift
import MySwiftPackage

print(hello_rust().toString())
// Or, if you converted the return type of the function to a `String`:
print(hello_rust())
```

**Output**
```
Hello Rust!
```

### Example: iOS app

To add the package to an iOS app in XCode, go to the target's general panel, click the `+` button in the `Frameworks, Libraries, and Embedded Content` section. Then, click `Add Other` and choose `Add Package Dependency`.

Import and use it in the same way as the executable.
