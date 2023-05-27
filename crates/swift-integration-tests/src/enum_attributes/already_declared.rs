//! Verify that the `#[swift_bridge(already_declared)]` module prevents us from emitting the
//! same type definitions twice.

use self::ffi1::AlreadyDeclaredEnumTest;

#[swift_bridge::bridge]
mod ffi1 {
    enum AlreadyDeclaredEnumTest {
        Variant,
    }
}

#[swift_bridge::bridge]
mod ffi2 {
    #[swift_bridge(already_declared)]
    enum AlreadyDeclaredEnumTest {}

    extern "Rust" {
        fn rust_reflect_already_declared_enum(
            arg: AlreadyDeclaredEnumTest,
        ) -> AlreadyDeclaredEnumTest;
    }

    extern "Rust" {
        fn test_rust_calls_swift_already_declared();
    }

    extern "Swift" {
        fn swift_reflect_already_declared_enum(
            arg: AlreadyDeclaredEnumTest,
        ) -> AlreadyDeclaredEnumTest;
    }
}

fn rust_reflect_already_declared_enum(arg: AlreadyDeclaredEnumTest) -> AlreadyDeclaredEnumTest {
    arg
}
fn test_rust_calls_swift_already_declared() {
    let val = ffi2::swift_reflect_already_declared_enum(AlreadyDeclaredEnumTest::Variant);
    assert!(matches!(val, AlreadyDeclaredEnumTest::Variant));
}
