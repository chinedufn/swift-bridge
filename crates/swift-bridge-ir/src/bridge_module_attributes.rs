use proc_macro2::Ident;
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
}

impl Parse for CfgAttr {
    /// Parses the arguments INSIDE `#[cfg(...)]`. syn 2 hands a list attribute's
    /// inner tokens to `Attribute::parse_args`, so the parentheses are already gone.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        if &ident == "feature" {
            input.parse::<Token![=]>()?;

            let feature_name = input.parse::<LitStr>()?;

            Ok(CfgAttr::Feature(feature_name))
        } else {
            todo!("Return an unsupported cfg kind error")
        }
    }
}
