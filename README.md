# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge)

> `swift-bridge` facilitates Rust and Swift interop.

`swift-bridge` is a library that lets you pass and share high-level types such as `Option<T>`, `String`, `Struct` and `Class` between Rust and Swift.

You declare the types and functions that you want to import and export using "bridge modules", and
then run `swift-bridge-build` at build time to generate the `Swift` and `C` FFI layer to make it all work.

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

For example, here's how to run the [`ios-rust-analyzer`](examples/ios-rust-analyzer) example project locally.

```sh
git clone https://github.com/chinedufn/swift-bridge
cd swift-bridge/examples/ios-rust-analyzer

open IosRustAnalyzer/IosRustAnalyzer.xcodeproj
# *** Click the "Run" button at the top left of Xcode ***
```

---

You can find information about using Rust and Swift together in [The `swift-bridge` book](https://chinedufn.github.io/swift-bridge).

## Built-In Types

In addition to allowing you to share your own custom structs, enums and classes between Rust and Swift,
`swift-bridge` comes with support for a number of Rust and Swift standard library types.

<!-- ANCHOR: built-in-types-table -->
| name in Rust                                                    | name in Swift                                                    | notes               |
| ---                                                             | ---                                                              | ---                 |
| u8, i8, u16, i16... etc                                         | UInt8, Int8, UInt16, Int16 ... etc                               |                     |
| bool                                                            | Bool                                                             |                     |
| String, &String, &mut String                                    | RustString, RustStringRef, RustStringRefMut                      |                     |
| &str                                                            | RustStr                                                          |                     |
| Vec\<T>                                                         | RustVec\<T>                                                      |                     |
| SwiftArray\<T>                                                  | Array\<T>                                                        | Not yet implemented |
| &[T]                                                            |                                                                  | Not yet implemented |
| &mut [T]                                                        |                                                                  | Not yet implemented |
| Box<T>                                                          |                                                                  | Not yet implemented |
| [T; N]                                                          |                                                                  | Not yet implemented |
| *const T                                                        | UnsafePointer\<T>                                                |                     |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         |                     |
| Option\<T>                                                      | Optional\<T>                                                     |                     |
| Result\<T>                                                      |                                                                  | Not yet implemented |
| Have a Rust standard library type in mind?<br /> Open an issue! |                                                                  |                     |
|                                                                 | Have a Swift standard library type in mind?<br /> Open an issue! |                     |
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

## Status

I'm using `swift-bridge` in a business-critical production application.

Here and there I'll run into something that I want to be able to bridge that is not yet supported,
but that's becoming less and less frequent over time.

If you're alright with pre-1.0 software you should feel free to use `swift-bridge` in your application.
Breaking API changes should be infrequent since our API surface area is fairly small.

If you run into an unsupported use case, please open an issue.

---

#### License

_Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE)._
