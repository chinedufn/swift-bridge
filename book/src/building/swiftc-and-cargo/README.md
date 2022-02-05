# swiftc + Cargo

One approach to building Rust and Swift is to use `Cargo` to compile your Rust code and `swiftc` to compile your Swift code.

In this case, either `Cargo` needs to link to a compiled Swift native library, or `swiftc` needs to link to a compiled Rust
native library.

We'll use an example project to explore both of these approaches.

## Project Setup

```
cargo new --lib swift-and-rust
cd swift-and-rust
```

```toml
# Cargo.toml

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```

```rust
// src/main.rs
fn main() {
    swift_and_rust::print_hello_swift();
}
```

```rust
// src/lib.rs

pub use ffi::print_hello_swift;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn print_hello_rust();
    }

    extern "Swift" {
        fn print_hello_swift();
    }
}

fn print_hello_rust() {
    println!("Hello from Rust")
}
```


```swift
// main.swift
run()
```

```swift
// lib.swift
func run() {
    print_hello_rust()
}
```

```sh
# briding-header.h

#ifndef BridgingHeader_h
#define BridgingHeader_h

#include "./generated/SwiftBridgeCore.h"
#include "./generated/swift-and-rust/swift-and-rust.h"

#endif /* BridgingHeader_h */
```

```sh
mkdir generated
```

## Swift links to a Rust native library

Add the following to your Cargo.toml

```toml
# Cargo.toml

[lib]
crate-type = ["staticlib"]
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

```sh
# build-swiftc-links-rust.sh

#!/bin/bash
set -e

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"

cargo build --target x86_64-apple-darwin
swiftc -L target/x86_64-apple-darwin/debug/ -lswift_and_rust -import-objc-header bridging-header.h \
  main.swift lib.swift ./generated/swift-and-rust/swift-and-rust.swift
```

```sh
./build-swiftc-links-rust.sh
./main
# The output should be "Hello from Rust"
```

## Rust links to a Swift native library

Unlike the when we had `swiftc` in the Rust code, you do not need to set the `crate-type`
when you have `Cargo` linking in the Swift code.

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

    println!("rustc-link-lib=static=swiftc_link_rust");
    println!("rustc-link-search=./");
}
```

```sh
# build-swift-static-lib.sh

#!/bin/bash
set -e

# The swift-bridge CLI does not exist yet. Open an issue if you need to use
# this approach and I'll happily whip up the CLI.
swift-bridge -f src/lib.rs > generated

swiftc -emit-library -static -module-name my_swift -import-objc-header bridging-header.h \
  lib.swift ./generated/swift-and-rust/swift-and-rust.swift
```

```sh
chmod +x build-swift-static-lib.sh
./build-swift-static-lib.sh

cargo build
./target/debug/swift_and_rust
# The output should be "Hello from Swift"
```
