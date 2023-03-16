use crate::bridged_type::shared_struct::StructField;
use crate::bridged_type::{BridgedType, SharedStruct, StructFields, StructSwiftRepr, TypePosition};
use crate::codegen::generate_rust_tokens::can_generate_vec_of_transparent_struct_functions;
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
                let initializer_params = match &shared_struct.fields {
                    StructFields::Named(named) => self.convert_fields_to_initializer_params(named),
                    StructFields::Unnamed(unnamed) => {
                        self.convert_fields_to_initializer_params(unnamed)
                    }
                    StructFields::Unit => "".to_string(),
                };

                let initializer_body = match &shared_struct.fields {
                    StructFields::Named(named) => self.convert_fields_to_initializer_body(named),
                    StructFields::Unnamed(unnamed) => {
                        self.convert_fields_to_initializer_body(unnamed)
                    }
                    StructFields::Unit => "".to_string(),
                };

                let fields = match &shared_struct.fields {
                    StructFields::Named(named) => self.declare_fields(named),
                    StructFields::Unnamed(unnamed) => self.declare_fields(unnamed),
                    StructFields::Unit => "".to_string(),
                };

                let convert_swift_to_ffi_repr =
                    shared_struct.convert_swift_to_ffi_repr("self", &self.types);
                let convert_ffi_repr_to_swift =
                    shared_struct.convert_ffi_expression_to_swift("self", &self.types);

                let vectorizable_impl = if can_generate_vec_of_transparent_struct_functions(&shared_struct) {
                    format!(
                        r#"
extension {struct_name}: Vectorizable {{
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {{
        __swift_bridge__$Vec_{struct_name}$new()
    }}

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {{
        __swift_bridge__$Vec_{struct_name}$drop(vecPtr)
    }}

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {{
        __swift_bridge__$Vec_{struct_name}$push(vecPtr, value.intoFfiRepr())
    }}

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {{
        let maybeStruct = __swift_bridge__$Vec_{struct_name}$pop(vecPtr)
        return maybeStruct.intoSwiftRepr()
    }}

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        let maybeStruct = __swift_bridge__$Vec_{struct_name}$get(vecPtr, index)
        return maybeStruct.intoSwiftRepr()
    }}

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        let maybeStruct = __swift_bridge__$Vec_{struct_name}$get_mut(vecPtr, index)
        return maybeStruct.intoSwiftRepr()
    }}

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {{
        __swift_bridge__$Vec_{struct_name}$len(vecPtr)
    }}
}}"#,
                        struct_name = struct_name
                    )
                } else {
                    format!("")    
                };

                // No need to generate any code. Swift will automatically generate a
                //  struct from our C header typedef that we generate for this struct.
                let swift_struct = format!(
                    r#"public struct {struct_name} {{{fields}
    public init({initializer_params}) {{{initializer_body}}}

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
}}{vectorizable_impl}"#,
                    struct_name = struct_name,
                    initializer_params = initializer_params,
                    initializer_body = initializer_body,
                    fields = fields,
                    ffi_repr_name = shared_struct.ffi_name_string(),
                    option_ffi_name = option_ffi_name,
                    convert_swift_to_ffi_repr = convert_swift_to_ffi_repr,
                    convert_ffi_repr_to_swift = convert_ffi_repr_to_swift,
                    vectorizable_impl = vectorizable_impl
                );

                Some(swift_struct)
            }
        }
    }

    fn convert_fields_to_initializer_params<'a, T>(
        &self,
        struct_fields: impl IntoIterator<Item = &'a T>,
    ) -> String
    where
        T: StructField + 'a,
    {
        let mut params = "".to_string();

        for field in struct_fields.into_iter() {
            let bridged_ty = BridgedType::new_with_type(field.field_type(), &self.types).unwrap();

            params += &format!(
                "{}: {},",
                field.swift_name_string(),
                bridged_ty.to_swift_type(TypePosition::SharedStructField, &self.types)
            );
        }

        if !params.is_empty() {
            params.pop();
        }

        params
    }

    fn convert_fields_to_initializer_body<'a, T>(
        &self,
        struct_fields: impl IntoIterator<Item = &'a T>,
    ) -> String
    where
        T: StructField + 'a,
    {
        let mut body = "".to_string();

        for field in struct_fields.into_iter() {
            body += &format!(
                "        self.{} = {}\n",
                field.swift_name_string(),
                field.swift_name_string()
            );
        }

        if !body.is_empty() {
            body = format!("\n{}    ", body);
        }

        body
    }

    fn declare_fields<'a, T>(&self, struct_fields: impl IntoIterator<Item = &'a T>) -> String
    where
        T: StructField + 'a,
    {
        let mut fields = "".to_string();

        for field in struct_fields.into_iter() {
            let bridged_ty = BridgedType::new_with_type(field.field_type(), &self.types).unwrap();

            fields += &format!(
                "    public var {}: {}\n",
                field.swift_name_string(),
                bridged_ty.to_swift_type(TypePosition::SharedStructField, &self.types)
            );
        }

        if !fields.is_empty() {
            fields = format!("\n{}", fields)
        }

        fields
    }
}
