# Rust `std::string::String` <---> Swift `String`

Rust's `std::string::String` can be passed to Swift as an owned `String`, a referenced `&String` or a mutably referenced
`&mut String`.

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

There is no zero-copy way to construct a Swift `String` from a byte buffer.

`swift-bridge` seeks to avoid unnecessary allocations, so when passing a Rust `std::string::String` to Swift we create
a `RustString` on the Swift side instead of automatically allocating a new Swift `String`.
Users can then create a Swift `String` themselves by calling the `RustString.toString()` method.

Since a Rust `std::string::String` is `Send+Sync`, a Swift `RustString` is `Sendable` so long as the swift code does not
violate Rust's ownership and aliasing rules.

`RustString`, `RustStringRef` and `RustStringRefMut` implement Swift's `Sendable` protocol.
This is thread safe so long as users uphold the rules in the [Safety](../../safety/README.md) chapter.