use crate::bridged_type::{EnumVariant, SharedEnum, StructFields};
use crate::errors::{ParseError, ParseErrors};
use crate::parse::move_input_cursor_to_next_comma;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::ItemEnum;

pub(crate) struct SharedEnumDeclarationParser<'a> {
    pub item_enum: ItemEnum,
    // Will be used in a future commit..
    #[allow(unused)]
    pub errors: &'a mut ParseErrors,
}

enum EnumAttr {
    AlreadyDeclared,
    Error(EnumAttrParseError),
}

enum EnumAttrParseError {
    UnrecognizedAttribute(Ident),
}

#[derive(Default)]
struct EnumAttribs {
    already_declared: bool,
}

struct ParsedAttribs(Vec<EnumAttr>);
impl Parse for ParsedAttribs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(ParsedAttribs(vec![]));
        }

        let opts = syn::punctuated::Punctuated::<_, syn::token::Comma>::parse_terminated(input)?;

        Ok(ParsedAttribs(opts.into_iter().collect()))
    }
}

impl Parse for EnumAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let attr = match key.to_string().as_str() {
            "already_declared" => EnumAttr::AlreadyDeclared,
            _ => {
                move_input_cursor_to_next_comma(input);
                EnumAttr::Error(EnumAttrParseError::UnrecognizedAttribute(key))
            }
        };

        Ok(attr)
    }
}

impl<'a> SharedEnumDeclarationParser<'a> {
    pub fn parse(self) -> Result<SharedEnum, syn::Error> {
        let item_enum = self.item_enum;

        let mut variants = vec![];

        let mut attribs = EnumAttribs::default();
        for attr in item_enum.attrs {
            let sections: ParsedAttribs = attr.parse_args()?;

            for attr in sections.0 {
                match attr {
                    EnumAttr::AlreadyDeclared => {
                        attribs.already_declared = true;
                    }
                    EnumAttr::Error(err) => match err {
                        EnumAttrParseError::UnrecognizedAttribute(attribute) => {
                            self.errors
                                .push(ParseError::EnumUnrecognizedAttribute { attribute });
                        }
                    },
                }
            }
        }

        for v in item_enum.variants {
            let variant = EnumVariant {
                name: v.ident,
                fields: StructFields::from_syn_fields(v.fields),
            };
            variants.push(variant);
        }

        let shared_enum = SharedEnum {
            name: item_enum.ident,
            variants,
            already_declared: attribs.already_declared,
        };

        Ok(shared_enum)
    }
}

#[cfg(test)]
mod tests {
    use crate::bridged_type::StructFields;
    use crate::errors::ParseError;
    use crate::test_utils::{parse_errors, parse_ok};
    use quote::{quote, ToTokens};

    /// Verify that we can parse an enum with no variants.
    #[test]
    fn parse_enum_no_variants() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum SomeEnum {
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.types.types().len(), 1);
        let ty = &module.types.types()[0].unwrap_shared_enum();
        assert_eq!(ty.name, "SomeEnum");
    }

    /// Verify that we can parse an enum with one variant.
    #[test]
    fn enum_with_one_variant() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum SomeEnum {
                    SomeVariant
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.types.types().len(), 1);

        let ty = &module.types.types()[0].unwrap_shared_enum();
        assert_eq!(ty.variants.len(), 1);

        assert_eq!(ty.variants[0].name, "SomeVariant");
    }

    /// Verify that we can parse an enum that has a variant that has a field.
    #[test]
    fn parse_enum_variant_field() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                enum SomeEnum {
                    SomeVariant(u8)
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.types.types().len(), 1);

        let ty = &module.types.types()[0].unwrap_shared_enum();
        assert_eq!(ty.variants[0].fields.normalized_fields().len(), 1);

        match &ty.variants[0].fields {
            StructFields::Unnamed(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].ty.to_token_stream().to_string(), "u8");
            }
            _ => panic!(),
        }
    }

    /// Verify that we can parse the `#[swift_bridge(already_declared)`] attribute.
    #[test]
    fn already_declared_attribute() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                #[swift_bridge(already_declared)]
                enum SomeEnum {}
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.types.types().len(), 1);

        let ty = &module.types.types()[0].unwrap_shared_enum();
        assert!(ty.already_declared);
    }

    /// Verify that we return an error if an attribute isn't recognized.
    #[test]
    fn error_if_attribute_unrecognized() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                #[swift_bridge(unrecognized, invalid_attribute = "hi")]
                enum SomeEnum{
                    Variant
                }
            }
        };

        let errors = parse_errors(tokens);

        assert_eq!(errors.len(), 2);

        match &errors[0] {
            ParseError::EnumUnrecognizedAttribute { attribute } => {
                assert_eq!(&attribute.to_string(), "unrecognized");
            }
            _ => panic!(),
        };
        match &errors[1] {
            ParseError::EnumUnrecognizedAttribute { attribute } => {
                assert_eq!(&attribute.to_string(), "invalid_attribute");
            }
            _ => panic!(),
        };
    }
}
