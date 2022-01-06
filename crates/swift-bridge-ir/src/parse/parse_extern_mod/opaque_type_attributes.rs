use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

#[derive(Default)]
pub(super) struct OpaqueTypeAttributes {
    pub already_declared: bool,
}

impl OpaqueTypeAttributes {
    pub fn store_attrib(&mut self, attrib: OpaqueTypeAttr) {
        match attrib {
            OpaqueTypeAttr::AlreadyDeclared => self.already_declared = true,
        }
    }
}

pub(super) enum OpaqueTypeAttr {
    AlreadyDeclared,
}

impl Parse for OpaqueTypeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let attrib = match key.to_string().as_str() {
            "already_declared" => OpaqueTypeAttr::AlreadyDeclared,
            _ => panic!("TODO: Return spanned error"),
        };

        Ok(attrib)
    }
}
