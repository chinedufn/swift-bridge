# String <---> String

Rust's `std::string::String` can be passed to Swift as an owned `String`, a referenced `&String` or a mutably referenced `&mut String`.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
	extern "Rust" {
	    type SomeRustType;

	    // Becomes a `RustString` when passed to Swift.
	    fn make_string() -> String;

	    // Becomes a `RustStringRef` when passed to Swift.
	    fn make_ref_string(&self) -> &String;

	    // Becomes a `RustStringRefMut` when passed to Swift.
	    fn make_ref_mut_string(&mut self) -> &mut String;

        // Swift calls this with a `RustString` and
        // Rust receives a `std::string::String`.
	    fn take_string(string: String);
	}

	extern "Swift" {
	    type SomeSwiftType;

	    fn make_rust_string() -> String;
	    fn make_swift_string() -> String;
	}
}
```

```swift
// Swift

func make_rust_string() -> RustString {
    RustString("Good Day")
}

// Swift can return anything that implements the `IntoRustString` protocol.
// `swift-brdige` automatically implements this for Swift's `String` type.
func make_swift_string() -> String {
    "Hello World"
}
```

## RustString

Since both Rust and Swift have their own `String` type, it is technically possible for us to automatically
convert owned Rust `std::string::String`s into Swift `String`s.

However, we do not do this since Swift does not currently have a way to construct a Swift `String` without copying.

This means that if we were to automatically convert from a Rust `std::string::String` to a Swift `String` we would have to create a new
Swift allocation and copy all of the Rust `std::string::String` bytes to that allocation.

`swift-bridge` seeks to avoid unnecessary allocations, so instead of performing this implicit allocation
we pass a `RustString` type from Rust to Swift.

The `RustString`'s `.toString()` method can then be called on the Swift side to get a Swift `String`.
