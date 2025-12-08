# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge) [![crates.io](https://img.shields.io/crates/v/swift-bridge)](https://crates.io/crates/swift-bridge)

> `swift-bridge` facilitates Rust and Swift interop.

`swift-bridge` makes it easy to pass and share high-level types between Rust and Swift,
such as `String`, `Option<T>`,  `Result<T, E>`, `struct`, `class` and more.

It also helps you bridge higher level language features, such as async functions and generics.

Using `swift-bridge` should be safer, more performant and more ergonomic than managing Rust and Swift
FFI by hand.

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
// We use the `swift_bridge::bridge` macro to declare a bridge module.
// Then at build time the `swift-bridge-build` crate is used to generate
// the corresponding Swift and C FFI glue code.
#[swift_bridge::bridge]
mod ffi {
    // Create "transparent" structs where both Rust and Swift can directly access the fields.
    struct AppConfig {
        file_manager: CustomFileManager,
    }

    // Transparent enums are also supported.
    enum UserLookup {
        ById(UserId),
        ByName(String),
    }

    // Export opaque Rust types, functions and methods for Swift to use.
    extern "Rust" {
        type RustApp;

        #[swift_bridge(init)]
        fn new(config: AppConfig) -> RustApp;
        
        fn get_user(&self, lookup: UserLookup) -> Option<&User>;
    }

    extern "Rust" {
        type User;
        type MessageBoard;

        #[swift_bridge(get(&nickname))]
        fn informal_name(self: &User) -> &str;
    }

    // Import opaque Swift classes and functions for Rust to use.
    extern "Swift" {
        type CustomFileManager;
        type CustomIoError;

        // Async functions are supported.
        async fn save_file(self: &CustomFileManager, name: &str, contents: &[u8]) -> Result<(), CustomIoError>;
    }
}

struct User {
    nickname: String
}
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
| Box\<T>                                                         |                                                                  | Not yet implemented                                                                |
| Box<dyn FnOnce(A,B,C) -> D>                                     | (A, B, C) -> D                                                   | Passing from Rust to Swift is supported, but Swift to Rust is not yet implemented. |
| Box<dyn Fn(A,B,C) -> D>                                         | (A, B, C) -> D                                                   | Not yet implemented                                                                |
| Arc\<T>                                                         |                                                                  | Not yet implemented                                                                |
| [T; N]                                                          |                                                                  | Not yet implemented                                                                |
| *const T                                                        | UnsafePointer\<T>                                                |                                                                                    |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         |                                                                                    |
| Option\<T>                                                      | Optional\<T>                                                     |                                                                                    |
| fn x() -> Result\<T, E>                                         | func x() throws -> T                                             |                                                                                    |
| fn x(arg: Result\<T, E>)                                        | func x(arg: RustResult\<T, E>)                                   |                                                                                    |
| (A, B, C, ...)| (A, B, C, ...)
| Have a Rust standard library type in mind?<br /> Open an issue! |                                                                  |                                                                                    |
|                                                                 | Have a Swift standard library type in mind?<br /> Open an issue! |                                                                                    |
<!-- ANCHOR_END: built-in-types-table -->

## Performance

`swift-bridge` aims to be useful in performance critical environments.

None of its generated FFI code uses object serialization, cloning, synchronization or any other form of unnecessary overhead.

## To Test

To run the test suite.

```sh
# Clone the repository
git clone git@github.com:chinedufn/swift-bridge.git
cd swift-bridge

# Run tests
cargo test --all && ./test-swift-rust-integration.sh && ./test-swift-packages.sh 
```

## Contributing

If you're interesting in contributing to `swift-bridge`, check out the [contributor's guide](https://chinedufn.github.io/swift-bridge/contributing/index.html).

After getting familiar with the contribution process, try looking at some of the [good first issues](https://github.com/chinedufn/swift-bridge/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
to see if any peak your interest.

These issues come with step-by-step instructions that should help guide you towards implementing your first patch.

## Minimum Supported Swift Version (MSSV)

`swift-bridge` currently guarantees that the Swift code that it generates will work on Swift `6.0` and later.
This is known the project's "Minimum Supported Swift Version" (MSSV).

`swift-bridge`'s current policy is that the minimum required Swift version can be increased at any time to
any Swift version that is at least one month old.

For instance, if Swift `9.10.11` is released on April 5, 2035, then on May 5, 2035 the `swift-bridge` project is allowed
to begin emitting Swift code that relies on Swift `9.10.11`.

We will increase our support windows when one or both of the following happen:

- We are no longer waiting for Swift features that increase the safety, performance and ergonomics of the Swift code that `swift-bridge` emits.
  - For instance, Swift recently introduced the `~Copyable` protocol, which we plan to use enforce ownership when Swift code uses opaque Rust types.

- The short support window is disrupting projects that use `swift-bridge` today.
  - Please open an issue if our MSSV policy impacts your project

## Acknowledgements

- [cxx](https://github.com/dtolnay/cxx) inspired the idea of using a bridge module to describe the FFI boundary.

---

#### License

_Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE)._
