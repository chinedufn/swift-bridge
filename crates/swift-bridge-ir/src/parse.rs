use crate::errors::ParseErrors;
use crate::parse::parse_extern_mod::ForeignModParser;
use crate::SwiftBridgeModule;
use quote::quote;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::{Item, ItemMod};

mod parse_extern_mod;
mod parse_struct;

impl Parse for SwiftBridgeModule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let module_and_errors: SwiftBridgeModuleAndErrors = input.parse()?;

        module_and_errors.errors.combine_all()?;

        Ok(module_and_errors.module)
    }
}

pub(crate) struct SwiftBridgeModuleAndErrors {
    pub module: SwiftBridgeModule,
    pub errors: ParseErrors,
}

/// The language that a bridge type or function's implementation lives in.
#[derive(Debug, Copy, Clone)]
pub(crate) enum HostLang {
    /// The type or function is defined Rust.
    Rust,
    /// The type or function is defined Swift.
    Swift,
}

impl HostLang {
    pub fn is_rust(&self) -> bool {
        matches!(self, HostLang::Rust)
    }

    pub fn is_swift(&self) -> bool {
        matches!(self, HostLang::Swift)
    }
}

impl Parse for SwiftBridgeModuleAndErrors {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors = ParseErrors::new();

        if let Ok(item_mod) = input.parse::<ItemMod>() {
            let module_name = item_mod.ident;

            let mut functions = vec![];
            let mut all_type_declarations = HashMap::new();

            for outer_mod_item in item_mod.content.unwrap().1 {
                match outer_mod_item {
                    Item::ForeignMod(foreign_mod) => {
                        ForeignModParser {
                            foreign_mod,
                            errors: &mut errors,
                            all_type_declarations: &mut all_type_declarations,
                            functions: &mut functions,
                        }
                        .parse()?;
                    }
                    _ => {
                        //
                        todo!("Push an error that the module may only contain `extern` blocks.")
                    }
                };
            }

            let module = SwiftBridgeModule {
                name: module_name,
                types: all_type_declarations.into_values().collect(),
                functions,
                swift_bridge_path: syn::parse2(quote! { swift_bridge }).unwrap(),
            };
            Ok(SwiftBridgeModuleAndErrors { module, errors })
        } else {
            return Err(syn::Error::new_spanned(
                input.to_string(),
                "Only modules are supported.",
            ));
        }
    }
}
