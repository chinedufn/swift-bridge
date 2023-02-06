# Result

## Returning Result from Rust -> Swift

```rust,no_run
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeRustType;

        fn run() -> Result<SomeRustType, String>;
    }
}
```

```swift
// Swift

func run() throws -> SomeRustType {
    // ...
}
```

## Swift function that takes a callback

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
}
```

```swift
// Swift

func run(arg: (RustResult<SomeRustType, String>) -> ()) {
    arg(.Err("Something went wrong"))
}
```
