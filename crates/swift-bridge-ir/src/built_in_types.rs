use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::ops::Deref;
use syn::{Path, ReturnType, Type};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum BuiltInType {
    Null,
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
    Bool,
    Pointer(BuiltInPointer),
    RefSlice(BuiltInRefSlice),
    /// &str
    Str,
    String,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInReference {
    pub mutable: bool,
    pub ty: Box<BuiltInType>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInPointer {
    pub kind: PointerKind,
    pub ty: Box<BuiltInType>,
}

/// &[T]
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInRefSlice {
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
                    Self::Pointer(BuiltInPointer {
                        kind,
                        ty: Box::new(ty),
                    })
                })
            }
            Type::Reference(ty_ref) => match ty_ref.elem.deref() {
                Type::Path(p) => {
                    let path = p.path.to_token_stream().to_string();
                    if path == "str" {
                        return Some(BuiltInType::Str);
                    }

                    None
                }
                Type::Slice(slice) => Self::with_type(&slice.elem)
                    .map(|ty| Self::RefSlice(BuiltInRefSlice { ty: Box::new(ty) })),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn with_return_type(ty: &ReturnType) -> Option<Self> {
        match ty {
            ReturnType::Default => Some(BuiltInType::Null),
            ReturnType::Type(_, ty) => BuiltInType::with_type(&ty),
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
            "String" => BuiltInType::String,
            "bool" => BuiltInType::Bool,
            _ => return None,
        };
        return Some(ty);
    }

    pub fn to_extern_rust_ident(&self, swift_bridge_path: &Path) -> TokenStream {
        let ty = match self {
            BuiltInType::U8 => quote! {u8},
            BuiltInType::I8 => quote! { i8 },
            BuiltInType::U16 => quote! { u16 },
            BuiltInType::I16 => quote! { i16 },
            BuiltInType::U32 => quote! { u32 },
            BuiltInType::I32 => quote! { i32 },
            BuiltInType::U64 => quote! { u64 },
            BuiltInType::I64 => quote! { i64 },
            BuiltInType::U128 => quote! { u128 },
            BuiltInType::I128 => quote! { i128 },
            BuiltInType::F32 => quote! { f32 },
            BuiltInType::F64 => quote! { f64 },
            BuiltInType::Usize => quote! { usize },
            BuiltInType::Isize => quote! { isize },
            BuiltInType::Bool => quote! { bool },
            BuiltInType::Pointer(ptr) => {
                let ty = ptr.ty.to_extern_rust_ident(swift_bridge_path);
                match ptr.kind {
                    PointerKind::Const => {
                        quote! {*const #ty }
                    }
                    PointerKind::Mut => {
                        quote! {*mut #ty }
                    }
                }
            }
            BuiltInType::RefSlice(slice) => {
                let ty = slice.ty.to_extern_rust_ident(swift_bridge_path);
                quote! {#swift_bridge_path::RustSlice<#ty>}
            }
            BuiltInType::Str => {
                quote! {#swift_bridge_path::string::RustStr}
            }
            BuiltInType::Null => {
                quote! { () }
            }
            BuiltInType::String => {
                quote! { *mut #swift_bridge_path::string::RustString }
            }
        };

        quote!(#ty)
    }

    pub fn to_swift(&self, must_be_c_compatible: bool) -> String {
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
            BuiltInType::Bool => "Bool".to_string(),
            BuiltInType::Pointer(ptr) => {
                let maybe_mutable = match ptr.kind {
                    PointerKind::Const => "",
                    PointerKind::Mut => "Mutable",
                };

                format!(
                    "Unsafe{}Pointer<{}>",
                    maybe_mutable,
                    ptr.ty.to_swift(must_be_c_compatible)
                )
            }
            BuiltInType::RefSlice(slice) => {
                if must_be_c_compatible {
                    "__private__RustSlice".to_string()
                } else {
                    format!(
                        "UnsafeBufferPointer<{}>",
                        slice.ty.to_swift(must_be_c_compatible)
                    )
                }
            }
            BuiltInType::Null => "()".to_string(),
            BuiltInType::Str => "RustStr".to_string(),
            BuiltInType::String => "RustString".to_string(),
        }
    }

    pub fn to_c(&self) -> String {
        match self {
            BuiltInType::U8 => "uint8_t".to_string(),
            BuiltInType::I8 => "int8_t".to_string(),
            BuiltInType::U16 => "uint16_t".to_string(),
            BuiltInType::I16 => "int16_t".to_string(),
            BuiltInType::U32 => "uint32_t".to_string(),
            BuiltInType::I32 => "int32_t".to_string(),
            BuiltInType::U64 => "uint64_t".to_string(),
            BuiltInType::I64 => "int64_t".to_string(),
            BuiltInType::U128 => "uint128_t".to_string(),
            BuiltInType::I128 => "i128_t".to_string(),
            BuiltInType::F32 => "float".to_string(),
            BuiltInType::F64 => "double".to_string(),
            BuiltInType::Usize => "uintptr_t".to_string(),
            BuiltInType::Isize => "intptr_t".to_string(),
            BuiltInType::Bool => "bool".to_string(),
            BuiltInType::Pointer(ptr) => {
                let maybe_const = match ptr.kind {
                    PointerKind::Const => " const ",
                    PointerKind::Mut => "",
                };
                format!("{}{}*", ptr.ty.to_c(), maybe_const)
            }
            BuiltInType::RefSlice(_slice) => "struct __private__RustSlice".to_string(),
            BuiltInType::Str => "struct RustStr".to_string(),
            BuiltInType::Null => "void".to_string(),
            BuiltInType::String => "void*".to_string(),
        }
    }

    // Wrap an expression of this BuiltInType to be suitable to send from Rust to Swift.
    //
    // Examples:
    // String -> swiftbridge:string::RustString
    // Str -> swiftbridge::string::RustStr
    pub fn wrap_rust_to_swift_expression_ffi_friendly(
        &self,
        swift_bridge_path: &Path,
        expression: &TokenStream,
    ) -> TokenStream {
        match self {
            BuiltInType::Null
            | BuiltInType::U8
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
            | BuiltInType::Isize
            | BuiltInType::F32
            | BuiltInType::F64
            | BuiltInType::Bool => {
                quote! { #expression }
            }
            BuiltInType::Pointer(_) => {
                quote! {
                    #expression
                }
            }
            BuiltInType::RefSlice(_) => {
                quote! {
                    #swift_bridge_path::RustSlice::from_slice( #expression )
                }
            }
            BuiltInType::Str => {
                quote! {
                    #swift_bridge_path::string::RustStr::from_str( #expression )
                }
            }
            BuiltInType::String => {
                quote! {
                    #swift_bridge_path::string::RustString( #expression ).box_into_raw()
                }
            }
        }
    }

    // Wrap an argument of this BuiltInType to convert it from it's FFI format to it's Rust type.
    //
    // Examples:
    // RustStr -> &str
    // *mut RustString -> String
    // RustSlice<u8> -> &[u8]
    pub fn wrap_swift_to_rust_arg_ffi_friendly(
        &self,
        _swift_bridge_path: &Path,
        arg: &TokenStream,
    ) -> TokenStream {
        match self {
            BuiltInType::Null
            | BuiltInType::U8
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
            | BuiltInType::Isize
            | BuiltInType::F32
            | BuiltInType::F64
            | BuiltInType::Bool => {
                quote! { #arg }
            }
            BuiltInType::Pointer(_) => {
                quote! { #arg }
            }
            BuiltInType::RefSlice(_reference) => {
                quote! { #arg.as_slice() }
            }
            BuiltInType::Str => {
                quote! { #arg.to_str() }
            }
            BuiltInType::String => {
                quote! {
                    unsafe { Box::from_raw(#arg).0 }
                }
            }
        }
    }

    // Used to wrap values returned from Rust
    //
    // Say we have an extern Rust function `create_string(str: &str) -> String`.
    // It would be called using `__swift_bridge__$create_string(str)`
    // But that would return a pointer to a swift_bridge::RustString.. So we need to convert that
    // to something Swift can make use of.
    // The final result on the Swift side would be:
    //
    // func create_string(_ str: RustStr) -> RustString {
    //     RustString(ptr: __swift_bridge__$create_string(str))
    // }
    //
    // Where this function converts
    //  `__swift_bridge__$create_string(str)` to `RustString(ptr: __swift_bridge__$create_string(str))`
    pub fn wrap_swift_called_rust_ffi_returned_value(&self, value: &str) -> String {
        match self {
            BuiltInType::Null
            | BuiltInType::U8
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
            | BuiltInType::Isize
            | BuiltInType::F32
            | BuiltInType::F64
            | BuiltInType::Bool => value.to_string(),
            BuiltInType::Pointer(_) => value.to_string(),
            BuiltInType::RefSlice(ty) => {
                format!(
                    r#"let slice = {value}; return UnsafeBufferPointer(start: slice.start.assumingMemoryBound(to: {ty}.self), count: Int(slice.len));"#,
                    value = value,
                    ty = ty.ty.to_swift(false)
                )
            }
            BuiltInType::Str => value.to_string(),
            BuiltInType::String => {
                format!("RustString(ptr: {})", value)
            }
        }
    }

    pub fn c_include(&self) -> Option<&'static str> {
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
            | BuiltInType::Isize => Some("stdint.h"),
            BuiltInType::Bool => Some("stdbool.h"),
            BuiltInType::Pointer(ptr) => ptr.ty.c_include(),
            BuiltInType::RefSlice(slice) => slice.ty.c_include(),
            _ => None,
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
            (quote! {&str}, BuiltInType::Str),
            (quote! {String}, BuiltInType::String),
            (
                quote! {*const u8},
                BuiltInType::Pointer(BuiltInPointer {
                    kind: PointerKind::Const,
                    ty: Box::new(BuiltInType::U8),
                }),
            ),
            (
                quote! {*mut f64},
                BuiltInType::Pointer(BuiltInPointer {
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
