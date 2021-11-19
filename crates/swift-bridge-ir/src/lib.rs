//! An intermediate representation of the FFI layer.
//!
//! Things annotated with the `#[swift_bridge::bridge]` attribute get parsed into this IR.
//!
//! This IR is then used to generate the C header files, Objective-C bridging headers, Swift code,
//! and Rust code needed to power Rust + Swift interop.

#![deny(missing_docs)]

use crate::parse::HostLang;
use proc_macro2::Ident;
use std::ops::Deref;
use syn::{ForeignItemType, Path};

use crate::parsed_extern_fn::ParsedExternFn;

mod errors;
mod parse;
mod to_tokens;

mod built_in_types;
mod parsed_extern_fn;

mod codegen;

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
    types: Vec<BridgedType>,
    functions: Vec<ParsedExternFn>,
}

#[derive(Clone)]
struct BridgedType {
    ty: ForeignItemType,
    host_lang: HostLang,
}

impl BridgedType {
    // "__swift_bridge__$TypeName$_free"
    fn free_link_name(&self) -> String {
        format!(
            "{}${}$_free",
            SWIFT_BRIDGE_PREFIX,
            self.ty.ident.to_string()
        )
    }

    // "__swift_bridge__TypeName__free"
    fn free_func_name(&self) -> String {
        format!("{}{}__free", SWIFT_BRIDGE_PREFIX, self.ty.ident.to_string())
    }

    fn ty_name_ident(&self) -> &Ident {
        &self.ty.ident
    }
}

impl Deref for BridgedType {
    type Target = ForeignItemType;

    fn deref(&self) -> &Self::Target {
        &self.ty
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
