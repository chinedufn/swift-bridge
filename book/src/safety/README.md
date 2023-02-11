# Safety

`swift-bridge` is fully type safe and mostly memory safe.

## Type Safety

All of the Rust and Swift FFI code that `swift-bridge` generates
for you is type safe.

## Memory Safety

You can ensure the memory safety of your Rust and Swift projects by following these rules:

- In Swift, never use a reference to a Rust type after its lifetime.

- In Swift, pass a mutable reference to Rust that will live alongside another active reference to that type.

- In Swift, never use a type after passing ownership to Rust.

Let's look at some examples of code that violates these rules:

### Never use a reference after its lifetime

It is possible to pass a reference from `Rust` -> `Swift` and then have `Swift` make use of that
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
// Swift

let someType = SomeType()

let name: RustStr = someType.name()
someType.drop()

// Undefined behavior since `SomeType` was dropped.
name.toString()
```

It isn't possible for `swift-bridge` to mitigate this, so be mindful when handling references.

### Never pass a mutable reference to Rust that will live alongside another active reference

Rust expects that if there is mutable reference to a value, no other references to that value are held.

This rule is not enforced on the Swift side, making it possible to pass aliasing pointers to Rust
and trigger undefined behavior.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type MyList;

        #[swift_bridge(init)]
        fn new() -> MyList;

        fn extend(a: &mut self, b: &MyList);
    }
}
```

```swift
// Swift

let myList = MyList()

// This can cause undefined behavior!
myList.extend(myList)
```

---

```rust
// Rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type MyList;

        fn new() -> MyList;

        fn allegedly_immutable(&self, callback: Box<dyn Fn()>);
        fn mutate(&mut self);
    }
}
```

```swift
// Swift

let myList = MyList()

myList.allegedly_immutable({
    // If the `allegedly_immutable` method calls this
    // callback we will take a mutable reference to `MyList`
    // while there is an active immutable reference.
    // This violates Rust's borrowing rules.
    myList.mutate()
})
```

To stay safe, when passing a mutable reference to a Rust value from Swift to Rust
do not pass any other references to that same value.

### Never use a value after it is dropped

Today, it is possible to pass ownership of a value from `Swift` to `Rust`
and then unsafely access the value from `Swift`.

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
