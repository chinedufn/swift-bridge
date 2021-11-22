# Option <---> Optional

Rust's `Option` is seen on the Swift side as a Swift `Optional`.

## Example

```rust,no_run
// Rust

#[swift_bridge::bridge]
mod ffi {
	extern "Rust" {
	    fn make_rust_option() -> Option<u8>;
	}

	extern "Swift" {
	    fn make_swift_optional() -> Option<bool>;
	}
}

fn make_rust_option() -> Option<u8> {
	if ffi::make_swift_optional == Some(true) {
	    Some(111)
	} else {
	    None
	}
}
```

```swift
// Swift

func call_rust_and_divide_by_2() -> Optional<UInt8> {
	if case let val? = make_rust_option() {
	    return val / 2
	} else {
	    nil
	}
}

func make_swift_optional() -> Bool? {
    true
}
```
