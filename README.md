# swift-bridge [![Actions Status](https://github.com/chinedufn/swift-bridge/workflows/test/badge.svg)](https://github.com/chinedufn/swift-bridge/actions) [![docs](https://docs.rs/swift-bridge/badge.svg)](https://docs.rs/swift-bridge)

> Call Rust from Swift and vice versa. 

`swift-bridge` generates code that helps you call Swift from Rust and vice versa.

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
    }

    unsafe extern "Swift" {
        type FileSystemClient;

        fn new_file_system_client() -> UnmanagedPointer<FileSystemClient>;

        fn read(&self, filename: &str) -> Vec<u8>;
    }
}

struct Stack {
    s: Vec<u8>
}
```

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

- [cxx](https://github.com/dtolnay/cxx)
  - Heavily inspired the approach of annotating a module with a macro, among other things.

- [Rust on iOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)
  - A blog post by Mozilla that explains how to run Rust on iOS
