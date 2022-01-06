# Conditional Compilation

You can use [the cfg attribute](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)
in order to conditionally generate bindings.

Here's an example of only generating some bindings when a "dev-utils" feature is enabled for the Rust crate.

```rust
struct App {
    users: HashMap<u8, Userg>
}

struct User {
    is_logged_in: bool
}

impl App {
	fn new() -> Self {
		App {
		    users: HashMap::new()
		}
	}

	#[cfg(feature = "dev-utils")]
	fn create_logged_in_user(&mut self, user_id: u8);
}

mod ffi {
	extern "Rust" {
	    type App;

        #[swift_bridge(init)]
	    fn new() -> App;

		fn create_logged_in_user(&mut self, user_id: u8)  {
			let user = User {
			    is_logged_in: true
			};
		    self.users.insert(user_id, user)
		}
	}
}

// This example module contains methods that are useful
// during testing and in SwiftUI previews.
// It is only available when the Rust crate is compiled with the "dev-utils" feature.
#[swift_bridge::bridge]
#[cfg(feature = "dev-utils")]
mod ffi_dev_utils {
	extern "Rust" {
	    #[swift_bridge(already_declared)]
        type App;

        fn create_logged_in_user(&mut self, user_id: u8);
	}
}
```

## Supported Conditions

Here are the conditions that are currently supported.

If you need a condition that isn't yet supported, please open an issue.

#### #[cfg(feature = "some-feature")]

```rust
#[swift_bridge]
mod ffi {
    // The code generator will only generate the corresponding
    // Swift and C code if the "extra-utils" feature is enabled.
    #[cfg(feature = "extra-utils")]
	extern "Rust" {
        // ....
    }
}
```

## Locations

Here are the different things that you can conditionally compile.

#### Bridge module

The bridge module can use the `cfg` attribute.

At build time the `swift_bridge_build` library determines whether or not a module will be compiled.

If not, we won't generate any of the corresponding C or Swift code for that module.

```rust
#[swift_bridge::bridge]
// This module's bindings will only be available when the Rust crate is compiled with
// the `ffi-extras` feature
#[cfg(feature = "ffi-extras")]
mod ffi_extras {
  // ...
}
```

#### extern "Rust" blocks

<em>...This hasn't been implemented yet but should be easy...</em>

Functions can methods can use the `#[cfg]` attribute.

```rust
#[swift_bridge::bridge]
mod ffi {
    #[cfg(all(unix, target_pointer_width = "32"))]
	extern "Rust" {
        // ...
    }
}
```


#### Rust functions amd methods

<em>...This hasn't been implemented yet but should be easy...</em>

Functions and methods can use the `#[cfg]` attribute.

```rust
#[swift_bridge::bridge]
mod ffi {
	extern "Rust" {
	    // This function's will only be available when
        // the Rust crate is compiled targetting Windows.
        #[cfg(target_os = "windows")]
	    fn play_solitaire();
    }
}
```
