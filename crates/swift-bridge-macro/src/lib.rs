use quote::quote;
use swift_bridge_ir::SwiftBridgeModule;
use syn::parse_macro_input;

#[cfg(test)]
mod test_utils;

#[proc_macro_attribute]
pub fn bridge(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let module = parse_macro_input!(input as SwiftBridgeModule);

    let tokens = quote! {
        #module
    };
    tokens.into()
}
