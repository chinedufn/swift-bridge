use proc_macro2::Ident;
use std::fmt::{Debug, Formatter};

mod enum_variant;
pub(crate) use self::enum_variant::EnumVariant;

#[derive(Clone)]
pub(crate) struct SharedEnum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
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
