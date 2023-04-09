# (A, B, C, ...) <---> (A, B, C, ...)

Rust's `(A, B, C, ...)` is seen on the Swift side as a `(A, B, C, ...)`.

## Returning Tuple from Rust -> Swift

```rust
// Rust

mod ffi {
    extern "Rust" {
        fn get_midpoint(
            point1: (f32, f32, f32),
            point2: (f32, f32, f32),
        ) -> (f32, f32, f32);
    }

    extern "Swift" {
        fn make_point() -> (f32, f32, f32);
    }
}

fn get_midpoint(
    point1: (f32, f32, f32),
    point2: (f32, f32, f32)
) -> (f32, f32, f32) {
    // ...
}
```

```swift
// Swift

func make_point() -> (Float, Float, Float) {
    (1.0, 2.0, 3.0)
}

let midpoint = get_midpoint(
    make_point(1.0, 2.0, 3.0),
    make_point(4.0, 5.0, 6.0)
)
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
