use crate::errors::{ParseError, ParseErrors};
use crate::{SharedStruct, StructField, StructSwiftRepr};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{ItemStruct, LitStr, Token};

// Ok.. let's plan what we need to support for our current Afia needs...
//
// So.. we want to generate a `struct` and a header for that struct that will allow us to use
// our existing pos_ffi_uuid.
//
// We want to always guarantee that the Rust side of the boat is safe... Any unsafety would be
// because of the Swift side.
// So... if a struct has all copy fields we can make it a Swift struct.
// We do not allow swift structs to be passed by reference, only by value.
//
// So we want to
// 1. Create a C typedef that Swift can see (pos_ffi_uuid)
// 2. Convert that into and from our Rust type (Uuid)
// 3. Ensure that only copy types can be passed as structs.
//
// Cool.. so just add some tests and code for these guarantees.
// Let's plan the steps to do that.
//
// # Implementation
//
// 1. (DONE) Add a test for parsing a unit struct. Unit structs are always C structs.
//
// 2. (DONE) Add a test for parsing an empty struct {}. Empty structs are always C structs.
//
// 3. (DONE) Add a test for parsing an empty tuple struct (). Empty structs are always C structs.
//
// 4. (DONE) Add a test for parsing a struct with one field. Check that we get an error that structs
//    with one or more fields need to be annotated with `#[swift_bridge(swift_repr = "...")]`
//
// 5. (DONE) Add a test for successfully parsing a struct with one field that is marked `swift_repr =
//    "struct"`
//
// 6. For now we can panic if the swift_repr is "class" that it is not yet implemented.
//
// 7. (SKIP) Add test verifying that we can parse a struct with a `[T: Copy; N]` array field.
//    - I don't need this right now. Afia can just transmute u128 into [u64, u64] and use that
//
// 8. (SKIP) Add a test verifying we get an error if we try to parse a field that is not copy
//    when the swift_repr is struct.
//    - We can parse it, it just has certain rules for how it can be used in functions.
//
// 9. (DONE) Add a C header generation test where we verify that a `typedef struct` is emitted
//    for a struct.
//
// 10. (DONE) Add test verifying that we properly emit the C typedef for a struct
//
// 11. (SKIP) Add test that we cannot parse a `swift_repr "struct"` if there is a non copy field.
//     - See number 8
//
// 12. (DONE) Add test verifying that we parse the `rust_into = SomeType` attribute
//
// 13. (SKIP) Add test verifying that we parse the `rust_from = SomeType` attribute
//     - I don't need this yet
//
// 14. (DONE) Add extern "Rust" function test where accept a swift_repr=struct type
//
// 15. (DONE) Add extern "Rust" function test where we return a swift_repr=struct type
//
// 16. (SKIP) Add extern "Swift" function test where accept a swift_repr=struct type
//     - I don't need this yet
//
// 17. (SKIP) Add extern "Swift" function test where we return a swift_repr=struct type
//     - I don't need this yet
//
// 18. (SKIP) Add extern "Rust" function test for accepting a `rust_into = Foo` argument
//     and making sure that we convert it into the function that takes a `Foo`
//     - Skipping this.. We can just make our real function taken an `impl Into<Uuid>`
//
// 19. (Skip_ Add extern "Rust" function test for returning a `rust_into = Foo` argument
//     - Skipping this.. We can just make our real function taken an `impl Into<Uuid>`
//
// 20. (SKIP) Add extern "Swift" function test for accepting a `rust_into = Foo` argument
//     - I don't need this yet
//
// 21. (SKIP) Add extern "Swift" function test for returning a `rust_into = Foo` argument
//     - I don't need this yet
//
// 22. (DONE) Add test that we use the `swift_name = "..."` attribute on the struct when creating
//     the typedef
//
// 23. Add swift-integration-test file `shared_types.rs` test out shared structs

pub(crate) struct SharedStructParser<'a> {
    pub item_struct: ItemStruct,
    pub errors: &'a mut ParseErrors,
}

enum StructAttr {
    SwiftRepr((StructSwiftRepr, LitStr)),
    SwiftName(LitStr),
    Error(StructAttrParseError),
}

enum StructAttrParseError {
    InvalidSwiftRepr(LitStr),
}

#[derive(Default)]
struct StructAttribs {
    swift_repr: Option<(StructSwiftRepr, LitStr)>,
    swift_name: Option<LitStr>,
}

impl Parse for StructAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        let attr = match key.to_string().as_str() {
            "swift_repr" => {
                let repr: LitStr = input.parse()?;
                match repr.value().as_str() {
                    "class" => StructAttr::SwiftRepr((StructSwiftRepr::Class, repr)),
                    "struct" => StructAttr::SwiftRepr((StructSwiftRepr::Structure, repr)),
                    _ => StructAttr::Error(StructAttrParseError::InvalidSwiftRepr(repr)),
                }
            }
            "swift_name" => {
                let name = input.parse()?;
                StructAttr::SwiftName(name)
            }
            _ => todo!("Return spanned error"),
        };

        Ok(attr)
    }
}

