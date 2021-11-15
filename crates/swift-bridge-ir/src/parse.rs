use crate::errors::ParseErrors;
use crate::SwiftBridgeModule;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Parser};

impl Parse for SwiftBridgeModule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // let mut errors = Errors::new();
        // errors.take_all()?;

        todo!()
    }
}

struct SwiftBridgeModuleParser;

impl Parser for SwiftBridgeModuleParser {
    type Output = (SwiftBridgeModule, ParseErrors);

    fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ParseError;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::parse::Parser;
    use syn::parse_quote;

    /// Verify that we can parse a SwiftBridgeModule from an empty module.
    #[test]
    fn parse_empty_module() {
        let tokens = quote! {
            mod foo { }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);

        assert_eq!(module.name.to_string(), "foo");
    }

    /// Verify that we store an error if no abi name was provided.
    #[test]
    fn error_if_no_abi_name_provided_for_an_extern_block() {
        let tokens = quote! {
            mod foo {
                extern {}
            }
        };
        let errors = parse_errors(tokens);

        assert_eq!(errors.len(), 1);
        match errors[0] {
            ParseError::AbiNameMissing { .. } => {}
            _ => panic!(),
        }
    }

    /// Verify that we store an error if the abi name isn't Rust or Swift.
    #[test]
    fn error_if_invalid_abi_name() {
        let tokens = quote! {
            mod foo {
                extern "SomeAbi" {}
            }
        };
        let errors = parse_errors(tokens);

        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ParseError::AbiNameInvalid { abi_name } => {
                assert_eq!(abi_name.value(), "SomeAbi");
            }
            _ => panic!(),
        }
    }

    /// Verify that we can parse a Rust type declaration.
    #[test]
    fn rust_type_declaration() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.extern_rust[0].types[0].ty.ident.to_string(), "Foo");
    }

    /// Verify that we can parse a Rust type's methods.
    /// We test all of the possible ways we can specify self.
    #[test]
    fn parses_rust_self_methods() {
        let tests = vec![
            quote! { fn bar (self); },
            quote! { fn bar (&self); },
            quote! { fn bar (&mut self); },
            quote! { fn bar (self: Foo); },
            quote! { fn bar (self: &Foo); },
            quote! { fn bar (self: &mut Foo); },
            quote! {
                #[swift_bridge(associated_to = Foo)]
                fn bar ();
            },
        ];

        for fn_definition in tests {
            let tokens = quote! {
                mod foo {
                    extern "Rust" {
                        type Foo;

                        #fn_definition;
                    }
                }
            };

            let module = parse_ok(tokens);

            let ty = &module.extern_rust[0].types[0];
            assert_eq!(ty.ty.ident.to_string(), "Foo");

            assert_eq!(
                ty.methods.len(),
                1,
                "Failed not parse {} into an associated method.",
                quote! {#fn_definition}.to_string()
            );
        }
    }

    /// Verify that we can parse a freestanding Rust function declaration.
    #[test]
    fn parse_rust_freestanding_function() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn bar () -> u8;
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.extern_rust[0].free_functions.len(), 1);
    }

    /// Verify that if an extern Rust block has more than one type, we push errors for any methods
    /// that have an ambiguous self.
    #[test]
    fn error_if_method_has_ambiguous_self() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    type AnotherType;

                    fn a (self);
                    fn b (&self);
                    fn c (&mut self);
                }
            }
        };

        let errors = parse_errors(tokens);

        assert_eq!(errors.len(), 3);

        let selfs = vec!["self", "&self", "&mut self"];
        for (idx, expected_self) in selfs.into_iter().enumerate() {
            match &errors[idx] {
                ParseError::AmbiguousSelf { self_ident } => {
                    assert_eq!(self_ident.to_string(), expected_self);
                }
                _ => panic!(),
            };
        }
    }

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        SwiftBridgeModuleParser.parse2(tokens).unwrap().0
    }

    fn parse_errors(tokens: TokenStream) -> ParseErrors {
        SwiftBridgeModuleParser.parse2(tokens).unwrap().1
    }
}
