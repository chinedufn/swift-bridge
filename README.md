# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge) [![crates.io](https://img.shields.io/crates/v/swift-bridge)](https://crates.io/crates/swift-bridge)

> `swift-bridge` facilitates Rust and Swift interop.

`swift-bridge` is a library that lets you pass and share high-level types such as `Option<T>`, `String`,
`Structs` and `Classes` between Rust and Swift.

It also lets you bridge higher level language features between Rust and Swift, such as async functions and generics.

## Installation

```toml
# In your Cargo.toml

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```

## Book

You can find information about using Rust and Swift together in [The `swift-bridge` Book](https://chinedufn.github.io/swift-bridge).

## Quick Peek

You use `swift-bridge` by declaring the types and functions that you want to import and export
in a "bridge module", and then annotating that bridge module with the `#[swift_bridge::bridge]`
macro.

Then, at build time, you use either the `swift-bridge-build` API or the `swift-bridge-cli` CLI to
parse your annotated bridge modules and generate the `Swift` and `C` side of the FFI layer.

Here's a quick peek at how you might describe an FFI boundary between Swift and Rust using a bridge module.

<!-- ANCHOR: bridge-module-example -->
```rust
// Use the `swift_bridge::bridge` macro to declare a bridge module that
// `swift-bridge-build` will parse at build time in order to generate
// the necessary Swift and C FFI glue code.
#[swift_bridge::bridge]
mod ffi {
    // Create shared structs where both Rust and Swift can directly access the fields.
    struct AppConfig {
        file_manager: CustomFileManager,
    }

    // Shared enums are also supported
    enum UserLookup {
        ById(UserId),
        ByName(String),
    }

    // Export Rust types, functions and methods for Swift to use.
    extern "Rust" {
        type RustApp;

        #[swift_bridge(init)]
        fn new(config: AppConfig);
        
        fn insert_user(&mut self, user_id: UserId, user: User);
        fn get_user(&self, lookup: UserLookup) -> Option<&User>;
    }

    extern "Rust" {
        type User;

        #[swift_bridge(Copy(4))]
        type UserId;

        #[swift_bridge(init)]
        fn new(user_id: UserId, name: String, email: Option<String>) -> User;
    }

    // Import Swift classes and functions for Rust to use.
    extern "Swift" {
        type CustomFileManager;
        fn save_file(&self, name: &str, contents: &[u8]);
    }
}

#[derive(Copy)]
struct UserId(u32);
```
<!-- ANCHOR_END: bridge-module-example -->

## Quick Start

The `swift-bridge` repository contains [example applications](examples) that you use to quickly try out the library,
or as a starting point for your own `Swift` + `Rust` based application.

For example, here's how to run the [`codegen-visualizer`](examples/codegen-visualizer) example project locally.

```sh
git clone https://github.com/chinedufn/swift-bridge
cd swift-bridge/examples/codegen-visualizer

open CodegenVisualizer/CodegenVisualizer.xcodeproj
# *** Click the "Run" button at the top left of Xcode ***
```

---

You can find information about using Rust and Swift together in [The `swift-bridge` Book](https://chinedufn.github.io/swift-bridge).

## Built-In Types

In addition to allowing you to share your own custom structs, enums and classes between Rust and Swift,
`swift-bridge` comes with support for a number of Rust and Swift standard library types.

<!-- ANCHOR: built-in-types-table -->
| name in Rust                                                    | name in Swift                                                    | notes                                                                              |
| ---                                                             | ---                                                              | ---                                                                                |
| u8, i8, u16, i16... etc                                         | UInt8, Int8, UInt16, Int16 ... etc                               |                                                                                    |
| bool                                                            | Bool                                                             |                                                                                    |
| String, &String, &mut String                                    | RustString, RustStringRef, RustStringRefMut                      |                                                                                    |
| &str                                                            | RustStr                                                          |                                                                                    |
| Vec\<T>                                                         | RustVec\<T>                                                      |                                                                                    |
| SwiftArray\<T>                                                  | Array\<T>                                                        | Not yet implemented                                                                |
| &[T]                                                            |                                                                  | Not yet implemented                                                                |
| &mut [T]                                                        |                                                                  | Not yet implemented                                                                |
| Box<T>                                                          |                                                                  | Not yet implemented                                                                |
| Box<dyn FnOnce(A,B,C)> -> D>                                    | (A, B, C) -> D                                                   | Passing from Rust to Swift is supported, but Swift to Rust is not yet implemented. |
| Box<dyn Fn(A,B,C)> -> D>                                        | (A, B, C) -> D                                                   | Not yet implemented                                                                |
| [T; N]                                                          |                                                                  | Not yet implemented                                                                |
| *const T                                                        | UnsafePointer\<T>                                                |                                                                                    |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         |                                                                                    |
| Option\<T>                                                      | Optional\<T>                                                     |                                                                                    |
| Result\<T>                                                      |                                                                  | Not yet implemented                                                                |
| Have a Rust standard library type in mind?<br /> Open an issue! |                                                                  |                                                                                    |
|                                                                 | Have a Swift standard library type in mind?<br /> Open an issue! |                                                                                    |
<!-- ANCHOR_END: built-in-types-table -->

## To Test

To run the test suite.

```sh
# Clone the repository
git clone git@github.com:chinedufn/swift-bridge.git
cd swift-bridge

# Run tests
cargo test --all && ./test-integration.sh
```

---

#### License

_Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE)._
