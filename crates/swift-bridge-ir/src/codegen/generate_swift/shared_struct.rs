use crate::bridged_type::{BridgedType, SharedStruct, StructFields, StructSwiftRepr, TypePosition};
use crate::SwiftBridgeModule;

impl SwiftBridgeModule {
    /// Generate the tokens for a shared struct.
    pub(super) fn generate_shared_struct_string(
        &self,
        shared_struct: &SharedStruct,
    ) -> Option<String> {
        if shared_struct.already_declared {
            return None;
        }

        let struct_name = &shared_struct.swift_name_string();
        let option_ffi_name = shared_struct.ffi_option_name_string();

        match shared_struct.swift_repr {
            StructSwiftRepr::Class => {
                todo!()
            }
            StructSwiftRepr::Structure => {
                let mut fields = match &shared_struct.fields {
                    StructFields::Named(named) => {
                        let mut fields = "".to_string();

                        for field in named.iter() {
                            let bridged_ty =
                                BridgedType::new_with_type(&field.ty, &self.types).unwrap();

                            fields += &format!(
                                "    var {}: {}\n",
                                field.swift_name_string(),
                                bridged_ty.to_swift_type(TypePosition::SharedStructField)
                            );
                        }

                        fields
                    }
                    StructFields::Unnamed(unnamed) => {
                        let mut fields = "".to_string();

                        for field in unnamed.iter() {
                            let bridged_ty =
                                BridgedType::new_with_type(&field.ty, &self.types).unwrap();

                            fields += &format!(
                                "    var {}: {}\n",
                                field.swift_name_string(),
                                bridged_ty.to_swift_type(TypePosition::SharedStructField)
                            );
                        }

                        fields
                    }
                    StructFields::Unit => "".to_string(),
                };

                if fields.len() > 0 {
                    fields = format!("\n{}", fields)
                }

                let convert_swift_to_ffi_repr =
                    shared_struct.convert_swift_to_ffi_repr("self", &self.types);
                let convert_ffi_repr_to_swift =
                    shared_struct.convert_ffi_expression_to_swift("self", &self.types);

                // No need to generate any code. Swift will automatically generate a
                //  struct from our C header typedef that we generate for this struct.
                let swift_struct = format!(
                    r#"public struct {struct_name} {{{fields}
    @inline(__always)
    func intoFfiRepr() -> {ffi_repr_name} {{
        {convert_swift_to_ffi_repr}
    }}
}}
extension {ffi_repr_name} {{
    @inline(__always)
    func intoSwiftRepr() -> {struct_name} {{
        {convert_ffi_repr_to_swift}
    }}
}}
extension {option_ffi_name} {{
    @inline(__always)
    func intoSwiftRepr() -> Optional<{struct_name}> {{
        if self.is_some {{
            return self.val.intoSwiftRepr()
        }} else {{
            return nil
        }}
    }}

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<{struct_name}>) -> {option_ffi_name} {{
        if let v = val {{
            return {option_ffi_name}(is_some: true, val: v.intoFfiRepr())
        }} else {{
            return {option_ffi_name}(is_some: false, val: {ffi_repr_name}())
        }}
    }}
}}"#,
                    struct_name = struct_name,
                    fields = fields,
                    ffi_repr_name = shared_struct.ffi_name_string(),
                    option_ffi_name = option_ffi_name,
                    convert_swift_to_ffi_repr = convert_swift_to_ffi_repr,
                    convert_ffi_repr_to_swift = convert_ffi_repr_to_swift
                );

                Some(swift_struct)
            }
        }
    }
}
