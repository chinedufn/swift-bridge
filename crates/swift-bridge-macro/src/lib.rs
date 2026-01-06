use quote::quote;
use swift_bridge_ir::{SwiftBridgeModule, SwiftBridgeModuleAttr, SwiftBridgeModuleAttrs};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, LitStr, Token};

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

/// Attribute macro that generates FFI glue code for a struct or enum defined outside a bridge module.
///
/// This macro allows you to define a struct in regular Rust code and use it in bridge modules
/// with `#[swift_bridge(already_declared)]`, achieving zero-duplication.
///
/// # Example
///
/// ```ignore
/// #[swift_bridge::bridged]
/// pub struct UserData {
///     pub id: u32,
///     pub active: bool,
/// }
///
/// #[swift_bridge::bridge]
/// mod ffi {
///     #[swift_bridge(already_declared)]
///     struct UserData;  // No fields needed - zero duplication!
///
///     extern "Swift" {
///         fn get_user() -> UserData;
///     }
/// }
/// ```
///
/// The macro generates:
/// - The original struct (preserved)
/// - FFI struct representation (`__swift_bridge__StructName`)
/// - `impl SharedStruct` trait
/// - `into_ffi_repr()` and `into_rust_repr()` conversion methods
/// - Swift code as a string constant (for the build script)
/// - C header code as a string constant (for the build script)
///
/// ## Enum Support
///
/// Enum support is planned but not yet implemented. Using `#[swift_bridge::bridged]` on an enum
/// will currently produce a compile error.
///
/// ## Attributes
///
/// - `swift_name = "SwiftName"`: Use a different name for the Swift struct.
///
/// ```ignore
/// #[swift_bridge::bridged(swift_name = "SwiftProduct")]
/// pub struct RustProduct {
///     pub id: u32,
/// }
/// // Generates Swift struct "SwiftProduct" while FFI uses "RustProduct"
/// ```
#[proc_macro_attribute]
pub fn bridged(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = parse_macro_input!(attr as BridgedAttrs);
    let item = parse_macro_input!(item as syn::Item);
    match item {
        syn::Item::Struct(item_struct) => {
            swift_bridge_ir::bridged_struct::generate_bridged_struct_tokens(
                item_struct,
                attrs.swift_name,
            )
            .into()
        }
        syn::Item::Enum(item_enum) => syn::Error::new_spanned(
            item_enum,
            "#[swift_bridge::bridged] does not yet support enums. Enum support is planned for a future release.",
        )
        .to_compile_error()
        .into(),
        other => syn::Error::new_spanned(
            other,
            "#[swift_bridge::bridged] only supports structs (and enums in the future)",
        )
        .to_compile_error()
        .into(),
    }
}

/// Parsed attributes for the `#[swift_bridge::bridged]` macro.
struct BridgedAttrs {
    swift_name: Option<String>,
}

impl Parse for BridgedAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut swift_name = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "swift_name" {
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                swift_name = Some(lit.value());
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("unknown attribute: {}", ident),
                ));
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(BridgedAttrs { swift_name })
    }
}

#[cfg(test)]
mod ui_tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }
}
