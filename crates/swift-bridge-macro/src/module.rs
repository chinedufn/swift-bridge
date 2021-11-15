use syn::__private::Span;
use syn::{ForeignItemFn, ForeignItemType, Ident, LitStr};

mod parse;
mod to_tokens;

pub struct Module {
    name: Ident,
    sections: Vec<ModuleSection>,
    // Any issues that we encountered while parsing the module.
    errors: Vec<ModuleParseError>,
}

enum ModuleSection {
    ExternRust(Vec<ExternRustItem>),
    ExternSwift,
}

enum ExternRustItem {
    TypeDeclaration(ForeignItemType),
    Func(ForeignItemFn),
}

// These get turned into `compile_error!`'s
enum ModuleParseError {
    AbiMissingName { foreign_module_span: Span },
    AbiNameInvalid { abi_name: LitStr },
}

enum AbiName {
    Rust,
    Swift,
}
