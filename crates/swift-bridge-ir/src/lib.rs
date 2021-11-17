//! An intermediate representation of the FFI layer.
//!
//! Things annotated with the `#[swift_bridge::bridge]` attribute get parsed into this IR.
//!
//! This IR is then used to generate the C header files, Objective-C bridging headers, Swift code,
//! and Rust code needed to power Rust + Swift interop.

#![deny(missing_docs)]

use crate::build_in_types::BuiltInType;
use crate::extern_rust::ExternRustSection;
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{FnArg, ForeignItemFn, Receiver, ReturnType, Token};

mod extern_rust;
mod extern_swift;

mod errors;
mod parse;
mod to_tokens;

mod build_in_types;
mod parsed_extern_fn;

#[cfg(test)]
mod test_utils;

const SWIFT_BRIDGE_PREFIX: &'static str = "__swift_bridge__";

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
pub struct SwiftBridgeModule {
    name: Ident,
    extern_rust: Vec<ExternRustSection>,
}

impl SwiftBridgeModule {
    /// Generate the contents of a Swift file based on the contents of this module.
    pub fn generate_swift(&self) -> String {
        let mut swift = "".to_string();

        for section in &self.extern_rust {
            swift += &section.generate_swift();
        }

        swift
    }

    /// Generate the contents of a C header file based on the contents of this module.
    pub fn generate_c_header(&self) -> String {
        let mut c_header = "".to_string();

        for section in &self.extern_rust {
            c_header += &section.generate_c_header();
        }

        c_header
    }
}

/// A method or associated function associated with a type.
///
/// fn bar (&self);
/// fn buzz (self: &Foo) -> u8;
///
/// #\[swift_bridge(associated_to = Foo)]
/// fn new () -> Foo;
///
/// ... etc
struct TypeMethod {
    func: ParsedExternFn,
    is_initializer: bool,
}

impl TypeMethod {
    fn extern_rust_tokens(&self, ty_declaration: &Ident) -> TokenStream {
        self.func
            .to_extern_rust_function_tokens(Some(ty_declaration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        //
    }
}
