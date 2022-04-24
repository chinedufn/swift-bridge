use crate::parse::OpaqueCopy;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::LitInt;

#[derive(Default)]
pub(super) struct OpaqueTypeAttributes {
    pub already_declared: bool,
    pub copy: Option<OpaqueCopy>,
}

impl OpaqueTypeAttributes {
    pub fn store_attrib(&mut self, attrib: OpaqueTypeAttr) {
        match attrib {
            OpaqueTypeAttr::AlreadyDeclared => self.already_declared = true,
            OpaqueTypeAttr::Copy { size } => self.copy = Some(OpaqueCopy { size_bytes: size }),
        }
    }
}

pub(super) enum OpaqueTypeAttr {
    AlreadyDeclared,
    Copy { size: usize },
}

impl Parse for OpaqueTypeAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = OpaqueTypeAttributes::default();

        let punctuated =
            syn::punctuated::Punctuated::<OpaqueTypeAttr, syn::Token![,]>::parse_terminated(input)?;

        for attr in punctuated.into_iter() {
            attributes.store_attrib(attr);
        }

        Ok(attributes)
    }
}

impl Parse for OpaqueTypeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let attrib = match key.to_string().as_str() {
            "already_declared" => OpaqueTypeAttr::AlreadyDeclared,
            // Copy(10)
            "Copy" => {
                let content;
                syn::parenthesized!(content in input);

                let size: LitInt = content.parse()?;
                OpaqueTypeAttr::Copy {
                    // TODO: Return an error if the integer cannot be parsed into a usize.
                    size: size.to_string().parse().unwrap(),
                }
            }
            _ => panic!("TODO: Return spanned error"),
        };

        Ok(attrib)
    }
}