impl<'a> SharedStructParser<'a> {
    pub fn parse(self) -> Result<SharedStruct, syn::Error> {
        let item_struct = self.item_struct;

        let mut attribs = StructAttribs::default();
        let mut fields = vec![];

        for attr in item_struct.attrs {
            let attr: StructAttr = attr.parse_args()?;
            match attr {
                StructAttr::SwiftRepr((repr, lit_str)) => {
                    attribs.swift_repr = Some((repr, lit_str));
                }
                StructAttr::SwiftName(name) => {
                    attribs.swift_name = Some(name);
                }
                StructAttr::Error(err) => match err {
                    StructAttrParseError::InvalidSwiftRepr(val) => {
                        self.errors.push(ParseError::StructInvalidSwiftRepr {
                            struct_ident: item_struct.ident.clone(),
                            swift_repr_attr_value: val.clone(),
                        });
                        attribs.swift_repr = Some((StructSwiftRepr::Structure, val));
                    }
                },
            }
        }

        let swift_repr = if item_struct.fields.len() == 0 {
            if let Some((swift_repr, lit_str)) = attribs.swift_repr {
                if swift_repr == StructSwiftRepr::Class {
                    self.errors.push(ParseError::EmptyStructHasSwiftReprClass {
                        struct_ident: item_struct.ident.clone(),
                        swift_repr_attr_value: lit_str,
                    });
                }
            }

            StructSwiftRepr::Structure
        } else if let Some((swift_repr, _)) = attribs.swift_repr {
            swift_repr
        } else {
            self.errors.push(ParseError::StructMissingSwiftRepr {
                struct_ident: item_struct.ident.clone(),
            });

            StructSwiftRepr::Structure
        };

        for field in item_struct.fields.iter() {
            let field = StructField {
                name: field.ident.clone(),
                ty: field.ty.clone(),
            };
            fields.push(field);
        }

        let shared_struct = SharedStruct {
            name: item_struct.ident,
            swift_repr,
            fields,
            swift_name: attribs.swift_name,
        };

        Ok(shared_struct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{parse_errors, parse_ok};
    use quote::{quote, ToTokens};

    /// Verify that we can parse a struct with no fields.
    /// Structs with no fields always have an implicit `swift_repr = "struct"`.
    #[test]
    fn parse_unit_struct() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                struct Foo;
                struct Bar;
                struct Bazz;
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.types.types().len(), 3);
        for (idx, name) in vec!["Foo", "Bar", "Bazz"].into_iter().enumerate() {
            let ty = &module.types.types()[idx].unwrap_shared_struct();

            assert_eq!(ty.name, name);
            assert_eq!(ty.swift_repr, StructSwiftRepr::Structure);
        }
    }

    /// Verify that we get an error if a Struct has one or more fields and no `swift_repr`
    /// attribute.
    #[test]
    fn error_if_missing_swift_repr() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                struct Foo {
                    bar: u8
                }
            }
        };

        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 1);

        match &errors[0] {
            ParseError::StructMissingSwiftRepr { struct_ident } => {
                assert_eq!(struct_ident, "Foo");
            }
            _ => panic!(),
        };
    }

    /// Verify that we push an error if the Swift representation attribute is invalid.
    #[test]
    fn error_if_invalid_swift_repr() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                #[swift_bridge(swift_repr = "an-invalid-value")]
                struct Foo {
                    bar: u8
                }
            }
        };

        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 1);

        match &errors[0] {
            ParseError::StructInvalidSwiftRepr {
                struct_ident,
                swift_repr_attr_value,
            } => {
                assert_eq!(struct_ident, "Foo");
                assert_eq!(swift_repr_attr_value.value(), "an-invalid-value");
            }
            _ => panic!(),
        };
    }

    /// Verify that we push an error if a struct with no fields has it's swift_repr set to "class",
    /// since there is no advantage to bearing that extra overhead.
    #[test]
    fn error_if_empty_struct_swift_repr_set_to_class() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                #[swift_bridge(swift_repr = "class")]
                struct Foo;

                #[swift_bridge(swift_repr = "class")]
                struct Bar;

                #[swift_bridge(swift_repr = "class")]
                struct Buzz;
            }
        };

        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 3);

        for (idx, struct_name) in vec!["Foo", "Bar", "Buzz"].into_iter().enumerate() {
            match &errors[idx] {
                ParseError::EmptyStructHasSwiftReprClass {
                    struct_ident,
                    swift_repr_attr_value,
                } => {
                    assert_eq!(struct_ident, struct_name);
                    assert_eq!(swift_repr_attr_value.value(), "class");
                }
                _ => panic!(),
            };
        }
    }

    /// Verify that we can parse a struct with a named field.
    #[test]
    fn parse_struct_with_named_u8_field() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                #[swift_bridge(swift_repr = "struct")]
                struct Foo {
                    bar: u8
                }
            }
        };

        let module = parse_ok(tokens);

        let ty = module.types.types()[0].unwrap_shared_struct();
        let field = &ty.fields[0];

        assert_eq!(field.name.as_ref().unwrap(), "bar");
        assert_eq!(field.ty.to_token_stream().to_string(), "u8");
    }

    /// Verify that we parse the swift_name = "..."
    #[test]
    fn parse_swift_name_attribute() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                #[swift_bridge(swift_name = "FfiFoo")]
                struct Foo;
            }
        };

        let module = parse_ok(tokens);

        let ty = module.types.types()[0].unwrap_shared_struct();
        assert_eq!(ty.swift_name.as_ref().unwrap().value(), "FfiFoo");
    }
}
