use crate::{ParsedExternFn, TypeMethod, SWIFT_BRIDGE_PREFIX};
use syn::ForeignItemType;

mod generate_c_header;
mod generate_swift;

#[derive(Default)]
pub(crate) struct ExternRustSection {
    pub types: Vec<ExternRustSectionType>,
    pub freestanding_fns: Vec<ParsedExternFn>,
}

pub(crate) struct ExternRustSectionType {
    /// `type Foo`
    pub ty: ForeignItemType,
    /// fn bar (&self);
    /// fn buzz (self: &Foo) -> u8;
    /// ... etc
    pub methods: Vec<TypeMethod>,
}

impl ExternRustSectionType {
    pub fn new(ty: ForeignItemType) -> Self {
        ExternRustSectionType {
            ty,
            methods: vec![],
        }
    }
}
