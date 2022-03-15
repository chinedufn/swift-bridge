use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Generics, Token};

pub(crate) struct GenericOpaqueType {
    #[allow(unused)]
    pub type_token: Token![type],
    pub ident: Ident,
    #[allow(unused)]
    pub generics: Generics,
    #[allow(unused)]
    pub semicolon: Token![;],
}

impl Parse for GenericOpaqueType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GenericOpaqueType {
            type_token: input.parse()?,
            ident: input.parse()?,
            generics: input.parse()?,
            semicolon: input.parse()?,
        })
    }
}
