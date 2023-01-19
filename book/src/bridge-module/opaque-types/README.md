# Opaque Types

... TODO OVERVIEW ...

## Exposing Opaque Rust Types

`extern "Rust` sections are used to expose Rust types and their associated methods and functions
so that they can be used from Swift code.

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

#### #[swift_bridge(Copy($SIZE))]

If you have an opaque Rust type that implements `Copy`, you will typically want to be
able to pass it between Swift and Rust by copying the bytes instead of allocating.

For example, let's say you have some new type wrappers for different kinds of IDs
within your system.

```
use uuid:Uuid;

#[derive(Copy)]
struct UserId(Uuid);

#[derive(Copy)]
struct OrganizationId(Uuid);
```

You can expose them using:

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Copy(16))]
        type UserId;

        #[swift_bridge(Copy(16))]
        type OrganizationId;
    }
}
```

The `16` indicates that a `UserId` has 16 bytes.

`swift-bridge` will add a compile time assertion that confirms that the given size is correct.

#### #[swift_bridge(Equatable)]

The `Equatable` attribute allows you to expose a Rust `PartialEq` implementation via Swift's
`Equatable` protocol.

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Equatable)]
        type RustPartialEqType;
    }
}

#[derive(PartialEq)]
struct RustPartialEqType(u32);

```

```swift
// In Swift

let val1 = RustPartialEqType(5)
let val2 = RustPartialEqType(10)

if val1 == val2 {
    print("Equal")
} else {
    print("Not equal")
}
```

#### #[swift_bridge(Hashable)]

The `Hashable` attribute allows you to expose a Rust `Hash` trait implementation via Swift's
`Hashable` protocol.

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Hashable)]
        type RustHashType;
    }
}

#[derive(Hash, PartialEq)]
struct RustHashType(u32);
```

```swift
// In Swift

let val = RustHashType(10);

let table: [RustHashType: String] = [:]

table[val] = "hello"
table[val] = "world"

//Should print "world"
print(table[val])
```