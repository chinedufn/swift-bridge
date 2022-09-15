# Boxed Functions

## Box<dyn FnOnce(A, B) -> C>

`swift-bridge` supports bridging boxed `FnOnce` functions with any number of arguments.

There is a panic if you attempt to call a bridged `FnOnce` function more than once.

```rust
#[swift_bridge::bridge]
mod ffi {
	extern "Swift" {
	    type CreditCardReader;
	    type Card;
	    type CardError;

        fn processCard(
            self: &CreditCardReader,
            callback: Box<dyn FnOnce(Result<Card, CardError>) -> ()>
        );
	}
}
```
