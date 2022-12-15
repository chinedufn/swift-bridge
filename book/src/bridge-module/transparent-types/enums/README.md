# Transparent Enums

You can define enums that can be created by both Rust and Swift.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    enum BarCode {
        Upc(i32, i32, i32, i32),
        QrCode {
            code: String
        }
    }

    extern "Rust" {
        fn get_link(code: BarCode) -> String;
    }

    extern "Swift" {
        fn create_bar_code(upc: bool) -> BarCode;
    }
}

fn get_link (code: BarCode) -> String {
    // ...
}
```

```swift
// Swift 

func create_bar_code(upc: Bool) -> BarCode {
    if upc {
        return BarCode.Upc(8, 85909, 51226, 3)
    } else {
        return BarCode.QrCode(code: "ABCDEFG")
    }
}
```

### Enum Attributes

#### #[swift_bridge(already_declared)]

```rust
#[swift_bridge::bridge]
mod ffi_1 {
    enum SomeTransparentEnum {
        Variant
    }
}

use ffi_1::SomeTransparentEnum;

#[swift_bridge::bridge]
mod ffi_2 {
    // The `already_declared` indicates that instead of creating a new enum
    // we should use super::SomeTransparentEnum;
    #[swift_bridge(already_declared)]
    enum SomeTransparentEnum;

    extern "Rust" {
        fn some_function() -> SomeTransparentEnum;
    }
}
```

#### #[swift_bridge(swift_name = "...")]

Set the name that is used when generating the enum on the Swift side.

```rust
#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_name = "RenamedEnum")]
    enum SomeTransparentEnum {
        Variant
    }
}
```
