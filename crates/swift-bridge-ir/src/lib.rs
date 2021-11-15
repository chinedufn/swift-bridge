//! An intermediate representation of the FFI layer.
//!
//! Things annotated with the `#[swift_bridge::bridge]` attribute get parsed into this IR.
//!
//! This IR is then used to generate the C header files, Objective-C bridging headers, Swift code,
//! and Rust code needed to power Rust + Swift interop.

#![deny(missing_docs)]

use crate::extern_rust::ExternRustSection;
use proc_macro2::Ident;
use syn::{ForeignItemType, Token, Type};

mod extern_rust;
mod extern_swift;

mod errors;
mod parse;

#[cfg(test)]
mod test_utils;

/// Represents a type definition within an `extern "Rust"` module, as well as all of its methods.
///
/// ```no_run,ignore
/// #[swift_bridge::bridge]
/// mod ffi {
///     extern "Rust" {
///         type Stack;
///
///         fn push(&mut self, val: u8);
///
///         fn pop(self: &mut Stack) -> Option<u8>;
///
///         fn as_ptr(&self) -> *const u8;
///
///         fn len(self: &Stack) -> usize;
///
///         fn consume(self);
///     }
///
///     extern "Swift" {
///         // TODO: Examples
///     }
/// }
/// ```
struct SwiftBridgeModule {
    name: Ident,
    extern_rust: Vec<ExternRustSection>,
}

/// A method or static method associated with a type.
///
/// fn bar (&self);
/// fn buzz (self: &Foo) -> u8;
///
/// #\[swift_bridge(associated_to = Foo)]
/// fn new () -> Foo;
///
/// ... etc
struct TypeMethod {
    this: Option<MethodSelf>,
    func: ExternFn,
}

struct ExternFn {
    fn_name: Ident,
    args: Vec<(Ident, Type)>,
    ret: Type,
}

// &self
// &mut self
// self: &Receiver
// self: &mutReceiver
// self
struct MethodSelf {
    reference: Option<Token![&]>,
    mutability: Option<Token![mut]>,
    receiver: ForeignItemType,
}
