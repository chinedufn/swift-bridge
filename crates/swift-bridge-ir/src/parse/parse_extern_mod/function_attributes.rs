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

impl Parse for FunctionAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = FunctionAttributes::default();

        let punctuated =
            syn::punctuated::Punctuated::<FunctionAttr, syn::Token![,]>::parse_terminated(input)?;

        for attr in punctuated.into_iter() {
            attributes.store_attrib(attr);
        }

        Ok(attributes)
    }
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

#[cfg(test)]
mod tests {
    use crate::errors::ParseError;
    use crate::test_utils::{parse_errors, parse_ok};
    use quote::{quote, ToTokens};

    /// Verify that we can parse the into_return_type attribute from extern "Rust" blocks.
    #[test]
    fn parse_extern_rust_into_return_type_attribute() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(into_return_type)]
                    fn some_function () -> Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        assert!(module.functions[0].into_return_type);
    }

    /// Verify that we can parse an associated function.
    #[test]
    fn parse_associated_function() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn bar ();
                }
            }
        };

        let module = parse_ok(tokens);

        let ty = &module.types.types()[0].unwrap_opaque();
        assert_eq!(ty.ident.to_string(), "Foo");

        assert_eq!(module.functions.len(), 1,);
    }

    /// Verify that we can parse an associated function that has arguments.
    #[test]
    fn associated_function_with_args() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn bar (arg: u8);
                }
            }
        };

        let module = parse_ok(tokens);

        let ty = &module.types.types()[0].unwrap_opaque();
        assert_eq!(ty.ident.to_string(), "Foo");

        assert_eq!(module.functions.len(), 1,);
    }

    /// Verify that we can parse an init function.
    #[test]
    fn initializer() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn bar () -> Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        let func = &module.functions[0];
        assert!(func.is_initializer);
    }

    /// Verify that we can parse an init function that takes inputs.
    #[test]
    fn initializer_with_inputs() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn bar (bazz: u8) -> Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        let func = &module.functions[0];
        assert!(func.is_initializer);
    }

    /// Verify that we push an error if the initialize type is not defined.
    #[test]
    fn error_if_initialized_type_not_defined() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    #[swift_bridge(init)]
                    fn bar () -> Foo;
                }
            }
        };

        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 1);

        match &errors[0] {
            ParseError::UndeclaredType { ty } => {
                assert_eq!(ty.to_token_stream().to_string(), "Foo")
            }
            _ => panic!(),
        }
    }

    /// Verify that we can parse a from attribute for a struct.
    #[test]
    fn parses_extern_rust_args_into_attribute() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(args_into = (some_arg, another_arg))]
                    fn some_function(some_arg: u8, another_arg: u16);
                }
            }
        };

        let module = parse_ok(tokens);

        let func = &module.functions[0];

        let args_into = func.args_into.as_ref().unwrap();
        assert_eq!(args_into.len(), 2);

        let assert_arg_into = |arg_name: &str| {
            assert!(args_into
                .iter()
                .find(|arg| { &arg.to_string() == arg_name })
                .is_some());
        };

        assert_arg_into("some_arg");
        assert_arg_into("another_arg");
    }

    /// Verify that we can parse a function that has multiple swift_bridge attributes.
    #[test]
    fn parses_multiple_function_swift_bridge_attributes() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(args_into = (a), into_return_type)]
                    fn some_function(a: u8);
                }
            }
        };

        let module = parse_ok(tokens);

        let func = &module.functions[0];
        assert_eq!(func.args_into.as_ref().unwrap().len(), 1);
        assert_eq!(func.into_return_type, true);
    }
}
