use crate::codegen::generate_swift::{generate_swift_class_methods, ClassProtocols};
use crate::parse::OpaqueForeignTypeDeclaration;
use crate::{ParsedExternFn, TypeDeclarations, SWIFT_BRIDGE_PREFIX};
use std::collections::HashMap;
use syn::Path;

pub(super) fn generate_swift_class(
    ty: &OpaqueForeignTypeDeclaration,
    associated_funcs_and_methods: &HashMap<String, Vec<&ParsedExternFn>>,
    class_protocols: &ClassProtocols,
    types: &TypeDeclarations,
    swift_bridge_path: &Path,
) -> String {
    let type_name = ty.to_string();

    let class_methods = generate_swift_class_methods(
        &type_name,
        associated_funcs_and_methods,
        types,
        swift_bridge_path,
    );

    create_class_declaration(
        ty,
        class_protocols,
        &class_methods.initializers,
        &class_methods.owned_self_methods,
        &class_methods.ref_self_methods,
        &class_methods.ref_mut_self_methods,
    )
}

fn create_class_declaration(
    ty: &OpaqueForeignTypeDeclaration,
    class_protocols: &ClassProtocols,
    initializers: &[String],
    owned_self_methods: &[String],
    ref_self_methods: &[String],
    ref_mut_self_methods: &[String],
) -> String {
    let type_name = &ty.ty_name_ident().to_string();

    let mut class_decl = {
        let free_func_call = format!("{}${}$_free(ptr)", SWIFT_BRIDGE_PREFIX, type_name);

        format!(
            r#"public class {type_name}: {type_name}RefMut {{
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {{
        super.init(ptr: ptr)
    }}

    deinit {{
        if isOwned {{
            {free_func_call}
        }}
    }}
}}"#,
            type_name = type_name,
            free_func_call = free_func_call
        )
    };

    let mut class_ref_mut_decl = {
        format!(
            r#"
public class {type_name}RefMut: {type_name}Ref {{
    public override init(ptr: UnsafeMutableRawPointer) {{
        super.init(ptr: ptr)
    }}
}}"#,
            type_name = type_name
        )
    };
    let mut class_ref_decl = {
        format!(
            r#"
public class {type_name}Ref {{
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {{
        self.ptr = ptr
    }}
}}"#,
            type_name = type_name,
        )
    };
    if let Some(identifiable) = class_protocols.identifiable.as_ref() {
        let identifiable_var = if identifiable.func_name == "id" {
            "".to_string()
        } else {
            format!(
                r#"
    public var id: {identifiable_return_ty} {{
        return self.{identifiable_func}()
    }}
"#,
                identifiable_func = identifiable.func_name,
                identifiable_return_ty = identifiable.return_ty
            )
        };

        class_ref_decl += &format!(
            r#"
extension {type_name}Ref: Identifiable {{{identifiable_var}}}"#,
            type_name = type_name,
            identifiable_var = identifiable_var,
        );
    }

    let initializers = if initializers.len() == 0 {
        "".to_string()
    } else {
        let initializers: String = initializers.join("\n\n");
        format!(
            r#"
extension {type_name} {{
{initializers}
}}"#,
            type_name = type_name,
            initializers = initializers
        )
    };

    let owned_instance_methods = if owned_self_methods.len() == 0 {
        "".to_string()
    } else {
        let owned_instance_methods: String = owned_self_methods.join("\n\n");
        format!(
            r#"
extension {type_name} {{
{owned_instance_methods}
}}"#,
            type_name = type_name,
            owned_instance_methods = owned_instance_methods
        )
    };

    let ref_instance_methods = if ref_self_methods.len() == 0 {
        "".to_string()
    } else {
        let ref_instance_methods: String = ref_self_methods.join("\n\n");
        format!(
            r#"
extension {type_name}Ref {{
{ref_instance_methods}
}}"#,
            type_name = type_name,
            ref_instance_methods = ref_instance_methods
        )
    };

    let ref_mut_instance_methods = if ref_mut_self_methods.len() == 0 {
        "".to_string()
    } else {
        let ref_mut_instance_methods: String = ref_mut_self_methods.join("\n\n");
        format!(
            r#"
extension {type_name}RefMut {{
{ref_mut_instance_methods}
}}"#,
            type_name = type_name,
            ref_mut_instance_methods = ref_mut_instance_methods
        )
    };

    if ty.already_declared {
        class_decl = "".to_string();
        class_ref_decl = "".to_string();
        class_ref_mut_decl = "".to_string();
    }

    let class = format!(
        r#"
{class_decl}{initializers}{owned_instance_methods}{class_ref_decl}{ref_mut_instance_methods}{class_ref_mut_decl}{ref_instance_methods}"#,
        class_decl = class_decl,
        class_ref_decl = class_ref_mut_decl,
        class_ref_mut_decl = class_ref_decl,
        initializers = initializers,
        owned_instance_methods = owned_instance_methods,
        ref_mut_instance_methods = ref_mut_instance_methods,
        ref_instance_methods = ref_instance_methods,
    );

    return class;
}
