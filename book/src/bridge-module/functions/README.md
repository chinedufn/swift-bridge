# Functions

... TODO OVERVIEW ...

## Function Attributes

#### #[swift_bridge(args_into = (arg_name, another_arg_name))]

Used to name the arguments that should have `.into()` called on them when
passing them to their handler function.

One use case is for exposing a third-party type as a shared struct.

```rust
mod pretend_this_is_some_third_party_crate {
    // We want to expose this third-party struct as a shared struct.
    pub struct UniqueId {
        id: u64
    }
}
use pretend_this_is_some_third_party_crate::UniqueId;

fn a_function (_some_arg: UniqueId, _an_arg: UniqueId, _cool_arg: u8) {
    // ...
}

mod ffi {
    struct FfiUniqueId(u64);

    extern "Rust" {
        // super::a_function does not take a `u64` or an `FfiUniqueId`,
        // but this still works since they both `impl Into<UniqueId>`.
        #[swift_bridge(args_into = (some_arg, an_arg))]
        fn a_function(some_arg: u64, an_arg: FfiUniqueId, cool_arg: u8);
    }
}

impl From<u64> for UniqueId {
    fn from(id: u64) -> UniqueId {
        UniqueId {
            id
        }
    }
}

impl Into<UniqueId> for ffi::FfiUniqueId {
    fn into(self) -> UniqueId {
        UniqueId(self.0)
    }
}
```

#### #[swift_bridge(associated_to = SomeType)]

Indicates that we are exposing an associated function for a type.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Message;

        // Exposes Message::parse to Swift as Message.parse
        #[swift_bridge(associated_to = Message)]
        fn parse(text: &str) -> Option<Message>;
    }
}

struct LongMessage(String);

impl LongMessage {
    fn parse(text: impl ToString) -> Option<Self> {
        let text = text.to_string();

        if text.len() > 10_000 {
            Some(LongMessage(text))
        } else {
            None
        }
    }
}
```

```swift
// Swift

func maybeSendLongMessage(text: String) {
    let maybeMessage = Message.parse(text)
    
    if let message = maybeMessage {
        // ... send the message
    }
}
```

#### #[swift_bridge(Identifiable)]

Used to generate a Swift `Idenfiable` protocol implementation.

```rust
// Rust

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeType;

        #[swift_bridge(Identifiable, swift_name = "someFunction")]
        fn some_function(&self) -> i16;
    }
}
```

```swift
// Generated Swift
// (rough example, the real generated code looks a little different)

class SomeType {
    // ...
}
extension SomeType: Identifiable {
    var id: UInt16 {
        return self.someFunction()
    }
}
```

#### #[swift_bridge(into_return_type)]

Allows a swift-bridge definition of `fn foo() -> T` to work for any `fn foo() -> impl Into<T>`.

```rust
use some_other_crate::Uuid;

#[swift_bridge::bridge]
mod ffi {
	struct FfiUuid {
	    uuid: [u8; 16]
	}

    extern "Rust" {
        #[swift_bridge(return_into)]
        fn make_uuid() -> FfiUuid;
    }
}

impl From<Uuid> for ffi::FFiUuid {
	fn from(uuid: Uuid) -> ffi::FfiUuid {
	    unsafe { std::mem::transmute(uuid) }
	}
}

use self::some_other_crate::make_uuid;
mod some_other_crate {
	pub struct Uuid {
	    uuid: [u8; 16]
	}

    // Here we can return a Uuid, even though swift-bridge is expecting an FfiUuid.
    pub fn make_uuid() -> Uuid {
        Uuid::new_v4()
    }
}
```

#### #[swift_bridge(return_with = path::to::some_function)]

Allows a swift-bridge definition of `fn foo() -> T` to work for a `fn foo() -> U` by
passing `T` to a `fn(T) -> U`.

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(return_with = some_module::convert_str_to_u32)]
        fn get_str_value_return_with() -> u32;
    }
}

fn get_str_value_return_with() -> &'static str {
    "123"
}

mod some_module {
    pub fn convert_str_to_u32(val: &str) -> u32 {
        val.parse().unwrap()
    }
}
```

#### #[swift_bridge(rust_name = "function_name")]

Use the given `rust_name` to find the function's implementation.

```rust
use some_other_crate::Uuid;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(rust_name = "another_function")]
        fn some_function();
    }
}

fn another_function() {
}
```
