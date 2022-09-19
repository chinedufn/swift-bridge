# Result

Rust's `Result` is seen on the Swift side as a `RustResult`.

## Example

```rust,no_run
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Swift" {
        fn run(
            arg: Box<dyn FnOnce(Result<SomeRustType, String>)>
        );
    }

    extern "Rust" {
        type SomeRustType;
    }
```

```swift
func run(arg: (RustResult<SomeRustType, String>) -> ()) {
    arg(.Err("Something went wrong"))
}
```
