//! An intermediate representation of the FFI layer.
//!
//! Things annotated with the `#[swift_bridge::bridge]` attribute get parsed into this IR.
//!
//! This IR is then used to generate the C header files, Objective-C bridging headers, Swift code,
//! and Rust code needed to power Rust + Swift interop.

#![deny(missing_docs)]

use proc_macro2::Ident;
use syn::{ForeignItemType, Token, Type};

mod extern_rust;

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
    extern_rusts: Vec<ExternRustSection>,
}

struct ExternRustSection {
    types: Vec<ExternRustSectionType>,
    free_functions: Vec<ExternRustSectionFn>,
}

struct ExternRustSectionType {
    ty: ForeignItemType,
    funcs: Vec<TypeFunction>,
}

struct FunctionOnType {
    ty: ForeignItemType,
}

/// A method or static method associated with a type.
struct TypeFunction {
    ty: ForeignItemType,
    self_: Option<MethodSelf>,
    func: ExternRustSectionFn,
}

struct ExternRustSectionFn {
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
