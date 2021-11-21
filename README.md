# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge)

> Call Rust from Swift and vice versa. 

`swift-bridge` generates code that helps you call Swift from Rust and vice versa.

_The bridging module approach that `swift-bridge` uses was inspired by [cxx](https://github.com/dtolnay/cxx)._

## TODO's

- Delete bridging header from Xcode
- Look up how to programatically set the linking settings.
  Our docs can recommend that as well as show how to manually set them
- Delete c header code generation
- Rename SwiftRustIntegrationTestRunner to SwiftRustIntegrationTests

## Quick Peek

- Write instructions on going from 0 to most basic iOS app

- Add book chapter on setting up iOS app from scratch

- Create examples dir example of iOS app
- Create examples dir example of macOS app

```rust
use swift_bridge::UnmanagedPtr;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Stack;

        fn push (stack: &mut Stack, val: u8);

        fn pop (stack: &mut Stack) -> Option<u8>;
      
        fn as_slice (&self) -> &[u8];
    }

    extern "Swift" {
        type FileSystemClient;

        #[swift_bridge(init)]
        fn new() -> FileSystemClient;

        fn read(&self, filename: &str) -> Vec<u8>;

        #[swift_bridge(associated_to = FileSystemClient)]
        fn write (bytes: &[u8]);
    }
}
```

## Built-In Types

`swift_bridge` comes with support for a number of Rust and Swift standard library types.

| name in Rust                                                    | name in Swift                                                    | notes               |
| ---                                                             | ---                                                              | ---                 |
| u8, i8, u16, i16... etc                                         | UInt8, Int8, UInt16, Int16 ... etc                               |                     |
| bool                                                            | Bool                                                             |                     |
| String                                                          | RustString                                                       |                     |
| &str                                                            | RustStr                                                          |                     |
| Vec<T>                                                          | RustVec\<T>                                                      | Not yet implemented |
| SwiftArray\<T>                                                  | Array\<T>                                                        | Not yet implemented |
| &[T]                                                            | UnsafeBufferPointer\<T>                                          |                     |
| &mut [T]                                                        | UnsafeMutableBufferPointer\<T>                                   | Not yet implemented |
| SwiftString                                                     | String                                                           |                     |
| Box<T>                                                          |                                                                  | Not yet implemented |
| [T; N]                                                          |                                                                  | Not yet implemented |
| *const T                                                        | UnsafePointer\<T>                                                | Not yet implemented |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         | Not yet implemented |
| Option\<T>                                                      |                                                                  | Not yet implemented |
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
cargo test --all
```

## See Also

- [Rust on iOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)
  - A blog post by Mozilla that explains how to run Rust on iOS.
