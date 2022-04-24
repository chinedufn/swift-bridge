# Transparent Structs

You can define structs whos fields can be accessed by both Rust and Swift.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge::bridge(swift_repr = "struct")]
    struct SomeSharedStruct {
        some_field: u8,
        another_field: Option<u64>
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
    return SomeSharedStruct(some_field: 123, another_field: nil)
}
```

### Struct Attributes

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

_Valid values are "struct" or "class"._

How the struct should appear on the Swift side.

Swift structs are copy-on-write, so we do not allow you to mutate the fields of a `swift_repr = "struct"`
since you wouldn't be changing the original struct.

The `swift_repr ="class"` representation allows you to pass mutable references to shared structs between Rust and Swift.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(struct_repr = "struct")]
    struct SomeStructReprStruct {
        field: UInt8,
    }

    #[swift_bridge(struct_repr = "class")]
    struct SomeStructReprClass {
        field: UInt8,
    }

    // NOTE: This is aspirational. Exposing methods on shared structs
    //  doesn't actually work yet.
    extern "Rust" {
        // All structs can expose `&self` methods.
        fn repr_struct_ref(self: &SomeStructReprStruct);
        fn repr_class_ref(self: &SomeStructReprStruct);

        // Only structs with `swift_repr = "class"` can expose `&self` methods.
        fn repr_class_ref_mut(self: &mut SomeStructReprStruct);
    }
}

impl ffi::SomeStructReprStruct {
    fn repr_struct_ref(&self) {
        // ...
    }

    // swift-bridge cannot expose this method since mutable methods
    // on `swift_repr = "struct"` structs are not supported.
    fn repr_struct_ref_mut(&mut self) {
        // ...
    }
}

impl ffi::SomeStructReprClass {
    fn repr_class_ref(&self) {
        // ...
    }

    fn repr_class_ref_mut(&mut self) {
        // ...
    }
}
```

```swift
// Generated Swift

struct SomeStructReprStruct {
    var field: UInt8
    
    func repr_struct_ref() {
      // ... call Rust's SomeStructReprStruct.repr_struct_ref() ...
    }
}

class SomeStructReprClass: SomeStructReprClassRefMut {
    // ...
}
class SomeStructReprClassRefMut: SomeStructReprClassRef {
    // ...
}
class SomeStructReprClassRef {
    // ...
}
```
