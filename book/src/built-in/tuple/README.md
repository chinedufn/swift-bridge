# (A, B, C, ...) <---> (A, B, C, ...)

Rust's `(A, B, C, ...)` is seen on the Swift side as a `(A, B, C, ...)`.

## Returning Tuple from Rust -> Swift

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeRustType;

        fn run() -> (SomeRustType, i32);
    }
}
```

```swift
// Swift

func run() -> (SomeRustType, i32) {
    // ...
}
```

## Taking Tuple from Swift -> Rust

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeRustType;
    }
    extern "Rust" {
        fn run(
            arg: (SomeRustType, i32)
        );
    }
}
```

```swift
// Swift

let someType = SomeType()
run((someType, 123))
```
