use crate::bridged_type::SharedEnum;
use crate::SwiftBridgeModule;

impl SwiftBridgeModule {
    /// Generate the tokens for a shared enum.
    pub(super) fn generate_shared_enum_string(&self, shared_enum: &SharedEnum) -> Option<String> {
        let enum_name = shared_enum.swift_name_string();
        let enum_ffi_name = shared_enum.ffi_name_string();
        let option_ffi_name = shared_enum.ffi_option_name_string();

        let mut variants = "".to_string();
        let mut convert_swift_to_ffi_repr = "".to_string();
        let mut convert_ffi_repr_to_swift = "".to_string();

        for variant in shared_enum.variants.iter() {
            let v = format!(
                r#"
    case {name}"#,
                name = variant.name
            );
            variants += &v;
        }
        if variants.len() > 0 {
            variants += "\n";
        }

        for variant in shared_enum.variants.iter() {
            let case = format!(
                r#"
            case {enum_name}.{variant_name}:
                return {enum_ffi_name}(tag: {enum_ffi_name}${variant_name})"#,
                enum_name = enum_name,
                enum_ffi_name = enum_ffi_name,
                variant_name = variant.name
            );
            convert_swift_to_ffi_repr += &case;
        }
        if convert_swift_to_ffi_repr.len() > 0 {
            convert_swift_to_ffi_repr += "\n        ";
        }

        for variant in shared_enum.variants.iter() {
            let case = format!(
                r#"
            case {enum_ffi_name}${variant_name}:
                return {enum_name}.{variant_name}"#,
                enum_name = enum_name,
                enum_ffi_name = enum_ffi_name,
                variant_name = variant.name
            );
            convert_ffi_repr_to_swift += &case;
        }
        if convert_ffi_repr_to_swift.len() > 0 {
            convert_ffi_repr_to_swift += &format!(
                r#"
            default:
                fatalError("Unreachable")
        "#
            );
        }

        let vectorizable_impl = if shared_enum.has_one_or_more_variants_with_data() {
            "".to_string()
        } else {
            format!(
                r#"
extension {enum_name}: Vectorizable {{
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {{
        __swift_bridge__$Vec_{enum_name}$new()
    }}

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {{
        __swift_bridge__$Vec_{enum_name}$drop(vecPtr)
    }}

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {{
        __swift_bridge__$Vec_{enum_name}$push(vecPtr, value.intoFfiRepr())
    }}

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {{
        let maybeEnum = __swift_bridge__$Vec_{enum_name}$pop(vecPtr)
        return maybeEnum.intoSwiftRepr()
    }}

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        let maybeEnum = __swift_bridge__$Vec_{enum_name}$get(vecPtr, index)
        return maybeEnum.intoSwiftRepr()
    }}

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        let maybeEnum = __swift_bridge__$Vec_{enum_name}$get_mut(vecPtr, index)
        return maybeEnum.intoSwiftRepr()
    }}

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {{
        __swift_bridge__$Vec_{enum_name}$len(vecPtr)
    }}
}}"#
            )
        };

        let swift_enum = format!(
            r#"public enum {enum_name} {{{variants}}}
extension {enum_name} {{
    func intoFfiRepr() -> {ffi_repr_name} {{
        switch self {{{convert_swift_to_ffi_repr}}}
    }}
}}
extension {enum_ffi_name} {{
    func intoSwiftRepr() -> {enum_name} {{
        switch self.tag {{{convert_ffi_repr_to_swift}}}
    }}
}}
extension {option_ffi_name} {{
    @inline(__always)
    func intoSwiftRepr() -> Optional<{enum_name}> {{
        if self.is_some {{
            return self.val.intoSwiftRepr()
        }} else {{
            return nil
        }}
    }}

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<{enum_name}>) -> {option_ffi_name} {{
        if let v = val {{
            return {option_ffi_name}(is_some: true, val: v.intoFfiRepr())
        }} else {{
            return {option_ffi_name}(is_some: false, val: {ffi_repr_name}())
        }}
    }}
}}{vectorizable_impl}"#,
            enum_name = enum_name,
            enum_ffi_name = enum_ffi_name,
            option_ffi_name = option_ffi_name,
            ffi_repr_name = shared_enum.ffi_name_string(),
            variants = variants,
            convert_swift_to_ffi_repr = convert_swift_to_ffi_repr,
            convert_ffi_repr_to_swift = convert_ffi_repr_to_swift
        );

        Some(swift_enum)
    }
}
