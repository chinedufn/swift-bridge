# The Bridge Module

With `swift-bridge` you use a "bridge module" in order to declare your FFI interface.

```rust
#[swift_bridge::bridge]
mod ffi {
    // Export Rust types and functions for Swift to use.
    extern "Rust" {
        type SomeRustType;
        fn some_type_method(&mut self) -> String;
    }

    // Import Swift types and functions for Swift to use.
    extern "Swift" {
        type SomeSwiftClass;

        #[swift_bridge(swift_name = "someClassMethod")]
        fn some_class_method(&self, arg: u8);
    }
}
```

Your bridge module can contain any number of `extern "Rust"` and `extern "Swift"` blocks, each declaring types
and functions to expose to and import from Swift, respectively.

## How it Works

After you declare your bridge module, you use two code generators at build time to make the FFI layer
that you described work.

One code generator generates the Rust side of the FFI layer, and the other code generator produces the Swift side.

#### Rust code generation

The `#[swift_bridge::bridge]` procedural macro parses your bridge module at compile time and then
 generates the Rust side of your FFI layer.

#### Swift code generation

At build time you run `swift-bridge-build` (or `swift-bridge-cli` for non-Cargo based setups) on files that contain
bridge modules in order to generate the `Swift` and `C` code necessary to make your bridge work.

## Let's Begin

This section's sub chapters will go into detail about the different ways that you can use bridge modules to
connect Rust and Swift.

In the meantime, here's a quick peak of a simple bridge module:

{{ #include ../../../README.md:bridge-module-example }}
