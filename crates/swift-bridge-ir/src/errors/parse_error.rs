use proc_macro2::Ident;
use syn::Error;
use syn::LitStr;

pub(crate) enum ParseError {
    /// `extern {}`
    AbiNameMissing {
        /// `extern {}`
        ///  ------
        extern_ident: Ident,
    },
    /// `extern "Foo" {}`
    AbiNameInvalid {
        /// `extern "Foo" {}`
        ///         -----
        abi_name: LitStr,
    },
    /// `fn foo (&self)`
    ///           ----
    AmbiguousSelf { self_ident: Ident },
}

impl Into<syn::Error> for ParseError {
    fn into(self) -> Error {
        match self {
            ParseError::AbiNameMissing { extern_ident } => Error::new_spanned(
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
            ParseError::AmbiguousSelf { self_ident } => Error::new_spanned(
                self_ident,
                r#"Could not infer a type for self. Try specifying the type:
self: SomeType
self: &SomeType
self: &mut SomeType
"#,
            ),
        }
    }
}
