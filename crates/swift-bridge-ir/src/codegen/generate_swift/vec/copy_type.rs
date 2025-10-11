use proc_macro2::Ident;

/// Generate the `extension MyRustType: Vectorizable {}` for the Swift side.
pub fn generate_vectorizable_extension(ty: &Ident) -> String {
    format!(
        r#"extension {ty}: Vectorizable {{
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {{
        __swift_bridge__$Vec_{ty}$new()
    }}

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {{
        __swift_bridge__$Vec_{ty}$drop(vecPtr)
    }}

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: {ty}) {{
        __swift_bridge__$Vec_{ty}$push(vecPtr, value.bytes)
    }}

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {{
        __swift_bridge__$Vec_{ty}$pop(vecPtr).intoSwiftRepr()
    }}

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Self? {{
        __swift_bridge__$Vec_{ty}$get(vecPtr, index).intoSwiftRepr()
    }}

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Self? {{
        __swift_bridge__$Vec_{ty}$get_mut(vecPtr, index).intoSwiftRepr()
    }}

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {{
        UnsafePointer<Self>(OpaquePointer(__swift_bridge__$Vec_{ty}$as_ptr(vecPtr)))
    }}

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {{
        __swift_bridge__$Vec_{ty}$len(vecPtr)
    }}
}}
"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_trimmed_generated_equals_trimmed_expected;
    use proc_macro2::Span;

    /// Verify that we generate the `extension MyRustType: Vectorizable { }` implementation
    /// for the Swift side.
    #[test]
    fn generates_vectorizable_extension() {
        let expected = r#"
extension ARustType: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ARustType$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ARustType$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ARustType) {
        __swift_bridge__$Vec_ARustType$push(vecPtr, value.bytes)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Self? {
        __swift_bridge__$Vec_ARustType$pop(vecPtr).intoSwiftRepr()
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Self? {
        __swift_bridge__$Vec_ARustType$get(vecPtr, index).intoSwiftRepr()
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Self? {
        __swift_bridge__$Vec_ARustType$get_mut(vecPtr, index).intoSwiftRepr()
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer<Self>(OpaquePointer(__swift_bridge__$Vec_ARustType$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ARustType$len(vecPtr)
    }
}
"#;

        assert_trimmed_generated_equals_trimmed_expected(
            &generate_vectorizable_extension(&Ident::new("ARustType", Span::call_site())),
            &expected,
        );
    }
}
