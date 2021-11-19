use quote::quote;
use swift_bridge_ir::{SwiftBridgeModule, SwiftBridgeModuleAttr, SwiftBridgeModuleAttrs};
use syn::parse_macro_input;

#[cfg(test)]
mod test_utils;

#[proc_macro_attribute]
pub fn bridge(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as SwiftBridgeModuleAttrs);
    let mut module = parse_macro_input!(input as SwiftBridgeModule);

    for arg in args.attributes {
        match arg {
            SwiftBridgeModuleAttr::SwiftBridgePath(path) => {
                module.set_swift_bridge_path(path);
            }
        }
    }

    let tokens = quote! {
        #module
    };
    tokens.into()
}
