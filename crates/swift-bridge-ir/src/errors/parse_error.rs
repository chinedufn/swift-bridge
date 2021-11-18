use proc_macro2::Span;
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
    AmbiguousSelf { self_: Receiver },
    /// fn foo (bar: &Bar);
    /// If Bar wasn't declared using a `type Bar` declaration.
    UndeclaredType { ty: String, span: Span },
    /// Declared a type that we already support.
    /// Example: `type u32`
    DeclaredBuiltInType { ty: ForeignItemType },
    /// By default, we do not allow you to pass an owned opaque type from Swift -> Rust.
    ///
    /// If we allowed this, Rust would drop the type, then the Swift class instance for the type
    /// would try to drop in again, leading to double free bugs.
    ///
    /// One can use the `#[swift_bridge(owned_arg = "enabled")] type MyType` attribute in order to
    /// support
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Not allowed
    /// extern "Rust" {
    ///     type Foo;
    ///     fn foo (arg: Foo);
    /// //               --- Owned foreign type arguments are not allowed.
    /// }
    /// ```
    OwnedForeignTypeArgNotAllowed { ty: Type },
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
            ParseError::DeclaredBuiltInType { ty } => {
                let message = format!(
                    r#"Type {} is already supported
"#,
                    ty.to_token_stream().to_string()
                );
                Error::new_spanned(ty, message)
            }
            ParseError::OwnedForeignTypeArgNotAllowed { ty } => {
                let message = format!(
                    r#"Foreign type arguments must be taken by reference.

To allow owned foreign type arguments:

```
// Valid values are "disabled", "enabled" and "enabled_unchecked" 
#[swift_bridge(owned_arg = "enabled")]
type SomeType;
```"#
                );
                Error::new_spanned(ty, message)
            }
        }
    }
}
