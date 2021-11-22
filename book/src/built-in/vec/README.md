# Vec <--> RustVec

Rust's `std::vec::Vec` is seen on the Swift side as a `RustVec`.

`RustVec` implements Swift's `IteratorProtocol`, allowing you do do things like:

```swift
let vec: RustVec = get_rust_vec_somehow()
for value in vec {
    print(value)
}
```

## Example

```rust,no_run
// Rust

#[swift_bridge::bridge]
mod ffi {
	extern "Rust" {
	    fn make_rust_vec() -> Vec<u32>;

	    fn make_rust_vec_with_initial_contents(initial: &[i16]) -> Vec<i16>;
	}
}

fn make_rust_vec() -> Vec<u32> {
    vec![5, 8, 11]
}

fn make_rust_vec_with_initial_contents(initial: &[16]) -> Vec<u16> {
    intial.to_vec()
}
```

```swift
// In Swift

func testMakeAVec () {
    let vec: RustVec = get_vec_from_rust()

    XCTAssertEqual(vec.pop(), 5)
    XCTAssertEqual(vec.pop(), 8)
    XCTAssertEqual(vec.pop(), 11)
    XCTAssertEqual(vec.pop(), nil)

    vec.push(50)
    vec.push(75)
    XCTAssertEqual(vec.get(1), 75)
}

func testMakeAnotherVec () {
    let initial: [Int16] = [3, 5, 7]
    let vec: RustVec = get_vec_from_rust(initial.toUnsafeBufferPointer())

    XCTAssertEqual(vec.len(), 3);

	for (index, value) in vec.enumerate() {
	    XCTAssertEqual(value, initial[index])
	}
}
```
