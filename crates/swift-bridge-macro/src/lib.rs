use syn::parse_macro_input;
mod module;
use quote::quote;

use self::module::Module;

#[cfg(test)]
mod test_utils;

#[proc_macro_attribute]
pub fn bridge(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let module = parse_macro_input!(input as Module);

    let tokens = quote! {
        #module
    };
    tokens.into()
}
