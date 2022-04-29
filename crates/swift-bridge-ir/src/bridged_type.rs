use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::{quote, quote_spanned};
use syn::{FnArg, Pat, PatType, Path, ReturnType, Type};

use crate::parse::{HostLang, OpaqueRustTypeGenerics, TypeDeclarations};
use crate::SWIFT_BRIDGE_PREFIX;

use self::bridged_option::BridgedOption;
pub(crate) use self::shared_enum::{EnumVariant, SharedEnum};
pub(crate) use self::shared_struct::{SharedStruct, StructFields, StructSwiftRepr};

mod bridged_option;
mod shared_enum;
mod shared_struct;

// FIXME: Rename to BridgedType
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum BridgedType {
    StdLib(StdLibType),
    Foreign(CustomBridgedType),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CustomBridgedType {
    Shared(SharedType),
    Opaque(OpaqueForeignType),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StdLibType {
    Null,
    // TODO: Move the ints, floats, and bool types into enum StdLibPrimitive since they tend
    //  to have the same or similar codegen. This lets us add their codegen methods to this
    //  new enum, which will help clean up some of our Option handling code where we match on
    //  all inner types.
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
    /// `*const T` or `*mut T`
    Pointer(BuiltInPointer),
    /// `&[T]` or `&mut [T]`
    RefSlice(BuiltInRefSlice),
    /// &str
    Str,
    String,
    Vec(BuiltInVec),
    Option(BridgedOption),
}

/// TODO: Add this to `OpaqueForeignType`
#[derive(Debug, Copy, Clone)]
pub(crate) enum TypePosition {
    FnArg(HostLang),
    FnReturn(HostLang),
    SharedStructField,
    SwiftCallsRustAsyncOnCompleteReturnTy,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInPointer {
    pub kind: PointerKind,
    pub pointee: Pointee,
}

/// The target of an `*const` or `*mut` pointer.
#[derive(Clone)]
pub(crate) enum Pointee {
    BuiltIn(Box<BridgedType>),
    /// `*const SomeType`
    ///         ^^^^^^^^ This is the Pointee
    Void(Type),
}

impl ToTokens for Pointee {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Pointee::BuiltIn(built_in) => {
                built_in.to_rust().to_tokens(tokens);
            }
            Pointee::Void(ty) => {
                ty.to_tokens(tokens);
            }
        };
    }
}

impl Debug for Pointee {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pointee::BuiltIn(built_in) => f.debug_tuple("BuiltIn").field(&built_in).finish(),
            Pointee::Void(ty) => f
                .debug_tuple("Void")
                .field(&ty.to_token_stream().to_string())
                .finish(),
        }
    }
}

impl PartialEq for Pointee {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BuiltIn(left), Self::BuiltIn(right)) => left == right,
            (Self::Void(left), Self::Void(right)) => {
                left.to_token_stream().to_string() == right.to_token_stream().to_string()
            }
            _ => false,
        }
    }
}

/// &[T]
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInRefSlice {
    pub ty: Box<BridgedType>,
}

/// Vec<T>
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInVec {
    pub ty: Box<BridgedType>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PointerKind {
    Const,
    Mut,
}

impl ToTokens for PointerKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PointerKind::Const => {
                let t = quote! { *const };
                t.to_tokens(tokens);
            }
            PointerKind::Mut => {
                let t = quote! { *mut };
                t.to_tokens(tokens);
            }
        }
    }
}

impl BridgedType {
    pub fn is_null(&self) -> bool {
        matches!(self, BridgedType::StdLib(StdLibType::Null))
    }
}

