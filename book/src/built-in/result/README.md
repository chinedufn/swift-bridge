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

## Returning Result from Swift -> Rust (async)

When an `extern "Swift"` async function returns `Result<T, E>`, the Swift implementation
must use typed throws (Swift 5.9+).

```rust,no_run
// Rust

#[swift_bridge::bridge]
mod ffi {
    enum MyError {
        SomethingWentWrong,
    }

    extern "Swift" {
        async fn fetch_data() -> Result<u32, MyError>;
    }
}

async fn example() {
    match ffi::fetch_data().await {
        Ok(data) => println!("Got: {}", data),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

```swift
// Swift

// Shared enums need Error conformance
extension MyError: Error {}

// IMPORTANT: Must use typed throws (Swift 5.9+)
func fetch_data() async throws(MyError) -> UInt32 {
    throw MyError.SomethingWentWrong
}
```

**Key requirements:**
- Use `throws(E)` (typed throws) instead of just `throws`
- Add `Error` conformance to shared enums: `extension MyError: Error {}`
- Requires Swift 5.9 or later
