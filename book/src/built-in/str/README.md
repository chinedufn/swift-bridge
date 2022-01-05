# &str <---> RustStr

Rust's `std::str` can be passed to Swift as a `RustStr`.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
	    type SomeRustType;

	    // Becomes a `RustStr` when passed to Swift.
	    fn make_str() -> &'static str;

	    fn get_str(self: &SomeRustType) -> &str;
	}

	extern "Swift" {
	    type SomeSwiftType;

        // Swift returns a `RustStr` and
        // Rust receives a `&str`.
	    fn make_string() -> &str;
	}
}
```
