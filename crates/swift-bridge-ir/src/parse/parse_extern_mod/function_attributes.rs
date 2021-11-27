use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

#[derive(Default)]
pub(super) struct FunctionAttributes {
    pub associated_to: Option<Ident>,
    pub is_initializer: bool,
    pub swift_name: Option<LitStr>,
}

impl FunctionAttributes {
    pub fn store_attrib(&mut self, attrib: FunctionAttr) {
        match attrib {
            FunctionAttr::AssociatedTo(ident) => {
                self.associated_to = Some(ident);
            }
            FunctionAttr::Init => self.is_initializer = true,
            FunctionAttr::SwiftName(name) => {
                self.swift_name = Some(name);
            }
        }
    }
}

pub(super) enum FunctionAttr {
    AssociatedTo(Ident),
    SwiftName(LitStr),
    Init,
}

impl Parse for FunctionAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let attrib = match key.to_string().as_str() {
            "associated_to" => {
                input.parse::<Token![=]>()?;
                let value: Ident = input.parse()?;

                FunctionAttr::AssociatedTo(value)
            }
            "swift_name" => {
                input.parse::<Token![=]>()?;
                let value: LitStr = input.parse()?;

                FunctionAttr::SwiftName(value)
            }
            "init" => FunctionAttr::Init,
            _ => panic!("TODO: Return spanned error"),
        };

        Ok(attrib)
    }
}
