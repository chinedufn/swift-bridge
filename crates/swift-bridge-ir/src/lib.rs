//! An intermediate representation of the FFI layer.
//!
//! Things annotated with the `#[swift_bridge::bridge]` attribute get parsed into this IR.
//!
//! This IR is then used to generate the C header files, Objective-C bridging headers, Swift code,
//! and Rust code needed to power Rust + Swift interop.

#![deny(missing_docs)]

use crate::build_in_types::BuiltInType;
use crate::extern_rust::ExternRustSection;
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
    this: Option<SelfRefMut>,
    func: ParsedExternFn,
    is_initializer: bool,
}

impl TypeMethod {
    fn extern_rust_tokens(&self, ty_declaration: &Ident) -> TokenStream {
        let sig = &self.func.func.sig;
        let fn_name = &sig.ident;

        let export_name = self.func.link_name(ty_declaration);

        let params = self.func.to_rust_param_names_and_types();
        let call_args = self.func.to_rust_call_args();

        let call_fn = quote! {
            #fn_name ( #call_args )
        };

        let inner = if let Some(this) = self.this.as_ref() {
            if let Some(reference) = this.reference {
                let maybe_mut = &this.mutability;

                quote! {
                    let this = unsafe { #reference #maybe_mut *this.ptr };
                    this.#call_fn
                }
            } else {
                todo!()
            }
        } else {
            match &sig.output {
                ReturnType::Default => {
                    quote! {
                        super::#ty_declaration::#call_fn
                    }
                }
                ReturnType::Type(_arrow, ty) => {
                    if BuiltInType::with_type(&ty).is_some() {
                        quote! {
                            super::#ty_declaration::#call_fn
                        }
                    } else {
                        quote! {
                            let val = super::#ty_declaration::#call_fn;
                            let val = Box::into_raw(Box::new(val));
                            swift_bridge::OwnedPtrToRust::new(val)
                        }
                    }
                }
            }
        };

        let prefixed_fn_name = Ident::new(
            &format!("{}_{}", ty_declaration.to_string(), fn_name.to_string()),
            fn_name.span(),
        );

        let output = match &sig.output {
            ReturnType::Default => {
                quote! {}
            }
            ReturnType::Type(arrow, ty) => {
                if let Some(_supported) = BuiltInType::with_type(&ty) {
                    quote! {#arrow #ty}
                } else {
                    quote_spanned! {ty.span()=> -> swift_bridge::OwnedPtrToRust<super::#ty> }
                }
            }
        };

        quote! {
            #[no_mangle]
            #[export_name = #export_name]
            pub extern "C" fn #prefixed_fn_name ( #params ) #output {
                #inner
            }
        }
    }
}

struct ParsedExternFn {
    func: ForeignItemFn,
}

impl ParsedExternFn {
    fn to_rust_param_names_and_types(&self) -> TokenStream {
        let mut params = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(receiver) => {
                    // FIXME: Change tests to not all use SomeType so that this fails...
                    // Needs to be based on  receiver.reference and receiver.mutability..
                    let this = quote! { this: swift_bridge::OwnedPtrToRust<super::SomeType> };
                    params.push(this);
                }
                FnArg::Typed(pat_ty) => {
                    params.push(quote! {#pat_ty});
                }
            };
        }

        quote! {
            #(#params),*
        }
    }

    fn to_swift_param_names_and_types(&self) -> String {
        let mut params: Vec<String> = vec![];

        for arg in &self.func.sig.inputs {
            match arg {
                FnArg::Receiver(receiver) => {
                    // FIXME: Change tests to not all use SomeType so that this fails...
                    // Needs to be based on  receiver.reference and receiver.mutability..
                    // let this = quote! { this: swift_bridge::OwnedPtrToRust<super::SomeType> };
                    // params.push(this);
                }
                FnArg::Typed(pat_ty) => {
                    let arg_name = pat_ty.pat.to_token_stream().to_string();

                    if let Some(built_in) = BuiltInType::with_type(&pat_ty.ty) {
                        params.push(format!("{}: {}", arg_name, built_in.to_swift()));
                    } else {
                        todo!("Add tests for generating functions for unsupported types")
                    };
                }
            };
        }

        params.join(", ")
    }

    // fn foo (&self, arg1: u8, arg2: u32)
    //  becomes..
    // arg1, arg2
    fn to_rust_call_args(&self) -> TokenStream {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => {}
                FnArg::Typed(pat_ty) => {
                    let pat = &pat_ty.pat;
                    args.push(quote! {#pat});
                }
            };
        }

        quote! {
            #(#args),*
        }
    }

    // fn foo (&self, arg1: u8, arg2: u32)
    //  becomes..
    // ptr, arg1, arg2
    fn to_swift_call_args(&self) -> String {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => args.push("ptr".to_string()),
                FnArg::Typed(pat_ty) => {
                    let pat = &pat_ty.pat;
                    args.push(pat.to_token_stream().to_string());
                }
            };
        }

        args.join(", ")
    }

    fn to_swift_return(&self) -> String {
        match &self.func.sig.output {
            ReturnType::Default => "".to_string(),
            ReturnType::Type(_, ty) => {
                if let Some(built_in) = BuiltInType::with_type(&ty) {
                    format!(" -> {}", built_in.to_swift())
                } else {
                    todo!("Handle non built in types")
                }
            }
        }
    }
}

impl ParsedExternFn {
    pub(crate) fn link_name(&self, ty_declaration: &Ident) -> String {
        format!(
            "{}${}${}",
            SWIFT_BRIDGE_PREFIX,
            ty_declaration.to_string(),
            self.func.sig.ident.to_string()
        )
    }
}

pub(crate) struct SelfRefMut {
    pub reference: Option<Token![&]>,
    pub mutability: Option<Token![mut]>,
}

impl From<Receiver> for SelfRefMut {
    fn from(r: Receiver) -> Self {
        SelfRefMut {
            reference: r.reference.map(|r| r.0),
            mutability: r.mutability,
        }
    }
}
