#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::parse_ok;
    use quote::{quote, ToTokens};

    /// Verify that we can parse a unit struct.
    #[test]
    fn parse_unit_struct() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                struct Foo;
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.types.len(), 1);
        assert_eq!(module.types[0].unwrap_shared_struct().name, "Foo");
    }

    /// Verify that we can parse a struct with a named field.
    #[test]
    fn parse_struct_with_named_u8_field() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                struct Foo {
                    bar: u8
                }
            }
        };

        let module = parse_ok(tokens);

        let ty = module.types[0].unwrap_shared_struct();
        let field = &ty.fields[0];

        assert_eq!(field.name.as_ref().unwrap(), "bar");
        assert_eq!(field.ty.to_token_stream().to_string(), "u8");
    }
}
