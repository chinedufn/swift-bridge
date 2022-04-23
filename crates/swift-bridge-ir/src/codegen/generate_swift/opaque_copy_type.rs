use crate::codegen::generate_swift::generate_swift_class_methods;
use crate::parse::OpaqueForeignTypeDeclaration;
use crate::{ParsedExternFn, TypeDeclarations, SWIFT_BRIDGE_PREFIX};
use std::collections::HashMap;
use syn::Path;

pub(super) fn generate_opaque_copy_struct(
    ty: &OpaqueForeignTypeDeclaration,
    associated_funcs_and_methods: &HashMap<String, Vec<&ParsedExternFn>>,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let type_name = &ty.ty.to_string();

    let class_methods = generate_swift_class_methods(
        &type_name,
        associated_funcs_and_methods,
        types,
        swift_bridge_path,
    );

    let mut extensions = "".to_string();

    append_methods_extension(&mut extensions, type_name, &class_methods.initializers);
    append_methods_extension(
        &mut extensions,
        type_name,
        &class_methods.owned_self_methods,
    );
    append_methods_extension(&mut extensions, type_name, &class_methods.ref_self_methods);

    if class_methods.owned_self_methods.len() > 0 {};

    let struct_definition = if !ty.already_declared {
        generate_struct_definition(type_name)
    } else {
        "".to_string()
    };

    format!(
        r#"{struct_definition}{extensions}"#,
        struct_definition = struct_definition,
        extensions = extensions
    )
}

fn generate_struct_definition(type_name: &str) -> String {
    format!(
        r#"public struct {type_name} {{
    fileprivate var bytes: {prefix}${type_name}

    func intoFfiRepr() -> {prefix}${type_name} {{
        bytes
    }}
}}
extension {prefix}${type_name} {{
    func intoSwiftRepr() -> {type_name} {{
        {type_name}(bytes: self)
    }}
}}"#,
        prefix = SWIFT_BRIDGE_PREFIX,
        type_name = type_name,
    )
}

fn append_methods_extension(extensions: &mut String, type_name: &str, methods: &[String]) {
    if methods.len() == 0 {
        return;
    }

    *extensions += &format!(
        r#"
extension {type_name} {{
"#,
        type_name = type_name
    );

    for (idx, method) in methods.iter().enumerate() {
        if idx > 0 {
            *extensions += "\n";
        }

        *extensions += method;
    }

    *extensions += "\n}";
}
