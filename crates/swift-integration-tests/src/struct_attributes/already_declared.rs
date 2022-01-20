//! Verify that the `#[swift_bridge(already_declared)]` module prevents us from emitting the
//! same type definitions twice.
//!
//! If the Xcode project is able to compile then we know that our attribute works,
//! because otherwise we would get build time errors that the class was defined twice.

use self::ffi1::AlreadyDeclaredStructTest;

#[swift_bridge::bridge]
mod ffi1 {
    #[swift_bridge(swift_repr = "struct")]
    struct AlreadyDeclaredStructTest {
        field: u8,
    }
}

#[swift_bridge::bridge]
mod ffi2 {
    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct AlreadyDeclaredStructTest;

    extern "Rust" {
        fn reflect_already_declared_struct(
            arg: AlreadyDeclaredStructTest,
        ) -> AlreadyDeclaredStructTest;
    }
}

fn reflect_already_declared_struct(arg: AlreadyDeclaredStructTest) -> AlreadyDeclaredStructTest {
    arg
}
