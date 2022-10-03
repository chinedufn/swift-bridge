use crate::bridged_type::BridgedType;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::{Debug, Formatter};
use syn::Type;

#[derive(Debug, PartialEq)]
pub(crate) struct BuiltInPointer {
    pub kind: PointerKind,
    pub pointee: Pointee,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum PointerKind {
    Const,
    Mut,
}

/// The target of an `*const` or `*mut` pointer.
pub(crate) enum Pointee {
    BuiltIn(Box<BridgedType>),
    /// `*const SomeType`
    ///         ^^^^^^^^ This is the Pointee
    Void(Type),
}

impl ToTokens for PointerKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PointerKind::Const => {
                let t = quote! { *const };
                t.to_tokens(tokens);
            }
            PointerKind::Mut => {
                let t = quote! { *mut };
                t.to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for Pointee {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Pointee::BuiltIn(built_in) => {
                built_in.to_rust_type_path().to_tokens(tokens);
            }
            Pointee::Void(ty) => {
                ty.to_tokens(tokens);
            }
        };
    }
}

impl Debug for Pointee {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pointee::BuiltIn(built_in) => f.debug_tuple("BuiltIn").field(&built_in).finish(),
            Pointee::Void(ty) => f
                .debug_tuple("Void")
                .field(&ty.to_token_stream().to_string())
                .finish(),
        }
    }
}

impl PartialEq for Pointee {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BuiltIn(_left), Self::BuiltIn(_right)) => {
                // left == right
                todo!()
            }
            (Self::Void(left), Self::Void(right)) => {
                left.to_token_stream().to_string() == right.to_token_stream().to_string()
            }
            _ => false,
        }
    }
}
