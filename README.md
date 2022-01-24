# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge)

> `swift-bridge` facilitates Rust and Swift interop.

## Book

You can find information about using Rust and Swift together in [The `swift-bridge` book](https://chinedufn.github.io/swift-bridge).

## Quick Start

The `swift-bridge` repository contains [example applications](examples) that you use to quickly try out the library,
or as a starting point for your own `Swift` + `Rust` based application.

For example, here's how to run the [`ios-rust-analyzer`](examples/ios-rust-analyzer) example project locally.

```
git clone https://github.com/chinedufn/swift-bridge
cd swift-bridge/examples/ios-rust-analyzer

open IosRustAnalyzer/IosRustAnalyzer.xcodeproj
# Click to "Run" button at the top left of Xcode
```

---

Alternatively, you can use the [tutorial](https://chinedufn.github.io/swift-bridge/tutorial/running-rust-analyzer-on-an-iphone/index.html) to learn how to create a `Swift` + `Rust` Xcode project from scratch.

## Quick Peek

Share types and functions between Swift and Rust.

```rust
// Rust

// You write the type signatures of your FFI boundary in Rust,
// which `swift-bridge` then uses to generate the FFI layer.
#[swift_bridge::bridge]
mod ffi {
    // Create structs that both Swift and Rust can use.
    #[swift_bridge(swift_repr = "struct")]
    struct Comparison {
        summary: Option<String>,
    }

    // Export Rust types and functions for Swift to use.
    extern "Rust" {
        type ARustStack;

        #[swift_bridge(init)]
        fn new() -> ARustStack;

        fn push(&mut self, val: String);
        fn pop(&mut self) -> Option<String>;
    }

    // Import Swift types and functions for Rust to use.
    extern "Swift" {
        type ASwiftGraph;

        fn compare_graphs(g1: &ASwiftGraph, g2: &ASwiftGraph) -> Comparison;
    }
}

struct ARustStack {
    stack: Vec<String>
}
impl ARustStack {
    fn new() -> ARustStack { /* ... */ }
    fn push(&mut self, val: String) { /* ... */ }
    fn pop(&mut self) -> Option<String> { /* ... */ }
}
```

```swift
// Swift

let stack = ARustStack()
stack.push("Hello, hello.")
let hello = stack.pop()!

class ASwiftGraph {
    // ...
}

func compare_graphs(g1: &ASwiftGraph, g2: &ASwiftGraph) -> Comparison {
    // ...
    return Comparison(summary: "Things went well.")
}
```

## Installation

```toml
# In your Cargo.toml

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```

## Built-In Types

In addition to allowing you to share your own custom types between Rust and Swift,
`swift_bridge` comes with support for a number of Rust and Swift standard library types.

| name in Rust                                                    | name in Swift                                                    | notes               |
| ---                                                             | ---                                                              | ---                 |
| u8, i8, u16, i16... etc                                         | UInt8, Int8, UInt16, Int16 ... etc                               |                     |
| bool                                                            | Bool                                                             |                     |
| String, &String, &mut String                                    | RustString, RustStringRef, RustStringRefMut                      |                     |
| &str                                                            | RustStr                                                          |                     |
| Vec\<T>                                                         | RustVec\<T>                                                      |                     |
| SwiftArray\<T>                                                  | Array\<T>                                                        | Not yet implemented |
| &[T]                                                            | UnsafeBufferPointer\<T>                                          |                     |
| &mut [T]                                                        | UnsafeMutableBufferPointer\<T>                                   | Not yet implemented |
| SwiftString                                                     | String                                                           |                     |
| Box<T>                                                          |                                                                  | Not yet implemented |
| [T; N]                                                          |                                                                  | Not yet implemented |
| *const T                                                        | UnsafePointer\<T>                                                |                     |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         |                     |
| Option\<T>                                                      | Optional\<T>                                                     |                     |
| Result\<T>                                                      |                                                                  | Not yet implemented |
| Have a Rust standard library type in mind?<br /> Open an issue! |                                                                  |                     |
|                                                                 | Have a Swift standard library type in mind?<br /> Open an issue! |                     |

## To Test

To run the test suite.

```sh
# Clone the repository
git clone git@github.com:chinedufn/swift-bridge.git
cd swift-bridge

# Run tests
cargo test --all && ./test-integration.sh
```

## Early Stages

Bridging Rust and Swift is fairly unexplored territory, so it will take some experimentation in order to
figure out the right API and code generation.

In these early days I'm looking for feedback from bleeding-edge users in order to continue to improve the
API and the generated code.

I can especially use feedback from people with Swift experience, since I don't have much.

---

The `0.1.x` versions will not follow semver.

We'll maintain semver from `0.2` and onwards.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
