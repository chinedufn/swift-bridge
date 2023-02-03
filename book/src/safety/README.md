# Safety

`swift-bridge` is fully type safe and mostly memory safe.

## Type Safety

The code that `swift-bridge` is type safe on both the Rust and Swift side,
so all of your interfacing between the two languages is type safe.

## Memory Safety

Needing to worry about memory safety should be very uncommon when using `swift-bridge`.

There are two known situations that can lead to unsafe memory access.

#### Lifetimes

It's possible to pass a reference from `Rust` -> `Swift` and then have `Swift` make use of that
reference after it is no longer safe to do so.

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeType;

        #[swift_bridge(init)]
        fn new();

        fn name(&str) -> &str;
        fn drop(self);
    }
}
```

```swift
let someType = SomeType()

let name: RustStr = someType.name()
someType.drop()

// Undefined behavior since `SomeType` was dropped.
name.toString()
```

It isn't possible for `swift-bridge` to mitigate this, so you just have to be careful with references.

#### Using an owned value after free

Today, it is possible to pass ownership of a value from `Swift` to `Rust` and then
try to use the value from `Swift`.

This mean that a user can accidentally trigger undefined behavior.

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type MyOwnedType;

        fn drop(ty: MyOwnedType);
    }
}
```

```swift
let myOwnedType = MyOwnedType()

drop(myOwnedType)

// Undefined behavior since we no longer own this value.
drop(myOwnedType)
```

After Swift introduces the [consume operator](https://github.com/apple/swift-evolution/blob/main/proposals/0366-move-function.md) we will
be able to prevent this issue by enforcing ownership at compile time.
