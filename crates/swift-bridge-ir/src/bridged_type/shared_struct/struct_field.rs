use proc_macro2::{Ident, TokenStream};
use quote::{quote_spanned, ToTokens};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::Type;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StructFields {
    Named(Vec<NamedStructField>),
    Unnamed(Vec<UnnamedStructField>),
    Unit,
}

impl StructFields {
    /// Returns true if the struct does not have any named or unnamed fields.
    pub fn is_empty(&self) -> bool {
        match self {
            StructFields::Named(named) => named.is_empty(),
            StructFields::Unnamed(unnamed) => unnamed.is_empty(),
            StructFields::Unit => true,
        }
    }
}

#[derive(Clone)]
pub(crate) struct NamedStructField {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Clone)]
pub(crate) struct UnnamedStructField {
    pub ty: Type,
    pub idx: usize,
}

impl NamedStructField {
    pub fn swift_name_string(&self) -> String {
        self.name.to_string()
    }
}

impl UnnamedStructField {
    pub fn swift_name_string(&self) -> String {
        format!("_{}", self.idx)
    }

    pub fn rust_field_accessor(&self) -> TokenStream {
        let idx = format!("{}", self.idx);
        let idx = TokenStream::from_str(&idx).unwrap();
        quote_spanned! {self.ty.span()=> #idx}
    }

    pub fn ffi_field_name(&self) -> Ident {
        Ident::new(&format!("_{}", self.idx), self.ty.span())
    }
}

impl PartialEq for NamedStructField {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_string() == other.name.to_string()
            && self.ty.to_token_stream().to_string() == other.ty.to_token_stream().to_string()
    }
}

impl Debug for NamedStructField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NamedStructField")
            .field("name", &self.name.to_string())
            .field("ty", &self.ty.to_token_stream())
            .finish()
    }
}

impl PartialEq for UnnamedStructField {
    fn eq(&self, other: &Self) -> bool {
        self.ty.to_token_stream().to_string() == other.ty.to_token_stream().to_string()
            && self.idx == other.idx
    }
}

impl Debug for UnnamedStructField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnnamedStructField")
            .field("ty", &self.ty.to_token_stream())
            .field("idx", &self.idx)
            .finish()
    }
}
