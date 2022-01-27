use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Error, Receiver};
use syn::{ForeignItemType, LitStr};
use syn::{Token, Type};

pub(crate) enum ParseError {
    /// `extern {}`
    AbiNameMissing {
        /// `extern {}`
        ///  ------
        extern_token: Token![extern],
    },
    /// `extern "Foo" {}`
    AbiNameInvalid {
        /// `extern "Foo" {}`
        ///         -----
        abi_name: LitStr,
    },
    /// `fn foo (&self)`
    ///           ----
    AmbiguousSelf {
        self_: Receiver,
    },
    /// fn foo (bar: &Bar);
    /// If Bar wasn't declared using a `type Bar` declaration.
    UndeclaredType {
        ty: Type,
    },
    /// Declared a type that we already support.
    /// Example: `type u32`
    DeclaredBuiltInType {
        ty: ForeignItemType,
    },
    /// A bridge module struct with one or more fields must have a
    /// `#\[swift_bridge(swift_repr ="...")\[\]` attribute so that we know whether to create a
    /// `struct` or `class` on the Swift side.
    StructMissingSwiftRepr {
        struct_ident: Ident,
    },
    /// Only "class" and "struct" can be used as swift_repr.
    StructInvalidSwiftRepr {
        swift_repr_attr_value: LitStr,
    },
    /// A struct was declared with an unrecognized attribute.
    StructUnrecognizedAttribute {
        attribute: Ident,
    },
    /// There is no reason to use `swift_repr = "class"` on an empty struct.
    /// It's extra overhead with no advantages.
    EmptyStructHasSwiftReprClass {
        struct_ident: Ident,
        swift_repr_attr_value: LitStr,
    },
    FunctionAttribute(FunctionAttributeParseError),
}

/// An error while parsing a function attribute.
pub(crate) enum FunctionAttributeParseError {
    Identifiable(IdentifiableParseError),
}

/// An error while parsing a function's `Identifiable` attribute.
pub(crate) enum IdentifiableParseError {
    /// An `Identifiable` implementation function must take a single `(&self)` argument.
    MustBeRefSelf { fn_ident: Ident },
    /// An `Identifiable` implementation function must return a value.
    MissingReturnType { fn_ident: Ident },
}

impl Into<syn::Error> for ParseError {
    fn into(self) -> Error {
        match self {
            ParseError::AbiNameMissing {
                extern_token: extern_ident,
            } => Error::new_spanned(
                extern_ident,
                format!(
                    r#"extern modules must have their abi set to "Rust" or "Swift".
```
extern "Rust" {{ ... }}
extern "Swift" {{ ... }}
``` 
                "#
                ),
            ),
            ParseError::AbiNameInvalid { abi_name } => Error::new_spanned(
                abi_name,
                r#"Invalid abi name. Must be either "Rust" or "Swift"."#,
            ),
            ParseError::AmbiguousSelf { self_: self_ident } => Error::new_spanned(
                self_ident,
                r#"Could not infer a type for self. Try specifying the type:
self: SomeType
self: &SomeType
self: &mut SomeType
"#,
            ),
            ParseError::UndeclaredType { ty } => {
                let ty_name = ty.to_token_stream().to_string();
                // "& Bar" -> "Bar"
                let ty_name = ty_name.split_whitespace().last().unwrap();

                let message = format!(
                    r#"Type must be declared with `type {}`.
"#,
                    ty_name
                );
                Error::new_spanned(ty, message)
            }
            ParseError::DeclaredBuiltInType { ty } => {
                let message = format!(
                    r#"Type {} is already supported
"#,
                    ty.to_token_stream().to_string()
                );
                Error::new_spanned(ty, message)
            }
            ParseError::StructMissingSwiftRepr { struct_ident } => {
                let message = format!(
                    r#"Shared structs with one or more fields must specify their swift
representation. 
 
```
// Valid values are "struct" and "class"
#[swift_bridge(swift_repr = "struct")]
struct MyStruct {{
    count: u8
}}
```

TODO: Link to documntation on how to decide on the swift representation.
"#
                );
                Error::new_spanned(struct_ident, message)
            }
            ParseError::StructInvalidSwiftRepr {
                swift_repr_attr_value,
            } => {
                let message = r#"Invalid value. Must be either "class" or "struct"#;
                Error::new_spanned(swift_repr_attr_value, message)
            }
            ParseError::EmptyStructHasSwiftReprClass {
                struct_ident,
                swift_repr_attr_value,
            } => {
                let message = format!(
                    r#"Empty structs must have `swift_repr = "struct"`, since a class representation
would be additional overhead with no advantages. 
 
```
#[swift_bridge(swift_repr = "struct")]
struct {struct_name}; 
```
"#,
                    struct_name = struct_ident.to_string()
                );
                Error::new_spanned(swift_repr_attr_value, message)
            }
            ParseError::StructUnrecognizedAttribute { attribute } => {
                let message = format!(r#"Did not recognize struct attribute "{}"."#, attribute);
                Error::new_spanned(attribute, message)
            }
            ParseError::FunctionAttribute(fn_attrib) => match fn_attrib {
                FunctionAttributeParseError::Identifiable(identifiable) => match identifiable {
                    IdentifiableParseError::MustBeRefSelf { fn_ident } => {
                        let message = format!(
                            r#"Identifiable function {} must take `&self` as its only argument."#,
                            fn_ident
                        );
                        Error::new_spanned(fn_ident, message)
                    }
                    IdentifiableParseError::MissingReturnType { fn_ident } => {
                        let message = format!(
                            r#"Identifiable function {} must have a return type."#,
                            fn_ident
                        );
                        Error::new_spanned(fn_ident, message)
                    }
                },
            },
        }
    }
}
