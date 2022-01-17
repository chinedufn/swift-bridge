# Structs

You can define structs whos fields can be accessed by both Rust and Swift.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge::bridge(swift_repr = "struct")]
    struct SomeSharedStruct {
        some_field: u8
    }

    extern "Rust" {
        fn some_function(val: SomeSharedStruct);
    }

    extern "Swift" {
        fn another_function() -> SomeSharedStruct;
    }
}

fn some_function (val: ffi::SomeSharedStruct) {
    // ...
}
```

```swift
// Swift 

func another_function() -> SomeSharedStruct {
    return SomeSharedStruct(some_field: 123)
}
```

## Struct Attributes

#### #[swift_bridge::bridge(already_declared)]

```rust
#[swift_bridge::bridge]
mod ffi_1 {
    #[swift_bridge(swift_repr = "struct")]
    struct SomeSharedStruct {
        some_field: u8
    }
}

use ffi_1::SomeSharedStruct;

#[swift_bridge::bridge]
mod ffi_2 {
    // The `already_declared` indicates that instead of creating a new Struct
    // we should use super::SomeSharedStruct;
    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct SomeSharedStruct;

    extern "Rust" {
        fn some_function() -> SomeSharedStruct;
    }
}
```

#### #[swift_bridge::bridge(swift_repr = "...")]

Valid values are "struct" or "class".

How the struct should appear on the Swift side.

Swift structs get copied on write, so you can't mutate the fields of a `swift_repr = "struct"` on the Swift
side and those changes reflected on the Rust side.

The "class" representation, allows you to pass mutable references to shared structs between Rust and Swift.
