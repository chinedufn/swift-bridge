use crate::ParsedExternFn;
use syn::ForeignItemType;

pub struct ExternSwiftSection {
    types: Vec<ExternSwiftSectionType>,
    free_functions: Vec<ParsedExternFn>,
}

struct ExternSwiftSectionType {
    ty: ForeignItemType,
    funcs: Vec<ParsedExternFn>,
}
