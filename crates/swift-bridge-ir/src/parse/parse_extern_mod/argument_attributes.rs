use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

#[derive(Default)]
pub(super) struct ArgumentAttributes {
    /// Ident: parameter_name, LitStr: argument_name
    pub label: Option<(Ident, LitStr)>,
}

enum ArgumentAttr {
    /// Ident: parameter_name, LitStr: argument_name
    ArgumentLabel((Ident, LitStr)),
}

impl Parse for ArgumentAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = ArgumentAttributes::default();
        let punctuated = syn::punctuated::Punctuated::<ArgumentAttr, syn::Token![,]>::parse_terminated(input)?;
        for attr in punctuated.into_iter() {
            match attr {
                ArgumentAttr::ArgumentLabel(label)=>{
                    attributes.label = Some(label);
                },
            }
        }
        Ok(attributes)
    }
}

impl Parse for ArgumentAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let attribute = match key.to_string().as_str() {
            "label" => {
                input.parse::<Token![=]>()?;
                let value: LitStr = input.parse()?;
                ArgumentAttr::ArgumentLabel((key, value))
            },
            _ => {
                let attrib = key.to_string();
                Err(syn::Error::new_spanned(
                    key,
                    format!(r#"Unrecognized attribute "{}"."#, attrib,),
                ))?
            }
        };
        Ok(attribute)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{parse_ok};
    use quote::{quote, ToTokens};

    #[test]
    fn parse_extern_rust_argument_attributes() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function(
                        #[swift_bridge(label = "argumentLabel1")] parameter_name1: i32,
                        parameter_name2: String,
                    );
                }
            }
        };

        let module = parse_ok(tokens);
        assert!(module.functions.len() == 1);
        assert!(module.functions[0].argument_labels.is_some());
        if let Some(argument_labels) = &module.functions[0].argument_labels {
            assert_eq!(argument_labels.len(), 1);
            assert_eq!(argument_labels[0].0.pat.to_token_stream().to_string(), "parameter_name1");
        }
    }
}
