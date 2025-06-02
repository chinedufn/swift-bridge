use proc_macro2::{Ident, TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::LitStr;
use syn::Token;

/// A `cfg` attribute on a bridge module.
///
/// ```no_run
/// #[swift_bridge::bridge]
/// // This is a cfg attribute.
/// #[cfg(feature = "some-feature")]
/// mod ffi {
/// }
/// ```
pub enum CfgAttr {
    /// #\[cfg(feature = "...")\]
    Feature(LitStr),
    /// Any cfg attribute, storing the full token stream inside the parentheses.
    #[allow(dead_code)]
    Generic(TokenStream),
}

impl Parse for CfgAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let fork = content.fork();
        if let Ok(ident) = fork.parse::<Ident>() {
            if ident == "feature" {
                content.parse::<Ident>()?; // consume 'feature'
                content.parse::<Token![=]>()?;
                let value = content.parse::<LitStr>()?;
                return Ok(CfgAttr::Feature(value));
            }
        }
        // Fallback: store the full content as a token stream
        Ok(CfgAttr::Generic(content.parse()?))
    }
}
