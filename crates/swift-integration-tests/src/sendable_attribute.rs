//! See `crates/swift-bridge-ir/src/codegen/codegen_tests/sendable_attribute.rs` for codegen tests.

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Sendable)]
        type SendableRustType;
        #[swift_bridge(init)]
        fn new() -> SendableRustType;

        // Verify that our generated code does not emit any duplicate symbols.
        //
        // We accomplish this by defining this second `Sendable` type.
        // If our code compiles then we know there weren't any duplicate codegen.
        #[swift_bridge(Sendable)]
        type AnotherSendableRustType;
    }

    extern "Swift" {
        #[swift_bridge(Sendable)]
        type SendableSwiftType;

        // Verify that our generated code does not emit any duplicate symbols.
        //
        // We accomplish this by defining this second `Sendable` type.
        // If our code compiles then we know there weren't any duplicate codegen.
        #[swift_bridge(Sendable)]
        type AnotherSendableSwiftType;
    }
}

struct SendableRustType;
impl SendableRustType {
    fn new() -> Self {
        Self
    }
}

struct AnotherSendableRustType;

const fn assert_send_sync<T: Send + Sync>() {}

// Verify that the `SendableSwiftType` implements `Send + Sync`.
#[allow(unused)]
const TEST_SENDABLE_SWIFT_TYPE_SEND_SYNC: () = {
    assert_send_sync::<ffi::SendableSwiftType>();
};
