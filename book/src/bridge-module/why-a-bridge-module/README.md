# Why a Bridge Module?

The `swift-bridge` project provides direct support for expressing the Rust+Swift FFI boundary using one or more bridge modules such as:
```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn generate_random_number() -> u32;
    }
}

fn generate_random_number() -> u32 {
    rand::random()
}
```

`swift-bridge`'s original maintainer wrote `swift-bridge` for use in a cross platform application where he preferred to keep his FFI code separate from his application code.
He believed that this separation would reduce the likelihood of him biasing his core application's design towards types that were easier to bridge to Swift.

While in the future `swift-bridge` may decide to directly support other approaches to defining FFI boundaries, at present only the bridge module approach is directly supported.

Users with other needs can write wrappers around `swift-bridge` to expose alternative frontends.

The `examples/without-a-bridge-macro` example demonstrates how to reuse `swift-bridge`'s code generation facilities without using a bridge module.

## Inline Annotations

The main alternative to the bridge module design would be to support inline annotations where one could describe their FFI boundary by annotating their Rust types.

For instance a user might wish to expose their Rust banking code to Swift using an approach such as:
```rust
// IMAGINARY CODE. WE DO NOT PROVIDE A WAY TO DO THIS.

#[derive(Swift)]
pub struct BankAccount {
    balance: u32
}

#[swift_bridge::bridge]
pub fn create_bank_account() -> BankAccount {
    BankAccount {
        balance: 0
    }
}
```

`swift-bridge` aims to be a low-level library that generates far more efficient FFI code than a human would write and maintain themselves.

The more information that `swift-bridge` has at compile time, the more efficient code it can generate.

Let's explore an example of bridging a `UserId` type, along with a function that returns the latest `UserId` in the system.

```rust
type Uuid = [u8; 16];

#[derive(Copy)]
struct UserId(Uuid);

pub fn get_latest_user() -> Result<UserId, ()> {
    Ok(UserId([123; 16]))
}
```

In our example, the `UserId` is a wrapper around a 16 byte UUID.

Exposing this as a bridge module might look like:

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Copy(16))]
        type UserId;

        fn get_latest_user() -> UserId;
    }
}
```

Exposing the `UserId` using inlined annotation might look something like:

```rust
// WE DO NOT SUPPORT THIS

type Uuid = [u8; 16];

#[derive(Copy, ExposeToSwift)]
struct UserId(Uuid);

#[swift_bridge::bridge]
pub fn get_latest_user() -> Result<UserId, ()> {
    UserId([123; 16])
}
```

In the bridge module example, `swift-bridge` knows at compile time that the `UserId` implements `Copy` and has a size of `16` bytes.

In the inlined annotation example, however, `swift-bridge` does not know the `UserId` implements `Copy`.

While it would be possible to inline this information, it would mean that users would need to remember to inline this information
on every function that used the `UserId`.
```rust
// WE DO NOT SUPPORT THIS

#[swift_bridge::bridge]
#[swift_bridge(UserId impl Copy(16))]
pub fn get_latest_user() -> Result<UserId, ()> {
    UserId([123; 16])
}
```

We expect that users would find it difficult to remember to repeat such annotations, meaning users would tend to expose less efficient bridges
than they otherwise could have.

If `swift-bridge` does not know that the `UserId` implements `Copy`, it will need to generate code like:
```rust
pub extern "C" fn __swift_bridge__get_latest_user() -> *mut UserId {
    let user = get_latest_user();
    match user {
        Ok(user) => Box::new(Box::into_raw(user)),
        Err(()) => std::ptr::null_mut() as *mut UserId,
    }
}
```

Whereas if `swift-bridge` knows that the `UserId` implements `Copy`, it might be able to avoid an allocation by generating code such as:
```rust
/// `swift-bridge` could conceivably generate code like this to bridge
/// a `Result<UserId, ()>`.
/// Here we use a 17 byte array where the first byte indicates `Ok` or `Err`
/// and, then `Ok`, the last 16 bytes hold the `UserId`.
/// We expect this to be more performant than the boxing in the previous
/// example codegen.
pub extern "C" fn __swift_bridge__get_latest_user() -> [u8; 17] {
    let mut bytes: [u8; 17] = [0; 17];

    let user = get_latest_user();

    match user {
        Ok(user) => {
            let user_bytes: [u8; 16] = unsafe { std::mem::transmute(user) };
            (&mut bytes[1..]).copy_from_slice(&user_bytes);

            bytes[0] = 255;
            bytes
        }
        Err(()) => {
            bytes
        }
    }
}
```

More generally, the more information that `swift-bridge` has about the FFI interface, the more optimized code it can generate.
The bridge module design steers users towards providing more information to `swift-bridge`, which we expect to lead to more efficient
applications.

Users that do not need such efficiency can explore reusing `swift-bridge` in alternative projects that better meet their needs.
