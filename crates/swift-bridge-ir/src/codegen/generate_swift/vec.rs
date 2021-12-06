use proc_macro2::Ident;

/// Generate the `extension MyRustType: Vectorizable {}` for the Swift side.
pub(super) fn generate_vectorizable_extension(ty: &Ident) -> String {
    format!(
        r#"extension {ty}: Vectorizable {{
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {{
        __swift_bridge__$Vec_{ty}$new()
    }}
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {{
        __swift_bridge__$Vec_{ty}$drop(vecPtr)
    }}
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: {ty}) {{
        __swift_bridge__$Vec_{ty}$push(vecPtr, {{value.isOwned = false; return value.ptr;}}())
    }}
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {{
        let pointer = __swift_bridge__$Vec_{ty}$pop(vecPtr)
        if pointer == nil {{
            return nil
        }} else {{
            return ({ty}(ptr: pointer!, isOwned: true) as! Self)
        }}
    }}
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        let pointer = __swift_bridge__$Vec_{ty}$get(vecPtr, index)
        if pointer == nil {{
            return nil
        }} else {{
            return ({ty}(ptr: pointer!, isOwned: false) as! Self)
        }}
    }}
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {{
        __swift_bridge__$Vec_{ty}$len(vecPtr)
    }}
}}
"#,
        ty = ty.to_string()
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
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ARustType$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ARustType$drop(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ARustType) {
        __swift_bridge__$Vec_ARustType$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ARustType$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ARustType(ptr: pointer!, isOwned: true) as! Self)
        }
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ARustType$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return (ARustType(ptr: pointer!, isOwned: false) as! Self)
        }
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
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
