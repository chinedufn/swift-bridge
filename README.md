# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge)

> Call Rust from Swift and vice versa. 

`swift-bridge` generates code that helps you call Swift from Rust and vice versa.

_`swift-bridge` takes inspiration from the awesome bridge module macro based approach pioneered by [cxx](https://github.com/dtolnay/cxx)._

## Installation

```toml
# In your Cargo.toml

[build-dependencies]
swift-bridge-build = "0.1"

[dependencies]
swift-bridge = "0.1"
```



## Quick Peek

Here's a quick peek at the Rust and Swift of a bridge that should give you a sense of how bindings look.

A more thorough walk through of `swift-bridge` can be found in the book (TODO: Link to GitHub pages).

```rust
// build.rs

fn main() {
    let bridges = vec!["src/lib.rs"];

    let out_dir = "./generated";
    swift_bridge_build::parse_bridges(&bridges)
        .write_all_concatenated(out_dir);

    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }
}
```

```rust
// lib.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type ARustStack;

        fn push (&mut self, val: u8);

        fn pop (&mut self) -> Option<u8>;
      
        fn as_slice (&self) -> &[u8];

        fn do_stuff(override: Option<u8>);
    }

    extern "Swift" {
        type SwiftApiClient;

        #[swift_bridge(init)]
        fn new_with_timeout(timeout: u8) -> SwiftApiClient;

        #[swift_bridge(associated_to = FileSystemClient)]
        fn version () -> u32;

        fn set_timeout(&self, timeout: u8);
    }
}

struct ARustStack(Vec<u8>);

impl ARustStack {
	fn push(&mut self, val: u8) {
	    self.0.push(val);
	}

	fn pop(&mut self) -> Option<u8> {
	    self.0.pop();
	}

	fn as_slice(&self) -> &[u8] {
	    self.0.pop();
	}
}

fn do_stuff(override: Option<u8>) {
    assert_eq!(SwiftApiClient::version(), 1);

    let client = SwiftApiClient::new_with_timeout(10);

	if let Some(override) = override {
        client.setTimeout(20)
	}
}
```

```swift
// Swift

class SwiftApiClient {
    var timeout: UInt8

	init(timeout: UInt8) {
        self.timeout = timeout
    }

	class func version() -> u32 {
	    1
	}

	func setTimeout(timeout: UInt8) {
	    self.timeout = timeout
	}
}
```

## TODO's

- Delete bridging header from Xcode
- Look up how to programatically set the linking settings.
  Our docs can recommend that as well as show how to manually set them
- Delete c header code generation
- Rename SwiftRustIntegrationTestRunner to SwiftRustIntegrationTests
- Look into std::mem::unitialized to see whether or not our usage of it is unsafe

## Quick Peek


- Write instructions on going from 0 to most basic iOS app

- Add book chapter on setting up iOS app from scratch

- Create examples dir example of iOS app
- Create examples dir example of macOS app

## Built-In Types

`swift_bridge` comes with support for a number of Rust and Swift standard library types.

| name in Rust                                                    | name in Swift                                                    | notes                                                                                                                                                                               |
| ---                                                             | ---                                                              | ---                                                                                                                                                                                 |
| u8, i8, u16, i16... etc                                         | UInt8, Int8, UInt16, Int16 ... etc                               |                                                                                                                                                                                     |
| bool                                                            | Bool                                                             |                                                                                                                                                                                     |
| String                                                          | RustString                                                       |                                                                                                                                                                                     |
| &str                                                            | RustStr                                                          |                                                                                                                                                                                     |
| Vec<T>                                                          | RustVec\<T>                                                      |                                                                                                                                                                                     |
| SwiftArray\<T>                                                  | Array\<T>                                                        | Not yet implemented                                                                                                                                                                 |
| &[T]                                                            | UnsafeBufferPointer\<T>                                          |                                                                                                                                                                                     |
| &mut [T]                                                        | UnsafeMutableBufferPointer\<T>                                   | Not yet implemented                                                                                                                                                                 |
| SwiftString                                                     | String                                                           |                                                                                                                                                                                     |
| Box<T>                                                          |                                                                  | Not yet implemented                                                                                                                                                                 |
| [T; N]                                                          |                                                                  | Not yet implemented                                                                                                                                                                 |
| *const T                                                        | UnsafePointer\<T>                                                |                                                                                                                                                                                     |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         |                                                                                                                                                                                     |
| Option\<T>                                                      | Optional\<T>                                                     | Currently only supported for primitive function return types.<br /> Other places such as function arguments are  not yet implemented.<br /> Non primitive T is not yet implemented. |
| Result\<T>                                                      |                                                                  | Not yet implemented                                                                                                                                                                 |
| Have a Rust standard library type in mind?<br /> Open an issue! |                                                                  |                                                                                                                                                                                     |
|                                                                 | Have a Swift standard library type in mind?<br /> Open an issue! |                                                                                                                                                                                     |

## To Test

To run the test suite.

```sh
# Clone the repository
git clone git@github.com:chinedufn/swift-bridge.git
cd swift-bridge

# Run tests
cargo test --all
```

## See Also

- [Rust on iOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)
  - A blog post by Mozilla that explains how to run Rust on iOS.
