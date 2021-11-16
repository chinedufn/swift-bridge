//! An intermediate representation of the FFI layer.
//!
//! Things annotated with the `#[swift_bridge::bridge]` attribute get parsed into this IR.
//!
//! This IR is then used to generate the C header files, Objective-C bridging headers, Swift code,
//! and Rust code needed to power Rust + Swift interop.

#![deny(missing_docs)]

use crate::extern_rust::ExternRustSection;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{ForeignItemFn, PatType, Receiver, ReturnType, Token, Type};

mod extern_rust;
mod extern_swift;

mod errors;
mod parse;
mod to_tokens;

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
    this: Option<SelfDesc>,
    func: ExternFn,
}

impl TypeMethod {
    /// ty_declaration is the `SomeType` in `type SomeType`
    fn rust_tokens(&self, ty_declaration: &Ident) -> TokenStream {
        let sig = &self.func.func.sig;
        let fn_name = &sig.ident;

        let export_name = format!(
            "{}${}${}",
            SWIFT_BRIDGE_PREFIX,
            ty_declaration.to_string(),
            fn_name.to_string()
        );

        let this = self.this.as_ref().map(|_| {
            quote! { this: swift_bridge::OwnedPtrToRust<super::SomeType>, }
        });

        let inner = if let Some(this) = self.this.as_ref() {
            if let Some(reference) = this.reference {
                let maybe_mut = &this.mutability;

                quote! {
                    let this = unsafe { #reference #maybe_mut *this.ptr };
                    this.#fn_name()
                }
            } else {
                todo!()
            }
        } else {
            quote! {
                let val = super::#ty_declaration::#fn_name();
                let val = Box::into_raw(Box::new(val));
                swift_bridge::OwnedPtrToRust::new(val)
            }
        };

        let prefixed_fn_name = Ident::new(
            &format!("{}_{}", ty_declaration.to_string(), fn_name.to_string()),
            fn_name.span(),
        );

        let output = match sig.output.clone() {
            ReturnType::Default => {
                quote! {}
            }
            ReturnType::Type(_arrow, ty) => {
                quote_spanned! {ty.span()=> -> swift_bridge::OwnedPtrToRust<super::#ty> }
            }
        };

        quote! {
            #[no_mangle]
            #[export_name = #export_name]
            pub extern "C" fn #prefixed_fn_name (#this ) #output {
                #inner
            }
        }
    }
}

struct ExternFn {
    func: ForeignItemFn,
}

// &self
// &mut self
// self: &Receiver
// self: &mutReceiver
// self
pub(crate) struct SelfDesc {
    pub reference: Option<Token![&]>,
    pub mutability: Option<Token![mut]>,
}

impl From<Receiver> for SelfDesc {
    fn from(r: Receiver) -> Self {
        SelfDesc {
            reference: r.reference.map(|r| r.0),
            mutability: r.mutability,
        }
    }
}
