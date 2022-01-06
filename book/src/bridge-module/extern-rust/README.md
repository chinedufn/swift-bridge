# extern "Rust"

`extern "Rust` sections are used to expose Rust types and functions over FFI so that they can be used from Swift code.

```rust
mod science;

use science::{ScienceLab, Hydrogen, Oxygen, make_water};

#[swift_bridge::bridge]
mod ffi {
	extern "Rust" {
	    type Water;

        #[swift_bridge(associated_to = "Water")]
	    fn new() -> Water;

	    fn is_wet(&self) -> bool;
	}

	extern "Rust" {
	    type ScienceLab;
	    type Hydrogen;
	    type Oxygen;

	    fn make_water(
	        lab: &ScienceLab,
	        hydrogen: Hydrogen,
	        oxygen: Oxygen
	    ) -> Water;
	}
}

pub struct Water;

impl Water {
	fn new () -> Self {
	    Water
	}

	fn is_wet(&self) -> bool {
	    unreachable!("Seriously...?")
	}
}
```

## Owned, Ref and RefMut

When you define a type in an `extern "Rust"` block, three corresponding Swift classes get generated.

```swift
// Equivalent to `SomeType` in Rust
class SomeType: SomeTypeRefMut {
    // ...
}

// Equivalent to `&mut SomeType` in Rust
class SomeTypeRefMut: SomeTypeRef {
    // ... 
}

// Equivalent to `&SomeType` in Rust
class SomeTypeRef {
    // ... 
}
```

Here's an example of how `&Type` and `&mut Type` are enforced:

```rust
// Rust

extern "Rust" {
    type SomeType;
    
    #[swift_bridge(init)]
    fn new() -> SomeType;
    
    // Callable by SomeType, SomeTypeRef and SomeTypeRefMut.
    fn (&self) everyone();
    
    // Callable by SomeType, and SomeTypeRefMut.
    fn (&mut self) only_owned_and_ref_mut();
    
    // Only callable by SomeType.
    fn (self) only_owned();
}

extern "Rust" {    
    fn make_ref() -> &'static SomeType;
    
    fn make_ref_mut() -> &'static mut SomeType;
}
```

```swift
// Swift

func methods() {
    let someType: SomeType = SomeType()
    let someTypeRef: SomeTypeRef = make_ref()
    let someTypeRefMut: SomeTypeRefMut = make_ref_mut()
    
    someType.everyone()
    someType.only_owned_and_ref_mut()
    someType.only_owned()
    
    someTypeRefMut.everyone()
    someTypeRefMut.only_owned_and_ref_mut()
    
    someTypeRef.everyone()
}

func functions() {
    let someType: SomeType = SomeType()
    let someTypeRef: SomeTypeRef = make_ref()
    let someTypeRefMut: SomeTypeRefMut = make_ref_mut()

    takeReference(someType)
    takeReference(someTypeRef)
    takeReference(someTypeRefMut)
}

// Can be called with SomeType, SomeTypeRef and SomeTypeRefMut
func useSomeType(someType: SomeTypeRef) {
    // ...
}
```

## Opaque Type Attributes

#### #[swift_bridge(already_declared)]

The `already_declared` attribute allows you to use the same type in multiple bridge modules.

```rust
use some_crate::App;

mod ffi {
	extern "Rust" {
	    type App;

        #[swift_bridge(init)]
	    fn new() -> App;
	}
}

#[swift_bridge::bridge]
#[cfg(feature = "dev-utils")]
mod ffi_dev_utils {
	extern "Rust" {
        // We won't emit Swift and C type definitions for this type
        // since we've already declared it elsewhere.
	    #[swift_bridge(already_declared)]
        type App;

        fn create_logged_in_user(&mut self, user_id: u8);
	}
}
```

## Function Attributes

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
        #[swift_bridge(into_return_type)]
        fn make_uuid() -> FfiUuid;
    }
}

impl From<Uuid> for ffi::FFiUuid {
	fn from(uuid: Uuid) -> ffi::FfiUuid {
	    unsafe { std::mem::transmute(uuid) }
	}
}

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
