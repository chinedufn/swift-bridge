use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

#[derive(Default)]
pub(super) struct FunctionAttributes {
    pub associated_to: Option<Ident>,
    pub is_initializer: bool,
    pub rust_name: Option<LitStr>,
    pub swift_name: Option<LitStr>,
    pub into_return_type: bool,
    pub args_into: Option<Vec<Ident>>,
}

impl FunctionAttributes {
    pub fn store_attrib(&mut self, attrib: FunctionAttr) {
        match attrib {
            FunctionAttr::AssociatedTo(ident) => {
                self.associated_to = Some(ident);
            }
            FunctionAttr::Init => self.is_initializer = true,
            FunctionAttr::RustName(name) => {
                self.rust_name = Some(name);
            }
            FunctionAttr::SwiftName(name) => {
                self.swift_name = Some(name);
            }
            FunctionAttr::IntoReturnType => {
                self.into_return_type = true;
            }
            FunctionAttr::ArgsInto(args) => self.args_into = Some(args),
        }
    }
}

pub(super) enum FunctionAttr {
    AssociatedTo(Ident),
    SwiftName(LitStr),
    RustName(LitStr),
    Init,
    IntoReturnType,
    ArgsInto(Vec<Ident>),
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
            "into_return_type" => FunctionAttr::IntoReturnType,
            "rust_name" => {
                input.parse::<Token![=]>()?;
                let value: LitStr = input.parse()?;

                FunctionAttr::RustName(value)
            }
            "args_into" => {
                input.parse::<Token![=]>()?;

                let content;
                syn::parenthesized!(content in input);

                let args = syn::punctuated::Punctuated::<_, Token![,]>::parse_terminated(&content)?;
                FunctionAttr::ArgsInto(args.into_iter().collect())
            }

            _ => panic!("TODO: Return spanned error"),
        };

        Ok(attrib)
    }
}
