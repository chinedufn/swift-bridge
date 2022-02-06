use crate::bridged_type::{EnumVariant, SharedEnum, StructFields};
use crate::errors::ParseErrors;
use syn::ItemEnum;

pub(crate) struct SharedEnumDeclarationParser<'a> {
    pub item_enum: ItemEnum,
    // Will be used in a future commit..
    #[allow(unused)]
    pub errors: &'a mut ParseErrors,
}

impl<'a> SharedEnumDeclarationParser<'a> {
    pub fn parse(self) -> Result<SharedEnum, syn::Error> {
        let item_enum = self.item_enum;

        let mut variants = vec![];

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
        };

        Ok(shared_enum)
    }
}

#[cfg(test)]
mod tests {
    use crate::bridged_type::StructFields;
    use crate::test_utils::parse_ok;
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
}
