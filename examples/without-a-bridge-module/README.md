# without-a-bridge-module

`swift-bridge`'s code generators live in `crates/swift-bridge-ir`.

This example demonstrates how one might use the `swift-bridge-ir` crate directly in order to generate a Rust+Swift FFI boundary.

This is mainly useful for library authors who might wish to expose an alternative frontend, such as being able to annotate types:
```rust
use some_third_party_lib;

/// An imaginary third-party library that wraps `swift-bridge-ir`
/// in a proc macro attribute that users can annotate their types
/// with.
#[some_third_party_lib::ExposeToSwift]
pub struct User {
    name: String
}
```

## To Run

```sh
./run.sh
```
