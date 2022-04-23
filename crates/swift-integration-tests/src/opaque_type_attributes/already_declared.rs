//! Verify that the `#[swift_bridge(already_declared)]` module prevents us from emitting the
//! same type definitions twice.
//!
//! If the Xcode project is able to compile then we know that our attribute works,
//! because otherwise we would get build time errors that the class was defined twice.

#[swift_bridge::bridge]
mod ffi1 {
    extern "Rust" {
        type AlreadyDeclaredTypeTest;
        #[swift_bridge(Copy(4))]
        type AlreadyDeclaredCopyTypeTest;
    }
}

#[swift_bridge::bridge]
mod ffi2 {
    extern "Rust" {
        #[swift_bridge(already_declared)]
        type AlreadyDeclaredTypeTest;

        #[swift_bridge(init)]
        fn new() -> AlreadyDeclaredTypeTest;

        fn an_owned_method(self) -> bool;
        fn a_ref_method(&self) -> bool;
        fn a_ref_mut_method(&mut self) -> bool;

        #[swift_bridge(associated_to = AlreadyDeclaredTypeTest)]
        fn an_associated_function() -> bool;
    }

    extern "Rust" {
        #[swift_bridge(already_declared, Copy(4))]
        type AlreadyDeclaredCopyTypeTest;

        #[swift_bridge(init)]
        fn new() -> AlreadyDeclaredCopyTypeTest;

        fn an_owned_method(self) -> bool;
        fn a_ref_method(&self) -> bool;

        #[swift_bridge(associated_to = AlreadyDeclaredCopyTypeTest)]
        fn an_associated_function() -> bool;
    }
}

pub struct AlreadyDeclaredTypeTest;

#[derive(Copy, Clone)]
pub struct AlreadyDeclaredCopyTypeTest([u16; 2]);
impl AlreadyDeclaredTypeTest {
    fn new() -> Self {
        AlreadyDeclaredTypeTest
    }

    fn an_owned_method(self) -> bool {
        true
    }

    fn a_ref_method(&self) -> bool {
        true
    }

    fn a_ref_mut_method(&mut self) -> bool {
        true
    }

    fn an_associated_function() -> bool {
        true
    }
}

impl AlreadyDeclaredCopyTypeTest {
    fn new() -> Self {
        AlreadyDeclaredCopyTypeTest([1, 2])
    }

    fn an_owned_method(self) -> bool {
        true
    }

    fn a_ref_method(&self) -> bool {
        true
    }

    fn an_associated_function() -> bool {
        true
    }
}
