use quote::ToTokens;
use syn::Type;

#[derive(Debug, PartialEq, Copy, Clone)]
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
}

impl BuiltInType {
    pub fn with_type(ty: &Type) -> Option<Self> {
        match ty {
            Type::Path(path) => {
                Self::with_str(path.path.segments.to_token_stream().to_string().as_str()) 
            }
            _ => None,
        }
    }
    
    pub fn with_str (string: &str) -> Option<BuiltInType> {
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
        return Some(ty)
        
    }

    pub fn to_swift(&self) -> &'static str {
        match self {
            BuiltInType::U8 => "UInt8",
            BuiltInType::I8 => "Int8",
            BuiltInType::U16 => "UInt16",
            BuiltInType::I16 => "Int16",
            BuiltInType::U32 => "UInt32",
            BuiltInType::I32 => "Int32",
            BuiltInType::U64 => "UInt64",
            BuiltInType::I64 => "Int64",
            BuiltInType::U128 => "UInt128",
            BuiltInType::I128 => "Int128",
            BuiltInType::F32 => "Float",
            BuiltInType::F64 => "Double",
            BuiltInType::Usize => "UInt",
            BuiltInType::Isize => "Int",
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
