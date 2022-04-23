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

... TODO
