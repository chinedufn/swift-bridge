use crate::SWIFT_BRIDGE_PREFIX;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::fmt::{Debug, Formatter};

mod enum_variant;
pub(crate) use self::enum_variant::EnumVariant;

#[derive(Clone)]
pub(crate) struct SharedEnum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

impl SharedEnum {
    /// SomeEnum
    pub fn swift_name_string(&self) -> String {
        format!("{}", self.name)
    }

    /// __swift_bridge__$SomeEnum
    pub fn ffi_name_string(&self) -> String {
        format!("{}${}", SWIFT_BRIDGE_PREFIX, self.name)
    }

    /// __swift_bridge__$SomeEnumTag
    pub fn ffi_tag_name_string(&self) -> String {
        format!("{}Tag", self.ffi_name_string())
    }

    /// __swift_bridge__SomeEnum
    pub fn ffi_name_tokens(&self) -> TokenStream {
        let name = Ident::new(
            &format!("{}{}", SWIFT_BRIDGE_PREFIX, self.name),
            self.name.span(),
        );
        quote! { #name }
    }
}

impl PartialEq for SharedEnum {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_string() == other.name.to_string() && self.variants == other.variants
    }
}

impl Debug for SharedEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedEnum")
            .field("name", &self.name.to_string())
            .field("variants", &self.variants)
            .finish()
    }
}
