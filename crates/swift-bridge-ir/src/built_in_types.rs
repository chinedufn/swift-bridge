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
    Vec(BuiltInVec),
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

/// Vec<T>
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInVec {
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
        if string.starts_with("Vec < ") {
            let inner = string.trim_start_matches("Vec < ");
            let inner = inner.trim_end_matches(" >");
            let inner = BuiltInType::with_str(inner)?;

            return Some(BuiltInType::Vec(BuiltInVec {
                ty: Box::new(inner),
            }));
        }

        let ty = match string {
            "u8" => BuiltInType::U8,
            "i8" => BuiltInType::I8,
            "u16" => BuiltInType::U16,
            "i16" => BuiltInType::I16,
            "u32" => BuiltInType::U32,
            "i32" => BuiltInType::I32,
            "u64" => BuiltInType::U64,
            "i64" => BuiltInType::I64,
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

    // Convert the BuiltInType to the corresponding Rust type.
    // U8 -> u8
    // Vec<U32> -> Vec<u32>
    fn to_rust(&self) -> TokenStream {
        match self {
            BuiltInType::Null => {
                quote! {()}
            }
            BuiltInType::U8 => quote! { u8 },
            BuiltInType::I8 => quote! { i8 },
            BuiltInType::U16 => quote! { u16},
            BuiltInType::I16 => quote! { i16},
            BuiltInType::U32 => quote! { u32 },
            BuiltInType::I32 => quote! { i32 },
            BuiltInType::U64 => quote! { u64 },
            BuiltInType::I64 => quote! { i64 },
            BuiltInType::Usize => quote! { usize },
            BuiltInType::Isize => quote! { isize },
            BuiltInType::F32 => quote! { f32 },
            BuiltInType::F64 => quote! { f64 },
            BuiltInType::Bool => quote! { bool },
            BuiltInType::Pointer(ptr) => {
                let maybe_mut = match ptr.kind {
                    PointerKind::Const => {
                        quote! {}
                    }
                    PointerKind::Mut => {
                        quote! { mut }
                    }
                };
                let ty = ptr.ty.to_rust();

                quote! { * #maybe_mut #ty }
            }
            BuiltInType::RefSlice(ref_slice) => {
                let ty = ref_slice.ty.to_rust();
                quote! { &[#ty]}
            }
            BuiltInType::Str => quote! { &str },
            BuiltInType::String => quote! { String },
            BuiltInType::Vec(v) => {
                let ty = v.ty.to_rust();
                quote! { Vec<#ty> }
            }
        }
    }

    // Get the corresponding Rust type for this Built in type
    //
    // U8 -> u8
    // RefSlice(U8) -> FfiSlice
    // Str -> RustStr
    pub fn to_ffi_compatible_rust_type(&self, swift_bridge_path: &Path) -> TokenStream {
        let ty = match self {
            BuiltInType::U8 => quote! {u8},
            BuiltInType::I8 => quote! { i8 },
            BuiltInType::U16 => quote! { u16 },
            BuiltInType::I16 => quote! { i16 },
            BuiltInType::U32 => quote! { u32 },
            BuiltInType::I32 => quote! { i32 },
            BuiltInType::U64 => quote! { u64 },
            BuiltInType::I64 => quote! { i64 },
            BuiltInType::F32 => quote! { f32 },
            BuiltInType::F64 => quote! { f64 },
            BuiltInType::Usize => quote! { usize },
            BuiltInType::Isize => quote! { isize },
            BuiltInType::Bool => quote! { bool },
            BuiltInType::Pointer(ptr) => {
                let ty = ptr.ty.to_ffi_compatible_rust_type(swift_bridge_path);
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
                let ty = slice.ty.to_ffi_compatible_rust_type(swift_bridge_path);
                quote! {#swift_bridge_path::FfiSlice<#ty>}
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
            BuiltInType::Vec(ty) => {
                let ty = ty.ty.to_rust();
                quote! { *mut Vec<#ty> }
            }
        };

        quote!(#ty)
    }

    pub fn to_swift_type(&self, must_be_c_compatible: bool) -> String {
        match self {
            BuiltInType::U8 => "UInt8".to_string(),
            BuiltInType::I8 => "Int8".to_string(),
            BuiltInType::U16 => "UInt16".to_string(),
            BuiltInType::I16 => "Int16".to_string(),
            BuiltInType::U32 => "UInt32".to_string(),
            BuiltInType::I32 => "Int32".to_string(),
            BuiltInType::U64 => "UInt64".to_string(),
            BuiltInType::I64 => "Int64".to_string(),
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
                    ptr.ty.to_swift_type(must_be_c_compatible)
                )
            }
            BuiltInType::RefSlice(slice) => {
                if must_be_c_compatible {
                    "__private__FfiSlice".to_string()
                } else {
                    format!(
                        "UnsafeBufferPointer<{}>",
                        slice.ty.to_swift_type(must_be_c_compatible)
                    )
                }
            }
            BuiltInType::Null => "()".to_string(),
            BuiltInType::Str => "RustStr".to_string(),
            BuiltInType::String => "RustString".to_string(),
            BuiltInType::Vec(ty) => {
                format!("RustVec<{}>", ty.ty.to_swift_type(must_be_c_compatible))
            }
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
            BuiltInType::RefSlice(_slice) => "struct __private__FfiSlice".to_string(),
            BuiltInType::Str => "struct RustStr".to_string(),
            BuiltInType::Null => "void".to_string(),
            BuiltInType::String => "void*".to_string(),
            BuiltInType::Vec(_) => "void*".to_string(),
        }
    }

    // Wrap an expression of this BuiltInType to be suitable to send from Rust to Swift.
    //
    // Examples:
    // If value foo is a String.. `foo` becomes `swiftbridge:string::RustString(foo)`
    // If value bar is a &str.. `bar` becomes `swiftbridge::string::RustStr::from_str(bar)`
    pub fn convert_rust_value_to_ffi_compatible_value(
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
                    #swift_bridge_path::FfiSlice::from_slice( #expression )
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
            BuiltInType::Vec(_) => {
                quote! { Box::into_raw(Box::new( #expression )) }
            }
        }
    }

    // Wrap an argument of this BuiltInType to convert it from it's FFI format to it's Rust type.
    //
    // Examples:
    // RustStr -> &str
    // *mut RustString -> String
    // FfiSlice<u8> -> &[u8]
    pub fn convert_ffi_value_to_rust_value(
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
            BuiltInType::Vec(_) => {
                quote! {
                    unsafe { Box::from_raw(#arg) }
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
    pub fn convert_ffi_value_to_swift_value(&self, value: &str) -> String {
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
            | BuiltInType::Usize
            | BuiltInType::Isize
            | BuiltInType::F32
            | BuiltInType::F64
            | BuiltInType::Bool => value.to_string(),
            BuiltInType::Pointer(_) => value.to_string(),
            BuiltInType::RefSlice(ty) => {
                format!(
                    "let slice = {value}; return UnsafeBufferPointer(start: slice.start.assumingMemoryBound(to: {ty}.self), count: Int(slice.len));",
                    value = value,
                    ty = ty.ty.to_swift_type(false)
                )
            }
            BuiltInType::Str => value.to_string(),
            BuiltInType::String => {
                format!("RustString(ptr: {})", value)
            }
            BuiltInType::Vec(ty) => {
                format!("RustVec(ptr: {}, isOwned: true)", value)
            }
        }
    }

    /// Convert from a Swift expression to it's FFI compatible counterpart.
    ///
    /// So.. Say we have an expression `value`
    ///
    /// If `value: UnsafeBufferPoint<T>` then `value` becomes:
    /// ```no_rust,ignore
    /// __private__FfiSlice(
    ///   start: UnsafeMutablePointer(mutating: value.baseAddress),
    ///   len: UInt(value.count)
    /// )
    /// ```
    ///
    pub fn convert_swift_expression_to_ffi_compatible(&self, value: &str) -> String {
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
            | BuiltInType::Usize
            | BuiltInType::Isize
            | BuiltInType::F32
            | BuiltInType::F64
            | BuiltInType::Bool
            | BuiltInType::Pointer(_) => value.to_string(),
            BuiltInType::RefSlice(_) => {
                //             format!(
                //                 r#"let buffer_pointer = {}
                // return __private__FfiSlice(start: UnsafeMutablePointer(mutating: buffer_pointer.baseAddress), len: UInt(buffer_pointer.count))"#,
                //                 value,
                //             )
                format!("{}.toFfiSlice()", value)
            }
            BuiltInType::Str => value.to_string(),
            BuiltInType::String => value.to_string(),
            BuiltInType::Vec(_) => {
                format!("{}.ptr", value)
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
            (quote! {usize}, BuiltInType::Usize),
            (quote! {isize}, BuiltInType::Isize),
            (quote! {f32}, BuiltInType::F32),
            (quote! {f64}, BuiltInType::F64),
            (quote! {&str}, BuiltInType::Str),
            (quote! {String}, BuiltInType::String),
            (
                quote! { Vec<u32>},
                BuiltInType::Vec(BuiltInVec {
                    ty: Box::new(BuiltInType::U32),
                }),
            ),
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
