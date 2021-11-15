use crate::{ExternFn, TypeMethod};
use syn::ForeignItemType;

pub(crate) struct ExternRustSection {
    pub types: Vec<ExternRustSectionType>,
    pub free_functions: Vec<ExternFn>,
}

pub(crate) struct ExternRustSectionType {
    /// `type Foo`
    pub ty: ForeignItemType,
    /// fn bar (&self);
    /// fn buzz (self: &Foo) -> u8;
    /// ... etc
    pub methods: Vec<TypeMethod>,
}
