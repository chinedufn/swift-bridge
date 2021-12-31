# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge)

> Call Rust from Swift and vice versa. 

`swift-bridge` makes it possible to pass complex types between Rust and Swift without needing to write any FFI glue code yourself.

You write the type signatures of your FFI boundary in Rust, and then `swift-bridge` uses that to auto-generate the FFI layer.

The generated FFI layer is both type-safe and memory-safe, allowing you to confidently communicate between languages.

---

_`swift-bridge` takes inspiration from the bridge module idea pioneered by [cxx](https://github.com/dtolnay/cxx)._

## Current Status

`swift-bridge` works, but it's still early so it's possible to run into edge cases where the generated code is subtly not-memory-safe.

All of these cases are addressable, and once they're all tackled using code generated from `swift-bridge` should be entirely memory safe
on both the Swift and Rust side.

So, `swift-bridge` is not yet production ready.

Right now I'm looking for feedback from bleeding-edge users in order to continue to improve the APIs and the generated code.

I can especially use feedback from people with Swift experience, since I don't have much.

I'm using `swift-bridge` to ship an application that has extreme reliability requirements, so you can rest assured that the core maintaners have a vested interest in addressing your feedback.

---

The `0.1.x` versions will not follow semver.

We'll maintain semver from `0.2` and onwards.

## Installation

```toml
# In your Cargo.toml

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```

## Resources

- [Book](https://chinedufn.github.io/swift-bridge)

- [Tutorial](https://chinedufn.github.io/swift-bridge/tutorial/running-rust-analyzer-on-an-iphone/index.html)

- [Examples](./examples)

## Quick Peek

Here's a quick peek at how defining FFI bindings looks.

A more thorough walk through of `swift-bridge` can be found in the [book](https://chinedufn.github.io/swift-bridge/index.html). 

```rust
// lib.rs

pub struct ARustStack(Vec<String>);

pub struct SomeType {
    map: HashMap<u32, Vec<u64>>
}

pub struct AnotherType(u8, Box<dyn FnMut() -> bool>);

#[swift_bridge::bridge]
mod ffi {
    // Shared struct definitions can have their fields directly
    // accessed by both Swift and Rust.
    #[swift_bridge(swift_repr = "struct")]
    struct SwiftStackConfig {
        max_size: u8
    }

    extern "Rust" {
        // Exposes super::ARustStack to Swift.
        type ARustStack;

        // Allows Swift to call the `push` method on `ARustStack`.
        // Swift will only be able to call this when it has
        // an owned `ARustStack` or a mutablly referenced
        // `&mut ARustStack`.
        fn push (&mut self, val: String);

        // Allows Swift to call the `pop` method on `ARustStack`.
        fn pop (&mut self) -> Option<String>;
    }

    extern "Rust" {
        // Exposes super::do_stuff to Swift
        fn do_stuff(a: &SomeType, b: &mut AnotherType) -> Option<SwiftStack>;
    }

    extern "Rust" {
        // Exposes super::SomeType to Swift
        type SomeType;
        // Exposes super::AnotherType to Swift
        type AnotherType;

        // The "init" annotation.
        // Indicates that this should show up on the Swift side as
        // a class initializer for SomeType.
        #[swift_bridge(init)]
        fn new() -> SomeType;

        #[swift_bridge(init)]
        fn new_with_u8(val: u8) -> AnotherType;

        fn call(self: &AnotherType) -> bool;
    }

    // Exposes a Swift `class SwiftStack` to Rust.
    extern "Swift" {
        type SwiftStack;

        #[swift_bridge(init)]
        fn new(config: SwiftStackConfig) -> SwiftStack;

        fn push(&mut self, val: String) -> Bool;
    }
}

impl ARustStack {
    fn push(&mut self, val: String) {
        self.0.push(val);
    }

    fn pop(&mut self) -> Option<String> {
        self.0.pop()
    }
}

impl SomeType {
    fn new() -> Self {
        SomeType::default()
    }
}

impl AnotherType {
    fn new_with_u8(val: u8) -> Self {
        AnotherType(val, Box::new(|| true))
    }

    fn call(&self) -> bool {
        (self.0)()
    }
}

fn do_stuff(_a: &SomeType, _b: &mut AnotherType) -> Option<ffi::SwiftStack> {
    let mut swift_stack = ffi::SwiftStack::new(max_size: 10);
    swift_stack.push("hello_world".to_string());

    Some(swift_stack)
}
```

```swift
// Swift

class SwiftStack {
    var stack: [String] = []
    var maxSize: UInt8

    init(config: SwiftStackConfig) {
        self.maxSize = config.max_size
    }

    func push(val: String)  {
        self.stack.push(val)
    }

}

func doThings() {
    // Calls SomeType::new()
    let someType = SomeType()
    // Calls AnotherType::new_with_u8
    let anotherType = AnotherType(val: 50)
    
    let stack: SwiftStack? = do_stuff(someType, anotherType)
}
```

## Known issues

TODO... make GitHub issues for these..

- Fix bug where we can define an extern "Rust" `fn foo () -> SomeType` even though the real definition is `fn foo () -> &SomeType {}`

## Built-In Types

`swift_bridge` comes with support for a number of Rust and Swift standard library types.

| name in Rust                                                    | name in Swift                                                    | notes                                                                                                                                                                                                                                       |
| ---                                                             | ---                                                              | ---                                                                                                                                                                                                                                         |
| u8, i8, u16, i16... etc                                         | UInt8, Int8, UInt16, Int16 ... etc                               |                                                                                                                                                                                                                                             |
| bool                                                            | Bool                                                             |                                                                                                                                                                                                                                             |
| String, &String, &mut String                                    | RustString                                                       |                                                                                                                                                                                                                                             |
| &str                                                            | RustStr                                                          |                                                                                                                                                                                                                                             |
| Vec<T>                                                          | RustVec\<T>                                                      |                                                                                                                                                                                                                                             |
| SwiftArray\<T>                                                  | Array\<T>                                                        | Not yet implemented                                                                                                                                                                                                                         |
| &[T]                                                            | UnsafeBufferPointer\<T>                                          |                                                                                                                                                                                                                                             |
| &mut [T]                                                        | UnsafeMutableBufferPointer\<T>                                   | Not yet implemented                                                                                                                                                                                                                         |
| SwiftString                                                     | String                                                           |                                                                                                                                                                                                                                             |
| Box<T>                                                          |                                                                  | Not yet implemented                                                                                                                                                                                                                         |
| [T; N]                                                          |                                                                  | Not yet implemented                                                                                                                                                                                                                         |
| *const T                                                        | UnsafePointer\<T>                                                |                                                                                                                                                                                                                                             |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         |                                                                                                                                                                                                                                             |
| Option\<T>                                                      | Optional\<T>                                                     | Currently only supports function return types that are primitive (i.e. `-> Option<i32>`), or `-> Option<String>`..<br /> More support will come.<br /> Other places such as function arguments are not yet implemented but will come.<br /> |
| Result\<T>                                                      |                                                                  | Not yet implemented                                                                                                                                                                                                                         |
| Have a Rust standard library type in mind?<br /> Open an issue! |                                                                  |                                                                                                                                                                                                                                             |
|                                                                 | Have a Swift standard library type in mind?<br /> Open an issue! |                                                                                                                                                                                                                                             |

## To Test

To run the test suite.

```sh
# Clone the repository
git clone git@github.com:chinedufn/swift-bridge.git
cd swift-bridge

# Run tests
cargo test --all && ./test-integration.sh
```

## See Also

- [Rust on iOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)
  - A blog post by Mozilla that explains how to run Rust on iOS.
