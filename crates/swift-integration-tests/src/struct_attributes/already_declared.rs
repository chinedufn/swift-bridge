//! Verify that the `#[swift_bridge(already_declared)]` module prevents us from emitting the
//! same type definitions twice.

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
        fn rust_reflect_already_declared_struct(
            arg: AlreadyDeclaredStructTest,
        ) -> AlreadyDeclaredStructTest;
    }

    extern "Swift" {
        fn swift_reflect_already_declared_struct(
            arg: AlreadyDeclaredStructTest,
        ) -> AlreadyDeclaredStructTest;
    }
}

fn rust_reflect_already_declared_struct(
    arg: AlreadyDeclaredStructTest,
) -> AlreadyDeclaredStructTest {
    arg
}
