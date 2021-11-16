use quote::ToTokens;
use syn::parse::Parse;
use syn::Type;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum SupportedType {
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
    F32,
    F64,
}

impl SupportedType {
    pub fn with_type(ty: &Type) -> Option<Self> {
        match ty {
            Type::Path(path) => {
                let ty = match path.path.segments.to_token_stream().to_string().as_str() {
                    "u8" => SupportedType::U8,
                    "i8" => SupportedType::I8,
                    "u16" => SupportedType::U16,
                    "i16" => SupportedType::I16,
                    "u32" => SupportedType::U32,
                    "i32" => SupportedType::I32,
                    "u64" => SupportedType::U64,
                    "i64" => SupportedType::I64,
                    "u128" => SupportedType::U128,
                    "i128" => SupportedType::I128,
                    "f32" => SupportedType::F32,
                    "f64" => SupportedType::F64,
                    _ => return None,
                };
                Some(ty)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    /// Verify that we can parse supported types.
    #[test]
    fn supported_types() {
        let tests = vec![
            (quote! {u8}, SupportedType::U8),
            (quote! {i8}, SupportedType::I8),
            (quote! {u16}, SupportedType::U16),
            (quote! {i16}, SupportedType::I16),
            (quote! {u32}, SupportedType::U32),
            (quote! {i32}, SupportedType::I32),
            (quote! {u64}, SupportedType::U64),
            (quote! {i64}, SupportedType::I64),
            (quote! {u128}, SupportedType::U128),
            (quote! {i128}, SupportedType::I128),
            (quote! {f32}, SupportedType::F32),
            (quote! {f64}, SupportedType::F64),
        ];
        for (tokens, expected) in tests {
            let ty: Type = parse_quote! {#tokens};
            assert_eq!(
                SupportedType::with_type(&ty),
                Some(expected),
                "{}",
                tokens.to_string()
            )
        }
    }
}
