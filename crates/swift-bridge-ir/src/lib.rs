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
use syn::parse::{Parse, ParseStream};
use syn::{ForeignItemType, Path, Type};
use syn::{PatType, Token};

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
    swift_bridge_path: Path,
}

impl SwiftBridgeModule {
    /// Set the path used for `swift_bridge` types such as `swift_bridge::RustString`.
    /// We set this to `crate` when we're inside of the `swift_bridge` crate.
    pub fn set_swift_bridge_path(&mut self, path: Path) {
        self.swift_bridge_path = path;
    }
}

/// `#\[swift_bridge::bridge(swift_bridge_path = swift_bridge\]`
pub struct SwiftBridgeModuleAttrs {
    #[allow(missing_docs)]
    pub attributes: Vec<SwiftBridgeModuleAttr>,
}

/// `#\[swift_bridge::bridge(swift_bridge_path = swift_bridge\]`
pub enum SwiftBridgeModuleAttr {
    /// Sets the path for the actual swift bridge crate that contains different helpers such
    /// as `RustString`.
    SwiftBridgePath(Path),
}

impl Parse for SwiftBridgeModuleAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(SwiftBridgeModuleAttrs { attributes: vec![] });
        }

        let opts = syn::punctuated::Punctuated::<_, Token![,]>::parse_terminated(input)?;

        Ok(SwiftBridgeModuleAttrs {
            attributes: opts.into_iter().collect(),
        })
    }
}

impl Parse for SwiftBridgeModuleAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let _equals = input.parse::<Token![=]>()?;

        let attr = match key.to_string().as_str() {
            "swift_bridge_path" => SwiftBridgeModuleAttr::SwiftBridgePath(input.parse()?),
            _ => {
                return Err(syn::Error::new(input.span(), "Unknown attribute."));
            }
        };

        Ok(attr)
    }
}

#[derive(Clone)]
enum BridgedType {
    Shared(SharedType),
    Opaque(OpaqueForeignType),
}

#[cfg(test)]
impl BridgedType {
    fn unwrap_shared(&self) -> &SharedType {
        match self {
            BridgedType::Shared(s) => s,
            BridgedType::Opaque(_) => panic!(),
        }
    }

    fn unwrap_shared_struct(&self) -> &SharedStruct {
        match self {
            BridgedType::Shared(SharedType::Struct(s)) => s,
            BridgedType::Opaque(_) => panic!(),
        }
    }

    fn unwrap_opaque(&self) -> &OpaqueForeignType {
        match self {
            BridgedType::Shared(_) => {
                panic!()
            }
            BridgedType::Opaque(o) => o,
        }
    }
}

#[derive(Clone)]
enum SharedType {
    Struct(SharedStruct),
}

#[derive(Clone)]
struct SharedStruct {
    name: Ident,
    fields: Vec<StructField>,
}

#[derive(Clone)]
struct StructField {
    name: Option<Ident>,
    ty: Type,
}

#[derive(Clone)]
struct OpaqueForeignType {
    ty: ForeignItemType,
    host_lang: HostLang,
}

impl OpaqueForeignType {
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

/// Whether or not a PatType's pattern is `self`.
///
/// `self: &Fpp` would be true
/// `arg: &Foo` would be false.
fn pat_type_pat_is_self(pat_type: &PatType) -> bool {
    match pat_type.pat.deref() {
        syn::Pat::Ident(pat_ident) if pat_ident.ident == "self" => true,
        _ => false,
    }
}

impl Deref for OpaqueForeignType {
    type Target = ForeignItemType;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn foo() {
        //
    }
}
