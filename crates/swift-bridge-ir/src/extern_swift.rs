use crate::{ExternFn, TypeMethod};
use syn::ForeignItemType;

pub struct ExternSwiftSection {
    types: Vec<ExternSwiftSectionType>,
    free_functions: Vec<ExternFn>,
}

struct ExternSwiftSectionType {
    ty: ForeignItemType,
    funcs: Vec<TypeMethod>,
}
