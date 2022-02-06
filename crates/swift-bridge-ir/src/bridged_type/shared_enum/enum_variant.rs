use crate::bridged_type::StructFields;
use proc_macro2::Ident;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub(crate) struct EnumVariant {
    pub name: Ident,
    // Will be used in a future commit.
    #[allow(unused)]
    pub fields: StructFields,
}

impl PartialEq for EnumVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_string() == other.name.to_string()
    }
}

impl Debug for EnumVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnumVariant")
            .field("name", &self.name.to_string())
            .field("fields", &self.fields)
            .finish()
    }
}
