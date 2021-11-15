use crate::module::{AbiName, ExternRustItem, Module, ModuleParseError, ModuleSection};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{ForeignItem, Item, ItemMod};

impl Parse for Module {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_mod: ItemMod = input.parse()?;

        let mut errors = vec![];

        let sections = if let Some((_brace, contents)) = item_mod.content {
            let mut sections = vec![];

            for content in contents {
                match content {
                    Item::ForeignMod(foreign) => {
                        if foreign.abi.name.is_none() {
                            errors.push(ModuleParseError::AbiMissingName {
                                foreign_module_span: foreign.span(),
                            });
                            continue;
                        }

                        let abi_name = foreign.abi.name.unwrap();
                        let abi_name = match abi_name.value().as_str() {
                            "Rust" => AbiName::Rust,
                            "Swift" => AbiName::Swift,
                            _ => {
                                errors.push(ModuleParseError::AbiNameInvalid { abi_name });
                                continue;
                            }
                        };

                        match abi_name {
                            AbiName::Rust => {
                                let mut items = vec![];

                                for item in foreign.items {
                                    match item {
                                        ForeignItem::Type(ty) => {
                                            items.push(ExternRustItem::TypeDeclaration(ty));
                                        }
                                        ForeignItem::Fn(func) => {
                                            items.push(ExternRustItem::Func(func));
                                        }
                                        _ => {}
                                    }
                                }

                                sections.push(ModuleSection::ExternRust(items));
                            }
                            AbiName::Swift => {
                                sections.push(ModuleSection::ExternSwift);
                            }
                        };
                    }
                    _ => panic!("Return an error"),
                }
            }

            sections
        } else {
            vec![]
        };

        let module = Module {
            name: item_mod.ident,
            sections,
            errors,
        };

        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    /// Verify that we can parse an empty module.
    #[test]
    fn parses_empty_module() {
        let tokens = quote! {
            mod foo { }
        };
        let module: Module = parse_quote!(#tokens);

        assert_eq!(module.name.to_string(), "foo");
    }

    /// Verify that we store an error if an extern does not have an ABI name.
    #[test]
    fn error_if_no_abi_name_provided() {
        let tokens = quote! {
            mod foo {
                extern { }
            }
        };
        let module: Module = parse_quote!(#tokens);

        assert_eq!(module.errors.len(), 1);
        match module.errors[0] {
            ModuleParseError::AbiMissingName {
                foreign_module_span: _,
            } => {}
            _ => panic!(),
        };
    }

    /// Verify that we store an error if the ABI name is not Rust or Swift.
    #[test]
    fn error_if_abi_name_invalid() {
        let tokens = quote! {
            mod foo {
                extern "Foo" { }
            }
        };
        let module: Module = parse_quote!(#tokens);

        assert_eq!(module.errors.len(), 1);
        match &module.errors[0] {
            ModuleParseError::AbiNameInvalid {
                abi_name: abi_name_ident,
            } => {
                assert_eq!(abi_name_ident.value(), "Foo")
            }
            _ => panic!(),
        };
    }

    /// Verify that we can parse an empty extern "Rust" block.
    #[test]
    fn parse_empty_extern_rust_block() {
        let tokens = quote! {
            mod foo {
                extern "Rust" { }
            }
        };
        let module: Module = parse_quote!(#tokens);

        match &module.sections[0] {
            ModuleSection::ExternRust(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!(),
        };
    }

    /// Verify that we can parse an empty extern "Swift" block.
    #[test]
    fn parse_empty_extern_swift_block() {
        let tokens = quote! {
            mod foo {
                extern "Swift" { }
            }
        };
        let module: Module = parse_quote!(#tokens);

        match &module.sections[0] {
            ModuleSection::ExternSwift => {}
            _ => panic!(),
        };
    }

    /// Verify that we can parse one Rust type declaration.
    #[test]
    fn rust_type_declaration() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;
                }
            }
        };
        let module: Module = parse_quote!(#tokens);

        match module.sections[0] {
            ModuleSection::ExternRust(ref items) => {
                assert_eq!(items.len(), 1);

                match items[0] {
                    ExternRustItem::TypeDeclaration(ref ty) => {
                        assert_eq!(ty.ident.to_string(), "Foo");
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        };
    }

    /// Verify that we can parse an extern fn definition within a Rust foreign module block.
    #[test]
    fn rust_extern_fn_declaration() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn new () -> u8;
                }
            }
        };
        let module: Module = parse_quote!(#tokens);

        match module.sections[0] {
            ModuleSection::ExternRust(ref items) => {
                assert_eq!(items.len(), 1);

                match items[0] {
                    ExternRustItem::Func(ref func) => {
                        assert_eq!(func.sig.ident, "new");
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        };
    }
}
