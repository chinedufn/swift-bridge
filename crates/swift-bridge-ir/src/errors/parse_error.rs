use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::Token;
use syn::{Error, Receiver};
use syn::{LitStr, Type};

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
    AmbiguousSelf { self_: Receiver },
    /// fn foo (bar: &Bar);
    /// If Bar wasn't declared using a `type Bar` declaration.
    UndeclaredType { ty: String, span: Span },
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
            ParseError::UndeclaredType { ty, span } => {
                let message = format!(
                    r#"Type must be declared with `type {}`.
"#,
                    ty
                );
                Error::new(span, message)
            }
        }
    }
}
