# Adding compile time errors

When users write bridge modules that will not compile we want to emit compile time errors that will
guide them towards the right fix.

For example, if a user wrote the following bridge module:

```rust
{{#include ../../../../crates/swift-bridge-macro/tests/ui/unrecognized-opaque-type-attribute.rs:mdbook-ui-test-example}}
```

We would want to emit a compile time error along the lines of:

```sh
{{#include ../../../../crates/swift-bridge-macro/tests/ui/unrecognized-opaque-type-attribute.stderr}}
```

This chapter shows how to add support for compile time errors.

## Implementing Support for a Compile Time Error

To support a new compile time error we first write an automated UI test for the error case.

These tests live in [`crates/swift-bridge-macro/tests/ui`][ui-tests] and are powered by the [trybuild] crate.

After adding our UI test, we create a new `ParseError` variant that can be used to describe the error.

Here are a few example parse errors:

```rust
// via: crates/swift-bridge-ir/src/errors/parse_error.rs

{{#include ../../../../crates/swift-bridge-ir/src/errors/parse_error.rs:mdbook-parse-error-enum}}

    // ...
}
```

After adding a parse error variant, we write the code to generate an error message for the new variant.
Here are a few examples:

````rust
// via: crates/swift-bridge-ir/src/errors/parse_error.rs

{{#include ../../../../crates/swift-bridge-ir/src/errors/parse_error.rs:mdbook-parse-error-message}}

        // ...
    }   
}
````

After adding our `ParseError` we can implement just enough code to make it pass.
This typically happens in `crates/swift-bridge-ir/src/parse.rs`, or one of its descendant modules.

For example, for the given UI test:

```rust
// via: crates/swift-bridge-macro/tests/ui/invalid-module-item.rs

#[swift_bridge::bridge]
mod ffi {
    use std;
    fn foo() {}
}

fn main() {}
```

```sh
# via: crates/swift-bridge-macro/tests/ui/invalid-module-item.stderr

error: Only `extern` blocks, structs and enums are supported.
 --> tests/ui/invalid-module-item.rs:6:5
  |
6 |     use std;
  |     ^^^^^^^^

error: Only `extern` blocks, structs and enums are supported.
 --> tests/ui/invalid-module-item.rs:7:5
  |
7 |     fn foo() {}
  |     ^^^^^^^^^^^
```

We push the `ParseError` error using:

```rust
// via: crates/swift-bridge-ir/src/parse.rs

for outer_mod_item in item_mod.content.unwrap().1 {
    match outer_mod_item {
        Item::ForeignMod(foreign_mod) => {
            // ...
        }
        Item::Struct(item_struct) => {
            // ...
        }
        Item::Enum(item_enum) => {
            // ...
        }
        invalid_item => {
            let error = ParseError::InvalidModuleItem { item: invalid_item };
            errors.push(error);
        }
    };
}
```

[ui-tests]: https://github.com/chinedufn/swift-bridge/tree/master/crates/swift-bridge-macro/tests/ui
[trybuild]: https://github.com/dtolnay/trybuild
