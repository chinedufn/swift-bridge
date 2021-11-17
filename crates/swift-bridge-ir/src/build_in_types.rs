use quote::ToTokens;
use syn::Type;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum BuiltInType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    Usize,
    Isize,
    F32,
    F64,
    Pointer(BuiltInTypePointer),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInTypePointer {
    pub kind: PointerKind,
    pub ty: Box<BuiltInType>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PointerKind {
    Const,
    Mut,
}

impl BuiltInType {
    pub fn with_type(ty: &Type) -> Option<Self> {
        match ty {
            Type::Path(path) => {
                Self::with_str(path.path.segments.to_token_stream().to_string().as_str())
            }
            Type::Ptr(ptr) => {
                let kind = if ptr.const_token.is_some() {
                    PointerKind::Const
                } else {
                    PointerKind::Mut
                };

                Self::with_type(&ptr.elem).map(|ty| {
                    Self::Pointer(BuiltInTypePointer {
                        kind,
                        ty: Box::new(ty),
                    })
                })
            }
            _ => None,
        }
    }

    pub fn with_str(string: &str) -> Option<BuiltInType> {
        let ty = match string {
            "u8" => BuiltInType::U8,
            "i8" => BuiltInType::I8,
            "u16" => BuiltInType::U16,
            "i16" => BuiltInType::I16,
            "u32" => BuiltInType::U32,
            "i32" => BuiltInType::I32,
            "u64" => BuiltInType::U64,
            "i64" => BuiltInType::I64,
            "u128" => BuiltInType::U128,
            "i128" => BuiltInType::I128,
            "usize" => BuiltInType::Usize,
            "isize" => BuiltInType::Isize,
            "f32" => BuiltInType::F32,
            "f64" => BuiltInType::F64,
            _ => return None,
        };
        return Some(ty);
    }

    pub fn to_swift(&self) -> String {
        match self {
            BuiltInType::U8 => "UInt8".to_string(),
            BuiltInType::I8 => "Int8".to_string(),
            BuiltInType::U16 => "UInt16".to_string(),
            BuiltInType::I16 => "Int16".to_string(),
            BuiltInType::U32 => "UInt32".to_string(),
            BuiltInType::I32 => "Int32".to_string(),
            BuiltInType::U64 => "UInt64".to_string(),
            BuiltInType::I64 => "Int64".to_string(),
            BuiltInType::U128 => "UInt128".to_string(),
            BuiltInType::I128 => "Int128".to_string(),
            BuiltInType::F32 => "Float".to_string(),
            BuiltInType::F64 => "Double".to_string(),
            BuiltInType::Usize => "UInt".to_string(),
            BuiltInType::Isize => "Int".to_string(),
            BuiltInType::Pointer(ptr) => {
                format!("UnsafeMutablePointer<{}>", ptr.ty.to_swift())
            }
        }
    }

    // FIXME: Delete this
    pub fn to_c(&self) -> &'static str {
        // match self {
        //     BuiltInType::U8 => "uint8_t",
        //     BuiltInType::I8 => "int8_t",
        //     BuiltInType::U16 => "uint16_t",
        //     BuiltInType::I16 => "int16_t",
        //     BuiltInType::U32 => "uint32_t",
        //     BuiltInType::I32 => "int32_t",
        //     BuiltInType::U64 => "uint64_t",
        //     BuiltInType::I64 => "int64_t",
        //     BuiltInType::U128 => "uint128_t",
        //     BuiltInType::I128 => "i128_t",
        //     BuiltInType::F32 => "float",
        //     BuiltInType::F64 => "double",
        //     BuiltInType::Usize => "uintptr_t",
        //     BuiltInType::Isize => "intptr_t",
        //     _ => todo!()
        // }
        unimplemented!("Delete this")
    }

    pub fn is_int(&self) -> bool {
        match self {
            BuiltInType::U8
            | BuiltInType::I8
            | BuiltInType::U16
            | BuiltInType::I16
            | BuiltInType::U32
            | BuiltInType::I32
            | BuiltInType::U64
            | BuiltInType::I64
            | BuiltInType::U128
            | BuiltInType::I128
            | BuiltInType::Usize
            | BuiltInType::Isize => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    /// Verify that we can parse built in types.
    #[test]
    fn build_in_types() {
        let tests = vec![
            (quote! {u8}, BuiltInType::U8),
            (quote! {i8}, BuiltInType::I8),
            (quote! {u16}, BuiltInType::U16),
            (quote! {i16}, BuiltInType::I16),
            (quote! {u32}, BuiltInType::U32),
            (quote! {i32}, BuiltInType::I32),
            (quote! {u64}, BuiltInType::U64),
            (quote! {i64}, BuiltInType::I64),
            (quote! {u128}, BuiltInType::U128),
            (quote! {i128}, BuiltInType::I128),
            (quote! {usize}, BuiltInType::Usize),
            (quote! {isize}, BuiltInType::Isize),
            (quote! {f32}, BuiltInType::F32),
            (quote! {f64}, BuiltInType::F64),
            (
                quote! {*const u8},
                BuiltInType::Pointer(BuiltInTypePointer {
                    kind: PointerKind::Const,
                    ty: Box::new(BuiltInType::U8),
                }),
            ),
            (
                quote! {*mut f64},
                BuiltInType::Pointer(BuiltInTypePointer {
                    kind: PointerKind::Mut,
                    ty: Box::new(BuiltInType::F64),
                }),
            ),
        ];
        for (tokens, expected) in tests {
            let ty: Type = parse_quote! {#tokens};
            assert_eq!(
                BuiltInType::with_type(&ty),
                Some(expected),
                "{}",
                tokens.to_string()
            )
        }
    }
}
