# String <---> String

Rust's `std::string::String` can be passed to Swift as an owned `String`, a referenced `&String` or 
a mutably referenced `&mut String`.

```rust
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

        // Swift returns a `RustString` and
        // Rust receives a `std::string::String`.
	    fn make_string() -> String;
	}
}
```
