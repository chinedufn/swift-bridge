# extern "Rust"

`extern "Rust` sections are used to expose Rust types and functions over FFI so that they can used from Swift code.

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

## Function Attributes

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
