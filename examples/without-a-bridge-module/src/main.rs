fn main() {
    // TODO: Use the `swift-bridge-ir` crate to generate a representation of the FFI
    //  boundary.
    //  Then use that representation to generate Rust, Swift and C code.
    //  Then write that code to a temporary directory and spawn a process to compile and run
    //  the generated code.
    //  Today, `swift-bridge-ir` has the `SwiftBridgeModule` type which represents a bridge module.
    //  One solution would be to create a new `RustSwiftFfiDefinition` type that holds the minimum
    //  information required to define an FFI boundary, and then change `swift-bridge-ir` from:
    //  - TODAY  -> `SwiftBridgeModule` gets converted into Rust+Swift+C Code
    //  - FUTURE -> `SwiftBridgeModule` gets converted into `RustSwiftFfiDefinition` which gets
    //    converted into Rust+Swift+C Code
    //  After that we can make this `without-a-bridge-module` example make use of the
    //  `RustSwiftFfiDefinition` to generate some Rust+Swift+C FFI glue code.
    //  ---
    //  If you are reading this and would like to wrap `swift-bridge-ir` in your own library please
    //  open an issue so that we know when and how to prioritize this work.
}
