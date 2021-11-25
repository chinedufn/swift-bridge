use crate::errors::{ParseError, ParseErrors};
use crate::parse::parse_extern_mod::ForeignModParser;
use crate::parse::parse_struct::SharedStructParser;
use crate::parse::type_declarations::TypeDeclarations;
use crate::{BridgedType, SharedType, SwiftBridgeModule};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Item, ItemMod};

mod parse_extern_mod;
mod parse_struct;

mod type_declarations;

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
            let mut all_type_declarations = TypeDeclarations::default();
            let mut maybe_undeclared = vec![];

            for outer_mod_item in item_mod.content.unwrap().1 {
                match outer_mod_item {
                    Item::ForeignMod(foreign_mod) => {
                        ForeignModParser {
                            errors: &mut errors,
                            all_type_declarations: &mut all_type_declarations,
                            functions: &mut functions,
                            maybe_undeclared_types: &mut maybe_undeclared,
                        }
                        .parse(foreign_mod)?;
                    }
                    Item::Struct(item_struct) => {
                        let shared_struct = SharedStructParser {
                            item_struct,
                            errors: &mut errors,
                        }
                        .parse()?;
                        all_type_declarations.insert(
                            shared_struct.name.to_string(),
                            BridgedType::Shared(SharedType::Struct(shared_struct)),
                        );
                    }
                    _ => {
                        todo!(
                            r#"
                        Push an error that the module may only contain `extern` blocks, structs
                        and enums
                        "#
                        )
                    }
                };
            }

            for (ty_name, ty_span) in maybe_undeclared.into_iter() {
                if all_type_declarations.contains_key(&ty_name) {
                    continue;
                }

                errors.push(ParseError::UndeclaredType {
                    ty: ty_name,
                    span: ty_span,
                });
            }

            let types = all_type_declarations
                .order()
                .into_iter()
                .map(|name| all_type_declarations.get(name).unwrap().clone())
                .collect();

            let module = SwiftBridgeModule {
                name: module_name,
                types,
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
