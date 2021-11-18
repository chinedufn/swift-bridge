use crate::ParsedExternFn;
use syn::ForeignItemType;

mod generate_c_header;
mod generate_swift;

#[derive(Default)]
pub(crate) struct ExternRustSection {
    pub types: Vec<ForeignItemType>,
    pub functions: Vec<ParsedExternFn>,
}