#[cfg(test)]
impl BridgedType {
    fn unwrap_stdlib(&self) -> &StdLibType {
        match self {
            BridgedType::StdLib(s) => s,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SharedType {
    Struct(SharedStruct),
    Enum(SharedEnum),
}

#[derive(Clone)]
pub(crate) struct OpaqueForeignType {
    pub ty: Ident,
    pub host_lang: HostLang,
    pub reference: bool,
    pub mutable: bool,
    pub has_swift_bridge_copy_annotation: bool,
    pub generics: OpaqueRustTypeGenerics,
}

impl OpaqueForeignType {
    fn swift_name(&self) -> String {
        format!("{}", self.ty)
    }

    /// The name of the type used to pass a `#[swift_bridge(Copy(...))]` type over FFI
    fn copy_rust_repr_type(&self) -> TokenStream {
        let ty = format!("{}{}", SWIFT_BRIDGE_PREFIX, self.ty);
        let ty = Ident::new(&ty, self.ty.span());
        quote! {
            #ty
        }
    }

    /// The name of the type used to pass a `#[swift_bridge(Copy(...))]` type over FFI
    fn copy_ffi_repr_type(&self) -> String {
        format!("{}${}", SWIFT_BRIDGE_PREFIX, self.ty)
    }
}

impl Debug for OpaqueForeignType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpaqueForeignType")
            .field("ty", &self.ty.to_token_stream())
            .field("host_lang", &self.host_lang)
            .field("reference", &self.reference)
            .field("mutable", &self.mutable)
            .finish()
    }
}

impl PartialEq for OpaqueForeignType {
    fn eq(&self, other: &Self) -> bool {
        self.ty.to_token_stream().to_string() == other.ty.to_token_stream().to_string()
            && self.host_lang == other.host_lang
            && self.reference == other.reference
            && self.mutable == other.mutable
    }
}

/// Whether or not a PatType's pattern is `self`.
///
/// `self: &Foo` would be true
/// `self: &mut Foo` would be true
/// `self: Foo` would be true
/// `arg: &Foo` would be false.
pub(crate) fn pat_type_pat_is_self(pat_type: &PatType) -> bool {
    match pat_type.pat.deref() {
        syn::Pat::Ident(pat_ident) if pat_ident.ident == "self" => true,
        _ => false,
    }
}

/// foo: u8 -> Some("foo")
pub(crate) fn fn_arg_name(fn_arg: &FnArg) -> Option<&Ident> {
    match fn_arg {
        FnArg::Receiver(_) => None,
        FnArg::Typed(pat_ty) => match pat_ty.pat.deref() {
            Pat::Ident(i) => Some(&i.ident),
            _ => None,
        },
    }
}

impl Deref for OpaqueForeignType {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl BridgedType {
    pub fn new_with_type(ty: &Type, types: &TypeDeclarations) -> Option<Self> {
        match ty {
            Type::Path(path) => {
                if let Some(ty) = types.get_with_type_path(path) {
                    Some(ty.to_bridged_type(false, false))
                } else {
                    Self::new_with_str(
                        path.path.segments.to_token_stream().to_string().as_str(),
                        types,
                    )
                }
            }
            Type::Ptr(ptr) => {
                let kind = if ptr.const_token.is_some() {
                    PointerKind::Const
                } else {
                    PointerKind::Mut
                };

                let ty = if let Some(ty) = Self::new_with_type(&ptr.elem, types) {
                    BridgedType::StdLib(StdLibType::Pointer(BuiltInPointer {
                        kind,
                        pointee: Pointee::BuiltIn(Box::new(ty)),
                    }))
                } else {
                    BridgedType::StdLib(StdLibType::Pointer(BuiltInPointer {
                        kind,
                        pointee: Pointee::Void(*ptr.elem.clone()),
                    }))
                };
                Some(ty)
            }
            Type::Reference(ty_ref) => match ty_ref.elem.deref() {
                Type::Path(p) => {
                    if let Some(ty) = types.get_with_type_path(p) {
                        Some(ty.to_bridged_type(true, ty_ref.mutability.is_some()))
                    } else {
                        let path = p.path.to_token_stream().to_string();
                        if path == "str" {
                            return Some(BridgedType::StdLib(StdLibType::Str));
                        }

                        None
                    }
                }
                Type::Slice(slice) => Self::new_with_type(&slice.elem, types).map(|ty| {
                    BridgedType::StdLib(StdLibType::RefSlice(BuiltInRefSlice { ty: Box::new(ty) }))
                }),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn new_with_return_type(ty: &ReturnType, types: &TypeDeclarations) -> Option<Self> {
        match ty {
            ReturnType::Default => Some(BridgedType::StdLib(StdLibType::Null)),
            ReturnType::Type(_, ty) => BridgedType::new_with_type(&ty, types),
        }
    }

    pub fn new_with_fn_arg(fn_arg: &FnArg, types: &TypeDeclarations) -> Option<Self> {
        match fn_arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_ty) => BridgedType::new_with_type(&pat_ty.ty, types),
        }
    }

    pub fn new_with_str(string: &str, types: &TypeDeclarations) -> Option<BridgedType> {
        if string.starts_with("Vec < ") {
            let inner = string.trim_start_matches("Vec < ");
            let inner = inner.trim_end_matches(" >");

            let inner = if let Some(declared_ty) = types.get(inner) {
                declared_ty.to_bridged_type(false, false)
            } else {
                let inner: Type = syn::parse2(TokenStream::from_str(inner).unwrap()).unwrap();
                BridgedType::new_with_type(&inner, types)?
            };

            return Some(BridgedType::StdLib(StdLibType::Vec(BuiltInVec {
                ty: Box::new(inner),
            })));
        } else if string.starts_with("Option < ") {
            let inner = string.trim_start_matches("Option < ");
            let inner = inner.trim_end_matches(" >");

            let inner: Type = syn::parse2(TokenStream::from_str(inner).unwrap()).unwrap();
            let inner = BridgedType::new_with_type(&inner, types)?;

            return Some(BridgedType::StdLib(StdLibType::Option(BridgedOption {
                ty: Box::new(inner),
            })));
        }

        let ty = match string {
            "u8" => BridgedType::StdLib(StdLibType::U8),
            "i8" => BridgedType::StdLib(StdLibType::I8),
            "u16" => BridgedType::StdLib(StdLibType::U16),
            "i16" => BridgedType::StdLib(StdLibType::I16),
            "u32" => BridgedType::StdLib(StdLibType::U32),
            "i32" => BridgedType::StdLib(StdLibType::I32),
            "u64" => BridgedType::StdLib(StdLibType::U64),
            "i64" => BridgedType::StdLib(StdLibType::I64),
            "usize" => BridgedType::StdLib(StdLibType::Usize),
            "isize" => BridgedType::StdLib(StdLibType::Isize),
            "f32" => BridgedType::StdLib(StdLibType::F32),
            "f64" => BridgedType::StdLib(StdLibType::F64),
            "String" => BridgedType::StdLib(StdLibType::String),
            "bool" => BridgedType::StdLib(StdLibType::Bool),
            _ => {
                return None;
            }
        };
        return Some(ty);
    }

    // Convert the BuiltInType to the corresponding Rust type.
    // U8 -> u8
    // Vec<U32> -> Vec<u32>
    fn to_rust(&self) -> TokenStream {
        match self {
            BridgedType::StdLib(stdlib_type) => {
                match stdlib_type {
                    StdLibType::Null => {
                        quote! {()}
                    }
                    StdLibType::U8 => quote! { u8 },
                    StdLibType::I8 => quote! { i8 },
                    StdLibType::U16 => quote! { u16},
                    StdLibType::I16 => quote! { i16},
                    StdLibType::U32 => quote! { u32 },
                    StdLibType::I32 => quote! { i32 },
                    StdLibType::U64 => quote! { u64 },
                    StdLibType::I64 => quote! { i64 },
                    StdLibType::Usize => quote! { usize },
                    StdLibType::Isize => quote! { isize },
                    StdLibType::F32 => quote! { f32 },
                    StdLibType::F64 => quote! { f64 },
                    StdLibType::Bool => quote! { bool },
                    StdLibType::Pointer(ptr) => {
                        let ptr_kind = &ptr.kind;

                        match &ptr.pointee {
                            Pointee::BuiltIn(ty) => {
                                let ty = ty.to_rust();
                                quote! { #ptr_kind #ty}
                            }
                            Pointee::Void(_ty) => {
                                // quote! { * #ptr_kind #ty };
                                panic!("Add a test case that hits this branch, then make it pass")
                            }
                        }
                    }
                    StdLibType::RefSlice(ref_slice) => {
                        let ty = ref_slice.ty.to_rust();
                        quote! { &[#ty]}
                    }
                    StdLibType::Str => quote! { &str },
                    StdLibType::String => quote! { String },
                    StdLibType::Vec(v) => {
                        let ty = v.ty.to_rust();
                        quote! { Vec<#ty> }
                    }
                    StdLibType::Option(opt) => {
                        let ty = opt.ty.to_rust();
                        quote! { Option<#ty> }
                    }
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                let ty_name = &shared_struct.name;
                quote! {
                    #ty_name
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                //
                todo!("Shared enum to Rust type name")
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                let ty_name = &opaque.ty;

                if opaque.host_lang.is_rust() {
                    quote! {
                        super:: #ty_name
                    }
                } else {
                    quote! {
                        #ty_name
                    }
                }
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
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::U8 => quote! {u8},
                StdLibType::I8 => quote! { i8 },
                StdLibType::U16 => quote! { u16 },
                StdLibType::I16 => quote! { i16 },
                StdLibType::U32 => quote! { u32 },
                StdLibType::I32 => quote! { i32 },
                StdLibType::U64 => quote! { u64 },
                StdLibType::I64 => quote! { i64 },
                StdLibType::F32 => quote! { f32 },
                StdLibType::F64 => quote! { f64 },
                StdLibType::Usize => quote! { usize },
                StdLibType::Isize => quote! { isize },
                StdLibType::Bool => quote! { bool },
                StdLibType::Pointer(ptr) => {
                    let kind = ptr.kind.to_token_stream();

                    let ty = match &ptr.pointee {
                        Pointee::BuiltIn(ty) => ty.to_ffi_compatible_rust_type(swift_bridge_path),
                        Pointee::Void(ty) => {
                            quote! { super::#ty }
                        }
                    };

                    quote! { #kind #ty}
                }
                StdLibType::RefSlice(slice) => {
                    let ty = slice.ty.to_ffi_compatible_rust_type(swift_bridge_path);
                    quote! {#swift_bridge_path::FfiSlice<#ty>}
                }
                StdLibType::Str => {
                    quote! {#swift_bridge_path::string::RustStr}
                }
                StdLibType::Null => {
                    quote! { () }
                }
                StdLibType::String => {
                    quote! { *mut #swift_bridge_path::string::RustString }
                }
                StdLibType::Vec(ty) => {
                    let ty = ty.ty.to_rust();
                    quote! { *mut Vec<#ty> }
                }
                StdLibType::Option(opt) => match opt.ty.deref() {
                    BridgedType::StdLib(stdlib_ty) => match stdlib_ty {
                        StdLibType::Null => {
                            todo!("Option<()> is not yet supported")
                        }
                        StdLibType::U8 => {
                            quote! { #swift_bridge_path::option::OptionU8 }
                        }
                        StdLibType::I8 => {
                            quote! { #swift_bridge_path::option::OptionI8 }
                        }
                        StdLibType::U16 => {
                            quote! { #swift_bridge_path::option::OptionU16 }
                        }
                        StdLibType::I16 => {
                            quote! { #swift_bridge_path::option::OptionI16 }
                        }
                        StdLibType::U32 => {
                            quote! { #swift_bridge_path::option::OptionU32 }
                        }
                        StdLibType::I32 => {
                            quote! { #swift_bridge_path::option::OptionI32 }
                        }
                        StdLibType::U64 => {
                            quote! { #swift_bridge_path::option::OptionU64 }
                        }
                        StdLibType::I64 => {
                            quote! { #swift_bridge_path::option::OptionI64 }
                        }
                        StdLibType::Usize => {
                            quote! { #swift_bridge_path::option::OptionUsize }
                        }
                        StdLibType::Isize => {
                            quote! { #swift_bridge_path::option::OptionIsize }
                        }
                        StdLibType::F32 => {
                            quote! { #swift_bridge_path::option::OptionF32 }
                        }
                        StdLibType::F64 => {
                            quote! { #swift_bridge_path::option::OptionF64 }
                        }
                        StdLibType::Bool => {
                            quote! { #swift_bridge_path::option::OptionBool }
                        }
                        StdLibType::Pointer(_) => {
                            todo!("Option<*const T> and Option<*mut T> are not yet supported")
                        }
                        StdLibType::RefSlice(_) => {
                            todo!("Option<&[T]> is not yet supported")
                        }
                        StdLibType::Str => {
                            quote! { #swift_bridge_path::string::RustStr }
                        }
                        StdLibType::String => opt.ty.to_ffi_compatible_rust_type(swift_bridge_path),
                        StdLibType::Vec(_) => {
                            todo!("Option<Vec<T>> is not yet supported")
                        }
                        StdLibType::Option(_) => {
                            todo!("Option<Option<T>> is not yet supported")
                        }
                    },
                    BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(
                        shared_struct,
                    ))) => {
                        let name = shared_struct.ffi_option_name_tokens();
                        quote! { #name }
                    }
                    BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(
                        shared_enum,
                    ))) => {
                        let name = shared_enum.ffi_option_name_tokens();
                        quote! { #name }
                    }
                    BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                        let type_name = &opaque.ty;

                        quote! { *mut super::#type_name }
                    }
                },
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                let ty_name = &shared_struct.name;
                let prefixed_ty_name = Ident::new(
                    &format!("{}{}", SWIFT_BRIDGE_PREFIX, ty_name),
                    ty_name.span(),
                );

                let prefixed_ty_name = if shared_struct.already_declared {
                    quote! { <super:: #ty_name as #swift_bridge_path::SharedStruct>::FfiRepr }
                } else {
                    quote! { #prefixed_ty_name }
                };

                prefixed_ty_name
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                let ffi_ty_name = shared_enum.ffi_name_tokens();

                quote! { #ffi_ty_name }
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                let ty_name = &opaque.ty;

                if opaque.has_swift_bridge_copy_annotation {
                    opaque.copy_rust_repr_type()
                } else {
                    if opaque.host_lang.is_rust() {
                        let generics = opaque.generics.angle_bracketed_generics_tokens();

                        if opaque.reference {
                            let ptr = if opaque.mutable {
                                quote! { *mut }
                            } else {
                                quote! { *const }
                            };

                            quote_spanned! {ty_name.span()=> #ptr super::#ty_name }
                        } else {
                            quote! { *mut super::#ty_name #generics }
                        }
                    } else {
                        quote! { #ty_name }
                    }
                }
            }
        };

        quote!(#ty)
    }

    // U8 -> UInt8
    // *const u32 -> UnsafePointer<UInt32>
    // ... etc
    pub fn to_swift_type(&self, type_pos: TypePosition, types: &TypeDeclarations) -> String {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::U8 => "UInt8".to_string(),
                StdLibType::I8 => "Int8".to_string(),
                StdLibType::U16 => "UInt16".to_string(),
                StdLibType::I16 => "Int16".to_string(),
                StdLibType::U32 => "UInt32".to_string(),
                StdLibType::I32 => "Int32".to_string(),
                StdLibType::U64 => "UInt64".to_string(),
                StdLibType::I64 => "Int64".to_string(),
                StdLibType::F32 => "Float".to_string(),
                StdLibType::F64 => "Double".to_string(),
                StdLibType::Usize => "UInt".to_string(),
                StdLibType::Isize => "Int".to_string(),
                StdLibType::Bool => "Bool".to_string(),
                StdLibType::Pointer(ptr) => {
                    let maybe_mutable = match ptr.kind {
                        PointerKind::Const => "",
                        PointerKind::Mut => "Mutable",
                    };

                    match &ptr.pointee {
                        Pointee::BuiltIn(ty) => {
                            format!(
                                "Unsafe{}Pointer<{}>",
                                maybe_mutable,
                                ty.to_swift_type(type_pos, types)
                            )
                        }
                        Pointee::Void(_) => {
                            format!("Unsafe{}RawPointer", maybe_mutable)
                        }
                    }
                }
                StdLibType::RefSlice(slice) => {
                    match type_pos {
                        TypePosition::FnArg(func_host_lang)
                        | TypePosition::FnReturn(func_host_lang) => {
                            if func_host_lang.is_swift() {
                                "__private__FfiSlice".to_string()
                            } else {
                                format!(
                                    "UnsafeBufferPointer<{}>",
                                    slice.ty.to_swift_type(type_pos, types)
                                )
                            }
                        }
                        TypePosition::SharedStructField => {
                            //
                            unimplemented!()
                        }
                        TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                            unimplemented!()
                        }
                    }
                }
                StdLibType::Null => "()".to_string(),
                StdLibType::Str => match type_pos {
                    TypePosition::FnArg(func_host_lang) => {
                        if func_host_lang.is_rust() {
                            "GenericToRustStr".to_string()
                        } else {
                            "RustStr".to_string()
                        }
                    }
                    TypePosition::FnReturn(_func_host_lang) => "RustStr".to_string(),
                    TypePosition::SharedStructField => "RustStr".to_string(),
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        unimplemented!()
                    }
                },
                StdLibType::String => match type_pos {
                    TypePosition::FnArg(_func_host_lang) => "GenericIntoRustString".to_string(),
                    TypePosition::FnReturn(_func_host_lang) => "RustString".to_string(),
                    TypePosition::SharedStructField => "RustString".to_string(),
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        "UnsafeMutableRawPointer?".to_string()
                    }
                },
                StdLibType::Vec(ty) => {
                    format!("RustVec<{}>", ty.ty.to_swift_type(type_pos, types))
                }
                StdLibType::Option(opt) => match type_pos {
                    TypePosition::FnArg(func_host_lang)
                    | TypePosition::FnReturn(func_host_lang) => {
                        if func_host_lang.is_swift() {
                            opt.ty.to_swift_type(type_pos, types)
                        } else {
                            format!("Optional<{}>", opt.ty.to_swift_type(type_pos, types))
                        }
                    }
                    TypePosition::SharedStructField => {
                        format!("Optional<{}>", opt.ty.to_swift_type(type_pos, types))
                    }
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        unimplemented!()
                    }
                },
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                match type_pos {
                    TypePosition::FnArg(func_host_lang)
                    | TypePosition::FnReturn(func_host_lang) => {
                        if func_host_lang.is_rust() {
                            shared_struct.swift_name_string()
                        } else {
                            shared_struct.ffi_name_string()
                        }
                    }
                    TypePosition::SharedStructField => shared_struct.swift_name_string(),
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        shared_struct.ffi_name_string()
                    }
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                match type_pos {
                    TypePosition::FnArg(func_host_lang)
                    | TypePosition::FnReturn(func_host_lang) => {
                        if func_host_lang.is_rust() {
                            shared_enum.swift_name_string()
                        } else {
                            shared_enum.ffi_name_string()
                        }
                    }
                    TypePosition::SharedStructField => shared_enum.swift_name_string(),
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        unimplemented!()
                    }
                }
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                if opaque.host_lang.is_rust() {
                    match type_pos {
                        TypePosition::FnArg(func_host_lang)
                        | TypePosition::FnReturn(func_host_lang) => {
                            if func_host_lang.is_rust() {
                                let mut class_name = opaque.ty.to_string();

                                if !opaque.has_swift_bridge_copy_annotation {
                                    if opaque.reference {
                                        class_name += "Ref";
                                    }

                                    if opaque.mutable {
                                        class_name += "Mut";
                                    }
                                }

                                format!(
                                    "{}{}",
                                    class_name,
                                    opaque
                                        .generics
                                        .angle_bracketed_generic_concrete_swift_types_string(types)
                                )
                            } else {
                                format!("UnsafeMutableRawPointer")
                            }
                        }
                        TypePosition::SharedStructField => {
                            //
                            unimplemented!()
                        }
                        TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                            unimplemented!()
                        }
                    }
                } else {
                    match type_pos {
                        TypePosition::FnArg(func_host_lang)
                        | TypePosition::FnReturn(func_host_lang) => {
                            if func_host_lang.is_rust() {
                                opaque.ty.to_string()
                            } else {
                                "__private__PointerToSwiftType".to_string()
                            }
                        }
                        TypePosition::SharedStructField => {
                            //
                            unimplemented!()
                        }
                        TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                            unimplemented!()
                        }
                    }
                }
            }
        }
    }

    pub fn to_c(&self) -> String {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::U8 => "uint8_t".to_string(),
                StdLibType::I8 => "int8_t".to_string(),
                StdLibType::U16 => "uint16_t".to_string(),
                StdLibType::I16 => "int16_t".to_string(),
                StdLibType::U32 => "uint32_t".to_string(),
                StdLibType::I32 => "int32_t".to_string(),
                StdLibType::U64 => "uint64_t".to_string(),
                StdLibType::I64 => "int64_t".to_string(),
                StdLibType::F32 => "float".to_string(),
                StdLibType::F64 => "double".to_string(),
                StdLibType::Usize => "uintptr_t".to_string(),
                StdLibType::Isize => "intptr_t".to_string(),
                StdLibType::Bool => "bool".to_string(),
                StdLibType::Pointer(ptr) => {
                    let maybe_const = match ptr.kind {
                        PointerKind::Const => " const ",
                        PointerKind::Mut => "",
                    };

                    match &ptr.pointee {
                        Pointee::BuiltIn(ty) => {
                            format!("{}{}*", ty.to_c(), maybe_const)
                        }
                        Pointee::Void(_) => "void*".to_string(),
                    }
                }
                StdLibType::RefSlice(_slice) => "struct __private__FfiSlice".to_string(),
                StdLibType::Str => "struct RustStr".to_string(),
                StdLibType::Null => "void".to_string(),
                StdLibType::String => "void*".to_string(),
                StdLibType::Vec(_) => "void*".to_string(),
                StdLibType::Option(opt) => opt.to_c(),
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                format!("struct {}", shared_struct.ffi_name_string())
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                format!("struct {}", shared_enum.ffi_name_string())
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                if opaque.host_lang.is_rust() {
                    if opaque.has_swift_bridge_copy_annotation {
                        format!("struct {}", opaque.copy_ffi_repr_type())
                    } else {
                        "void*".to_string()
                    }
                } else {
                    "struct __private__PointerToSwiftType".to_string()
                }
            }
        }
    }

    /// This function is used to convert `*const Type` -> `*const super::Type`
    ///
    /// If the BuiltInType is not a pointer, or it is a pointer to a built in type such as
    /// `*const u8`, it does not get transformed.
    ///
    /// ## Example Case
    ///
    /// If we have an:
    ///
    /// ```no_rust,ignore
    /// extern "Swift" {
    ///     fn void_pointers (arg1: *const SomeType) -> *mut AnotherType;
    /// }
    /// ```
    ///
    /// We want to generate:
    /// ```no_rust, ignore
    /// fn void_pointers (arg1: *const super::SomeType) -> *mut super::AnotherType {
    ///     unsafe { __swift_bridge__void_pointers(arg1) }
    /// }
    ///
    pub fn maybe_convert_pointer_to_super_pointer(&self) -> TokenStream {
        match self {
            BridgedType::StdLib(stdlib_type) => {
                match stdlib_type {
                    StdLibType::Pointer(pointer) => match &pointer.pointee {
                        Pointee::BuiltIn(_built_in) => {
                            //
                            self.to_rust()
                        }
                        Pointee::Void(_) => {
                            let pointer_kind = &pointer.kind;
                            let pointee = &pointer.pointee;

                            quote! { #pointer_kind super:: #pointee }
                        }
                    },
                    _ => self.to_rust(),
                }
            }
            _ => self.to_rust(),
        }
    }

    // Wrap an expression of this BuiltInType to be suitable to send from Rust to Swift.
    //
    // Examples:
    // If value foo is a String.. `foo` becomes `swiftbridge:string::RustString(foo)`
    // If value bar is a &str.. `bar` becomes `swiftbridge::string::RustStr::from_str(bar)`
    pub fn convert_rust_value_to_ffi_compatible_value(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
    ) -> TokenStream {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null
                | StdLibType::U8
                | StdLibType::I8
                | StdLibType::U16
                | StdLibType::I16
                | StdLibType::U32
                | StdLibType::I32
                | StdLibType::U64
                | StdLibType::I64
                | StdLibType::Usize
                | StdLibType::Isize
                | StdLibType::F32
                | StdLibType::F64
                | StdLibType::Bool => {
                    quote! { #expression }
                }
                StdLibType::Pointer(_) => {
                    quote! {
                        #expression
                    }
                }
                StdLibType::RefSlice(_) => {
                    quote! {
                        #swift_bridge_path::FfiSlice::from_slice( #expression )
                    }
                }
                StdLibType::Str => {
                    quote! {
                        #swift_bridge_path::string::RustStr::from_str( #expression )
                    }
                }
                StdLibType::String => {
                    quote! {
                        #swift_bridge_path::string::RustString( #expression ).box_into_raw()
                    }
                }
                StdLibType::Vec(_) => {
                    quote! { Box::into_raw(Box::new( #expression )) }
                }
                StdLibType::Option(opt) => {
                    opt.convert_rust_value_to_ffi_value(expression, swift_bridge_path)
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                quote! {
                    #expression.into_ffi_repr()
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                quote! {
                    #expression.into_ffi_repr()
                }
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                let ty_name = &opaque.ty;

                if opaque.host_lang.is_rust() {
                    if opaque.has_swift_bridge_copy_annotation {
                        let copy_ty = opaque.copy_rust_repr_type();
                        quote! {
                            #copy_ty::from_rust_repr(#expression)
                        }
                    } else if opaque.reference {
                        let ptr = if opaque.mutable {
                            quote! { *mut }
                        } else {
                            quote! { *const }
                        };

                        quote! {
                            #expression as #ptr super::#ty_name
                        }
                    } else {
                        let generics = opaque.generics.angle_bracketed_generics_tokens();
                        quote! {
                            Box::into_raw(Box::new(#expression)) as *mut super::#ty_name #generics
                        }
                    }
                } else {
                    quote! {
                        #expression
                    }
                }
            }
        }
    }

    // Wrap an argument of this BuiltInType to convert it from it's FFI format to it's Rust type.
    //
    // Examples:
    // RustStr -> &str
    // *mut RustString -> String
    // FfiSlice<u8> -> &[u8]
    pub fn convert_ffi_value_to_rust_value(&self, value: &TokenStream, span: Span) -> TokenStream {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null
                | StdLibType::U8
                | StdLibType::I8
                | StdLibType::U16
                | StdLibType::I16
                | StdLibType::U32
                | StdLibType::I32
                | StdLibType::U64
                | StdLibType::I64
                | StdLibType::Usize
                | StdLibType::Isize
                | StdLibType::F32
                | StdLibType::F64
                | StdLibType::Bool => {
                    quote_spanned! {span=> #value }
                }
                StdLibType::Pointer(_) => {
                    quote_spanned! {span=> #value }
                }
                StdLibType::RefSlice(_reference) => {
                    quote_spanned! {span=> #value.as_slice() }
                }
                StdLibType::Str => {
                    quote_spanned! {span=> #value.to_str() }
                }
                StdLibType::String => {
                    quote_spanned! {span=>
                        unsafe { Box::from_raw(#value).0 }
                    }
                }
                StdLibType::Vec(_) => {
                    quote_spanned! {span=>
                        unsafe { * Box::from_raw(#value) }
                    }
                }
                StdLibType::Option(bridged_option) => {
                    bridged_option.convert_ffi_value_to_rust_value(value)
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                quote_spanned! {span=>
                    #value.into_rust_repr()
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                quote_spanned! {span=>
                    #value.into_rust_repr()
                }
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                if opaque.host_lang.is_rust() {
                    if opaque.has_swift_bridge_copy_annotation {
                        let maybe_ref = if opaque.reference {
                            quote! {&}
                        } else {
                            quote! {}
                        };
                        quote! {
                            #maybe_ref #value.into_rust_repr()
                        }
                    } else if opaque.reference {
                        let maybe_mut = if opaque.mutable {
                            quote! { mut }
                        } else {
                            quote! {}
                        };

                        quote! {
                            unsafe {  & #maybe_mut * #value }
                        }
                    } else {
                        quote! {
                            unsafe { * Box::from_raw(  #value ) }
                        }
                    }
                } else {
                    if opaque.reference {
                        todo!("Handle referenced opaque Swift types")
                    } else {
                        quote! {
                            #value
                        }
                    }
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
    pub fn convert_ffi_value_to_swift_value(
        &self,
        value: &str,
        type_pos: TypePosition,
        types: &TypeDeclarations,
    ) -> String {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null
                | StdLibType::U8
                | StdLibType::I8
                | StdLibType::U16
                | StdLibType::I16
                | StdLibType::U32
                | StdLibType::I32
                | StdLibType::U64
                | StdLibType::I64
                | StdLibType::Usize
                | StdLibType::Isize
                | StdLibType::F32
                | StdLibType::F64
                | StdLibType::Bool => value.to_string(),
                StdLibType::Pointer(ptr) => match &ptr.pointee {
                    Pointee::BuiltIn(_) => value.to_string(),
                    Pointee::Void(_ty) => match ptr.kind {
                        PointerKind::Const => match type_pos {
                            TypePosition::FnArg(func_host_lang) => {
                                if func_host_lang.is_rust() {
                                    format!("UnsafeRawPointer({}!)", value)
                                } else {
                                    value.to_string()
                                }
                            }
                            TypePosition::FnReturn(_) => {
                                format!("UnsafeRawPointer({}!)", value)
                            }
                            TypePosition::SharedStructField => {
                                format!("UnsafeRawPointer({}!)", value)
                            }
                            TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                                unimplemented!()
                            }
                        },
                        PointerKind::Mut => value.to_string(),
                    },
                },
                StdLibType::RefSlice(ty) => {
                    format!(
                           "let slice = {value}; return UnsafeBufferPointer(start: slice.start.assumingMemoryBound(to: {ty}.self), count: Int(slice.len));",
                           value = value,
                           ty = ty.ty.to_swift_type(type_pos,types)
                       )
                }
                StdLibType::Str => value.to_string(),
                StdLibType::String => match type_pos {
                    TypePosition::FnArg(_)
                    | TypePosition::FnReturn(_)
                    | TypePosition::SharedStructField => {
                        format!("RustString(ptr: {})", value)
                    }
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        format!("RustString(ptr: {}!)", value)
                    }
                },
                StdLibType::Vec(_ty) => {
                    format!("RustVec(ptr: {})", value)
                }
                StdLibType::Option(opt) => opt.convert_ffi_expression_to_swift(value),
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                format!("{}.intoSwiftRepr()", value)
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                format!("{}.intoSwiftRepr()", value)
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                let mut ty_name = opaque.ty.to_string();

                if opaque.reference {
                    ty_name += "Ref";
                }
                if opaque.mutable {
                    ty_name += "Mut";
                }

                if opaque.host_lang.is_rust() {
                    if opaque.has_swift_bridge_copy_annotation {
                        format!(
                            "{ty_name}(bytes: {value})",
                            ty_name = ty_name,
                            value = value,
                        )
                    } else {
                        format!("{ty_name}(ptr: {value})", ty_name = ty_name, value = value,)
                    }
                } else {
                    format!(
                        "Unmanaged<{ty_name}>.fromOpaque({value}.ptr).takeRetainedValue()",
                        ty_name = ty_name,
                        value = value
                    )
                }
            }
        }
    }

    /// Convert from a Swift expression to it's FFI compatible counterpart.
    ///
    /// So.. Say we have an expression `value`
    ///
    /// If `value: UnsafeBufferPoint<T>` then `value` becomes: `value.toFfiSlice()`,
    /// where `.toFfiSlice()` is a function that creates a `__private__FfiSlice` which is
    /// C compatible.
    pub fn convert_swift_expression_to_ffi_compatible(
        &self,
        value: &str,
        type_pos: TypePosition,
    ) -> String {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null
                | StdLibType::U8
                | StdLibType::I8
                | StdLibType::U16
                | StdLibType::I16
                | StdLibType::U32
                | StdLibType::I32
                | StdLibType::U64
                | StdLibType::I64
                | StdLibType::Usize
                | StdLibType::Isize
                | StdLibType::F32
                | StdLibType::F64
                | StdLibType::Bool => value.to_string(),
                StdLibType::RefSlice(_) => {
                    format!("{}.toFfiSlice()", value)
                }
                StdLibType::Pointer(ptr) => match &ptr.pointee {
                    Pointee::BuiltIn(_) => value.to_string(),
                    Pointee::Void(_ty) => match type_pos {
                        TypePosition::FnArg(func_host_lang)
                        | TypePosition::FnReturn(func_host_lang) => {
                            if ptr.kind == PointerKind::Const && func_host_lang.is_rust() {
                                format!("UnsafeMutableRawPointer(mutating: {})", value)
                            } else {
                                value.to_string()
                            }
                        }
                        TypePosition::SharedStructField => {
                            todo!("Pointers in shared struct fields are not yet supported")
                        }
                        TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                            unimplemented!()
                        }
                    },
                },
                StdLibType::Str => match type_pos {
                    TypePosition::FnArg(func_host_lang)
                    | TypePosition::FnReturn(func_host_lang) => {
                        if func_host_lang.is_rust() {
                            format!("{val}AsRustStr", val = value)
                        } else {
                            value.to_string()
                        }
                    }
                    TypePosition::SharedStructField => {
                        todo!("&str in shared struct fields is not yet supported")
                    }
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        unimplemented!()
                    }
                },
                StdLibType::String => {
                    format!(
                        "{{ let rustString = {value}.intoRustString(); rustString.isOwned = false; return rustString.ptr }}()",
                        value = value
                    )
                }
                StdLibType::Vec(_) => {
                    format!(
                        "{{ let val = {value}; val.isOwned = false; return val.ptr }}()",
                        value = value
                    )
                }
                StdLibType::Option(option) => {
                    option.convert_swift_expression_to_ffi_compatible(value, type_pos)
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                format!("{}.intoFfiRepr()", value)
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                format!("{}.intoFfiRepr()", value)
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                let ty_name = &opaque.ty;

                if opaque.host_lang.is_rust() {
                    if opaque.has_swift_bridge_copy_annotation {
                        format!("{}.intoFfiRepr()", value)
                    } else if opaque.reference {
                        format!("{}.ptr", value)
                    } else {
                        match type_pos {
                            TypePosition::FnArg(func_host_lang)
                            | TypePosition::FnReturn(func_host_lang) => {
                                if func_host_lang.is_rust() {
                                    format!(
                                        "{{{}.isOwned = false; return {}.ptr;}}()",
                                        value, value
                                    )
                                } else {
                                    format!(
                                        "{type_name}(ptr: {value})",
                                        type_name = ty_name,
                                        value = value
                                    )
                                }
                            }
                            TypePosition::SharedStructField => {
                                todo!("Opaque types in shared struct fields are not yet supported")
                            }
                            TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                                unimplemented!()
                            }
                        }
                    }
                } else {
                    match type_pos {
                        TypePosition::FnArg(func_host_lang)
                        | TypePosition::FnReturn(func_host_lang) => {
                            if func_host_lang.is_rust() {
                                format!(
                                    "__private__PointerToSwiftType(ptr: Unmanaged.passRetained({}).toOpaque())",
                                    value
                                )
                            } else {
                                format!(
                                    "Unmanaged<{type_name}>.fromOpaque({value}.ptr).takeRetainedValue()",
                                    type_name = ty_name,
                                    value = value
                                )
                            }
                        }
                        TypePosition::SharedStructField => {
                            todo!("Opaque types in shared struct fields are not yet supported")
                        }
                        TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                            unimplemented!()
                        }
                    }
                }
            }
        }
    }

    pub fn c_include(&self) -> Option<&'static str> {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::U8
                | StdLibType::I8
                | StdLibType::U16
                | StdLibType::I16
                | StdLibType::U32
                | StdLibType::I32
                | StdLibType::U64
                | StdLibType::I64
                | StdLibType::Usize
                | StdLibType::Isize => Some("stdint.h"),
                StdLibType::Bool => Some("stdbool.h"),
                StdLibType::Pointer(ptr) => match &ptr.pointee {
                    Pointee::BuiltIn(ty) => ty.c_include(),
                    Pointee::Void(_) => None,
                },
                StdLibType::RefSlice(slice) => slice.ty.c_include(),
                StdLibType::Vec(_vec) => Some("stdint.h"),
                _ => None,
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                // TODO: Iterate over the fields and see if any of them need imports..
                None
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                // TODO: Iterate over the fields and see if any of them need imports..
                None
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(_opaque)) => None,
        }
    }

    /// When we want to return an Option::None we still need to return a dummy value to appease the
    /// type checker, even though it never gets used by the caller.
    fn rust_unused_option_none_val(&self, swift_bridge_path: &Path) -> UnusedOptionNoneValue {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null => UnusedOptionNoneValue {
                    rust: quote! { () },
                    swift: "()".into(),
                },
                StdLibType::U8
                | StdLibType::I8
                | StdLibType::U16
                | StdLibType::I16
                | StdLibType::U32
                | StdLibType::I32
                | StdLibType::U64
                | StdLibType::I64
                | StdLibType::Usize
                | StdLibType::Isize => UnusedOptionNoneValue {
                    rust: quote! { 123 },
                    swift: "123".into(),
                },
                StdLibType::F32 | StdLibType::F64 => UnusedOptionNoneValue {
                    rust: quote! { 0.123 },
                    swift: "0.123".into(),
                },
                StdLibType::Bool => UnusedOptionNoneValue {
                    rust: quote! { bool },
                    swift: "bool".into(),
                },
                StdLibType::Pointer(_) => {
                    todo!("Support Option<*const T> and Option<*mut T>")
                }
                StdLibType::RefSlice(_) => {
                    todo!("Support Option<&[T]>")
                }
                StdLibType::Str => {
                    UnusedOptionNoneValue {
                        rust: quote! {
                            #swift_bridge_path::string::RustStr {start: std::ptr::null::<u8>(), len: 0}
                        },
                        // TODO: Add integration tests:
                        //  Rust: crates/swift-integration-tests/src/option.rs
                        //  Swift: OptionTests.swift
                        swift: "TODO_SWIFT_OPTIONAL_STR_SUPPORT".to_string(),
                    }
                }
                StdLibType::String => {
                    UnusedOptionNoneValue {
                        rust: quote! {
                            std::ptr::null::<#swift_bridge_path::string::RustString>() as *mut #swift_bridge_path::string::RustString
                        },
                        // TODO: Add integration tests:
                        //  Rust: crates/swift-integration-tests/src/option.rs
                        //  Swift: OptionTests.swift
                        swift: "TODO_SWIFT_OPTIONAL_STRING_SUPPORT".to_string(),
                    }
                }
                StdLibType::Vec(_) => {
                    todo!("Support Option<Vec<T>>")
                }
                StdLibType::Option(_) => {
                    todo!("Support nested Option<Option<T>>")
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                let option_name = shared_struct.ffi_option_name_tokens();
                UnusedOptionNoneValue {
                    rust: quote! { #option_name { is_some: false, val: std::mem::MaybeUninit::uninit() } },
                    swift: "TODO..Support Swift Option<Enum>::None value".into(),
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                let option_name = shared_enum.ffi_option_name_tokens();
                UnusedOptionNoneValue {
                    rust: quote! { #option_name { is_some: false, val: std::mem::MaybeUninit::uninit() } },
                    swift: "TODO..Support Swift Option<Enum>::None value".into(),
                }
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                let ty_name = &opaque.ty;

                if opaque.reference {
                    todo!("Support returning Option<&T> where T is an opaque type")
                } else {
                    UnusedOptionNoneValue {
                        rust: quote! { std::ptr::null::<#ty_name>() as *mut super::#ty_name },
                        swift: "TODO..Support Swift Option<T>::None value".into(),
                    }
                }
            }
        }
    }

    /// Whether or not the type is a `String`, or a type that contains an owned String such as
    /// `Option<String>` or `struct Foo { field: String } `
    pub fn contains_owned_string_recursive(&self) -> bool {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::String => true,
                StdLibType::Vec(inner) => inner.ty.contains_owned_string_recursive(),
                StdLibType::Option(inner) => inner.ty.contains_owned_string_recursive(),
                _ => false,
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                // TODO: Check fields for String
                false
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                // TODO: Check fields for &str
                false
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(_)) => false,
        }
    }

    /// Whether or not the type is a `&str`, or a type that contains a &str such as
    /// `Option<&str>` or `struct Foo { field: &'static str } `
    pub fn contains_ref_string_recursive(&self) -> bool {
        match self {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Str => true,
                StdLibType::Vec(inner) => inner.ty.contains_ref_string_recursive(),
                StdLibType::Option(inner) => inner.ty.contains_ref_string_recursive(),
                _ => false,
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                // TODO: Check fields for &str
                false
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                // TODO: Check fields for &str
                false
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(_)) => false,
        }
    }

    /// Convert a rust expression into this type using
    pub fn rust_expression_into(&self, expression: &TokenStream) -> TokenStream {
        match self {
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                let struct_name = &shared_struct.name;

                let maybe_super = if shared_struct.already_declared {
                    quote! { super:: }
                } else {
                    quote! {}
                };

                quote! {
                    { let val: #maybe_super #struct_name = #expression.into(); val }
                }
            }
            // TODO: Instead of this catchall.. explicitly match on all variants and use
            //  a similar approach to how we handle shared structs
            _ => {
                quote! { #expression.into() }
            }
        }
    }
}

struct UnusedOptionNoneValue {
    rust: TokenStream,
    #[allow(unused)]
    swift: String,
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    /// Verify that we can parse std lib types.
    #[test]
    fn std_lib_types() {
        let tests = vec![
            (quote! {u8}, StdLibType::U8),
            (quote! {i8}, StdLibType::I8),
            (quote! {u16}, StdLibType::U16),
            (quote! {i16}, StdLibType::I16),
            (quote! {u32}, StdLibType::U32),
            (quote! {i32}, StdLibType::I32),
            (quote! {u64}, StdLibType::U64),
            (quote! {i64}, StdLibType::I64),
            (quote! {usize}, StdLibType::Usize),
            (quote! {isize}, StdLibType::Isize),
            (quote! {f32}, StdLibType::F32),
            (quote! {f64}, StdLibType::F64),
            (quote! {&str}, StdLibType::Str),
            (quote! {String}, StdLibType::String),
            (
                quote! { Vec<u32>},
                StdLibType::Vec(BuiltInVec {
                    ty: Box::new(BridgedType::StdLib(StdLibType::U32)),
                }),
            ),
            (
                quote! { Option<u32>},
                StdLibType::Option(BridgedOption {
                    ty: Box::new(BridgedType::StdLib(StdLibType::U32)),
                }),
            ),
            (
                quote! {*const u8},
                StdLibType::Pointer(BuiltInPointer {
                    kind: PointerKind::Const,
                    pointee: Pointee::BuiltIn(Box::new(BridgedType::StdLib(StdLibType::U8))),
                }),
            ),
            (
                quote! {*mut f64},
                StdLibType::Pointer(BuiltInPointer {
                    kind: PointerKind::Mut,
                    pointee: Pointee::BuiltIn(Box::new(BridgedType::StdLib(StdLibType::F64))),
                }),
            ),
            (
                quote! {*const c_void},
                StdLibType::Pointer(BuiltInPointer {
                    kind: PointerKind::Const,
                    pointee: Pointee::Void(syn::parse2(quote! {c_void}).unwrap()),
                }),
            ),
        ];
        for (tokens, expected) in tests {
            let ty: Type = parse_quote! {#tokens};
            assert_eq!(
                BridgedType::new_with_type(&ty, &TypeDeclarations::default())
                    .unwrap()
                    .unwrap_stdlib(),
                &expected,
                "{}",
                tokens.to_string()
            )
        }
    }
}
