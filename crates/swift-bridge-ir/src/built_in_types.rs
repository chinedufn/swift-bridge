use crate::parse::{HostLang, TypeDeclarations};
use crate::SWIFT_BRIDGE_PREFIX;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::ToTokens;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use syn::{FnArg, ForeignItemType, LitStr, PatType, Path, ReturnType, Type};

// FIXME: Rename to BridgedType
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum BuiltInType {
    StdLib(StdLibType),
    Foreign(ForeignBridgedType),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ForeignBridgedType {
    Shared(SharedType),
    Opaque(OpaqueForeignType),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StdLibType {
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
    /// `*const T` or `*mut T`
    Pointer(BuiltInPointer),
    /// `&[T]` or `&mut [T]`
    RefSlice(BuiltInRefSlice),
    /// &str
    Str,
    String,
    Vec(BuiltInVec),
    Option(BuiltInOption),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInReference {
    pub mutable: bool,
    pub ty: Box<BuiltInType>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInPointer {
    pub kind: PointerKind,
    pub pointee: Pointee,
}

/// The target of an `*const` or `*mut` pointer.
#[derive(Clone)]
pub(crate) enum Pointee {
    BuiltIn(Box<BuiltInType>),
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
    pub ty: Box<BuiltInType>,
}

/// Vec<T>
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInVec {
    pub ty: Box<BuiltInType>,
}

/// Option<T>
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BuiltInOption {
    pub ty: Box<BuiltInType>,
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

#[cfg(test)]
impl BuiltInType {
    fn unwrap_stdlib(&self) -> &StdLibType {
        match self {
            BuiltInType::StdLib(s) => s,
            _ => panic!(),
        }
    }

    fn _unwrap_shared(&self) -> &SharedType {
        match self {
            BuiltInType::Foreign(ForeignBridgedType::Shared(s)) => s,
            _ => panic!(),
        }
    }

    fn unwrap_shared_struct(&self) -> &SharedStruct {
        match self {
            BuiltInType::Foreign(ForeignBridgedType::Shared(SharedType::Struct(s))) => s,
            _ => panic!(),
        }
    }

    fn unwrap_opaque(&self) -> &OpaqueForeignType {
        match self {
            BuiltInType::Foreign(ForeignBridgedType::Opaque(o)) => o,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
impl ForeignBridgedType {
    pub fn unwrap_shared_struct(&self) -> &SharedStruct {
        match self {
            ForeignBridgedType::Shared(SharedType::Struct(s)) => s,
            _ => panic!(),
        }
    }

    pub fn unwrap_opaque(&self) -> &OpaqueForeignType {
        match self {
            ForeignBridgedType::Opaque(o) => o,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SharedType {
    Struct(SharedStruct),
}

#[derive(Clone)]
pub(crate) struct SharedStruct {
    pub name: Ident,
    pub swift_repr: StructSwiftRepr,
    pub fields: Vec<StructField>,
    pub swift_name: Option<LitStr>,
    pub fields_format: FieldsFormat,
}

impl PartialEq for SharedStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_string() == other.name.to_string()
            && self.swift_repr == other.swift_repr
            && self.fields == other.fields
            && self.swift_name.as_ref().map(|l| l.value())
                == other.swift_name.as_ref().map(|l| l.value())
            && self.fields_format == other.fields_format
    }
}

impl SharedStruct {
    pub fn swift_name_string(&self) -> String {
        self.swift_name
            .as_ref()
            .map(|s| s.value())
            .unwrap_or(self.name.to_string())
    }
}

impl Debug for SharedStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedStruct")
            .field("name", &self.name.to_string())
            .field("swift_repr", &self.swift_repr)
            .field("fields", &self.fields)
            .field("swift_name", &self.swift_name.as_ref().map(|l| l.value()))
            .field("fields_format", &self.fields_format)
            .finish()
    }
}

/// Whether to create a class or a structure when creating the Swift representation of a shared
/// struct.
///
/// https://docs.swift.org/swift-book/LanguageGuide/ClassesAndStructures.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum StructSwiftRepr {
    Class,
    /// # Invariants
    ///
    /// (These invariants aren't implemented yet)
    ///
    /// - Cannot be owned by Swift it it contains one or more fields that need to run destructors.
    ///   - Since Swift struct cannot run de-initializers on structs. Only on classes.
    /// - Can always be passed to Swift by immutable reference
    ///   - Since this means Swift does not need to run any de-initializers, which it cannot do
    ///     for structs.
    Structure,
}

#[derive(Clone)]
pub(crate) struct StructField {
    pub name: Option<Ident>,
    pub ty: Type,
}

impl PartialEq for StructField {
    fn eq(&self, other: &Self) -> bool {
        self.name.as_ref().map(|n| n.to_string()) == other.name.as_ref().map(|n| n.to_string())
            && self.ty.to_token_stream().to_string() == other.ty.to_token_stream().to_string()
    }
}

impl Debug for StructField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructField")
            .field("name", &self.name.as_ref().map(|n| n.to_string()))
            .field("ty", &self.ty.to_token_stream())
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum FieldsFormat {
    Named,
    Unnamed,
    Unit,
}

#[derive(Clone)]
pub(crate) struct OpaqueForeignType {
    pub ty: ForeignItemType,
    pub host_lang: HostLang,
}

impl Debug for OpaqueForeignType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpaqueForeignType")
            .field("ty", &self.ty.to_token_stream())
            .field("host_lang", &self.host_lang)
            .finish()
    }
}

impl PartialEq for OpaqueForeignType {
    fn eq(&self, other: &Self) -> bool {
        self.ty.to_token_stream().to_string() == other.ty.to_token_stream().to_string()
            && self.host_lang == other.host_lang
    }
}

impl OpaqueForeignType {
    // "__swift_bridge__$TypeName$_free"
    pub fn free_link_name(&self) -> String {
        format!(
            "{}${}$_free",
            SWIFT_BRIDGE_PREFIX,
            self.ty.ident.to_string()
        )
    }

    // "__swift_bridge__TypeName__free"
    pub fn free_func_name(&self) -> String {
        format!("{}{}__free", SWIFT_BRIDGE_PREFIX, self.ty.ident.to_string())
    }

    pub fn ty_name_ident(&self) -> &Ident {
        &self.ty.ident
    }
}

/// Whether or not a PatType's pattern is `self`.
///
/// `self: &Fpp` would be true
/// `arg: &Foo` would be false.
pub(crate) fn pat_type_pat_is_self(pat_type: &PatType) -> bool {
    match pat_type.pat.deref() {
        syn::Pat::Ident(pat_ident) if pat_ident.ident == "self" => true,
        _ => false,
    }
}

impl Deref for OpaqueForeignType {
    type Target = ForeignItemType;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl BuiltInType {
    pub fn new_with_type(ty: &Type, types: &TypeDeclarations) -> Option<Self> {
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

                let ty = if let Some(ty) = Self::new_with_type(&ptr.elem, types) {
                    BuiltInType::StdLib(StdLibType::Pointer(BuiltInPointer {
                        kind,
                        pointee: Pointee::BuiltIn(Box::new(ty)),
                    }))
                } else {
                    BuiltInType::StdLib(StdLibType::Pointer(BuiltInPointer {
                        kind,
                        pointee: Pointee::Void(*ptr.elem.clone()),
                    }))
                };
                Some(ty)
            }
            Type::Reference(ty_ref) => match ty_ref.elem.deref() {
                Type::Path(p) => {
                    let path = p.path.to_token_stream().to_string();
                    if path == "str" {
                        return Some(BuiltInType::StdLib(StdLibType::Str));
                    }

                    None
                }
                Type::Slice(slice) => Self::new_with_type(&slice.elem, types).map(|ty| {
                    BuiltInType::StdLib(StdLibType::RefSlice(BuiltInRefSlice { ty: Box::new(ty) }))
                }),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn new_with_return_type(ty: &ReturnType, types: &TypeDeclarations) -> Option<Self> {
        match ty {
            ReturnType::Default => Some(BuiltInType::StdLib(StdLibType::Null)),
            ReturnType::Type(_, ty) => BuiltInType::new_with_type(&ty, types),
        }
    }

    pub fn new_with_fn_arg(fn_arg: &FnArg, types: &TypeDeclarations) -> Option<Self> {
        match fn_arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_ty) => BuiltInType::new_with_type(&pat_ty.ty, types),
        }
    }

    pub fn with_str(string: &str) -> Option<BuiltInType> {
        if string.starts_with("Vec < ") {
            let inner = string.trim_start_matches("Vec < ");
            let inner = inner.trim_end_matches(" >");
            let inner = BuiltInType::with_str(inner)?;

            return Some(BuiltInType::StdLib(StdLibType::Vec(BuiltInVec {
                ty: Box::new(inner),
            })));
        } else if string.starts_with("Option < ") {
            let inner = string.trim_start_matches("Option < ");
            let inner = inner.trim_end_matches(" >");
            let inner = BuiltInType::with_str(inner)?;

            return Some(BuiltInType::StdLib(StdLibType::Option(BuiltInOption {
                ty: Box::new(inner),
            })));
        }

        let ty = match string {
            "u8" => BuiltInType::StdLib(StdLibType::U8),
            "i8" => BuiltInType::StdLib(StdLibType::I8),
            "u16" => BuiltInType::StdLib(StdLibType::U16),
            "i16" => BuiltInType::StdLib(StdLibType::I16),
            "u32" => BuiltInType::StdLib(StdLibType::U32),
            "i32" => BuiltInType::StdLib(StdLibType::I32),
            "u64" => BuiltInType::StdLib(StdLibType::U64),
            "i64" => BuiltInType::StdLib(StdLibType::I64),
            "usize" => BuiltInType::StdLib(StdLibType::Usize),
            "isize" => BuiltInType::StdLib(StdLibType::Isize),
            "f32" => BuiltInType::StdLib(StdLibType::F32),
            "f64" => BuiltInType::StdLib(StdLibType::F64),
            "String" => BuiltInType::StdLib(StdLibType::String),
            "bool" => BuiltInType::StdLib(StdLibType::Bool),
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
            BuiltInType::StdLib(stdlib_type) => {
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
            _ => todo!(),
        }
    }

    // Get the corresponding Rust type for this Built in type
    //
    // U8 -> u8
    // RefSlice(U8) -> FfiSlice
    // Str -> RustStr
    pub fn to_ffi_compatible_rust_type(&self, swift_bridge_path: &Path) -> TokenStream {
        let ty = match self {
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                StdLibType::Option(opt) => opt.ty.to_rust(),
            },
            _ => todo!(),
        };

        quote!(#ty)
    }

    // U8 -> UInt8
    // *const u32 -> UnsafePointer<UInt32>
    // ... etc
    pub fn to_swift_type(&self, must_be_c_compatible: bool) -> String {
        match self {
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                                ty.to_swift_type(must_be_c_compatible)
                            )
                        }
                        Pointee::Void(_) => {
                            format!("Unsafe{}RawPointer", maybe_mutable)
                        }
                    }
                }
                StdLibType::RefSlice(slice) => {
                    if must_be_c_compatible {
                        "__private__FfiSlice".to_string()
                    } else {
                        format!(
                            "UnsafeBufferPointer<{}>",
                            slice.ty.to_swift_type(must_be_c_compatible)
                        )
                    }
                }
                StdLibType::Null => "()".to_string(),
                StdLibType::Str => "RustStr".to_string(),
                StdLibType::String => "RustString".to_string(),
                StdLibType::Vec(ty) => {
                    format!("RustVec<{}>", ty.ty.to_swift_type(must_be_c_compatible))
                }
                StdLibType::Option(opt) => {
                    if must_be_c_compatible {
                        opt.ty.to_swift_type(must_be_c_compatible)
                    } else {
                        format!("Optional<{}>", opt.ty.to_swift_type(must_be_c_compatible))
                    }
                }
            },
            _ => panic!(),
        }
    }

    pub fn to_c(&self) -> String {
        match self {
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                StdLibType::Option(opt) => opt.ty.to_c(),
            },
            _ => todo!(),
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
            BuiltInType::StdLib(stdlib_type) => {
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
            _ => todo!(),
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
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                    let ty = opt.ty.to_rust();

                    quote! {
                        if let Some(val) = #expression {
                            #swift_bridge_path::option::_set_option_return(true);
                            val
                        } else {
                            #swift_bridge_path::option::_set_option_return(false);
                            <#ty as #swift_bridge_path::option::FfiOptional>::unused_value()
                        }
                    }
                }
            },
            _ => todo!(),
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
        value: &TokenStream,
        is_returned_value: bool,
    ) -> TokenStream {
        match self {
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                    quote! { #value }
                }
                StdLibType::Pointer(_) => {
                    quote! { #value }
                }
                StdLibType::RefSlice(_reference) => {
                    quote! { #value.as_slice() }
                }
                StdLibType::Str => {
                    quote! { #value.to_str() }
                }
                StdLibType::String => {
                    quote! {
                        unsafe { Box::from_raw(#value).0 }
                    }
                }
                StdLibType::Vec(_) => {
                    quote! {
                        unsafe { Box::from_raw(#value) }
                    }
                }
                StdLibType::Option(_) => {
                    if is_returned_value {
                        quote! {
                            let value = #value;
                            if swift_bridge::option::_get_option_return() {
                                Some(value)
                            } else {
                                None
                            }
                        }
                    } else {
                        todo!("Option<T> function arguments are not yet supported.")
                    }
                }
            },
            _ => todo!(),
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
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                        PointerKind::Const => {
                            format!("UnsafeRawPointer({}!)", value)
                        }
                        PointerKind::Mut => value.to_string(),
                    },
                },
                StdLibType::RefSlice(ty) => {
                    format!(
                           "let slice = {value}; return UnsafeBufferPointer(start: slice.start.assumingMemoryBound(to: {ty}.self), count: Int(slice.len));",
                           value = value,
                           ty = ty.ty.to_swift_type(false)
                       )
                }
                StdLibType::Str => value.to_string(),
                StdLibType::String => {
                    format!("RustString(ptr: {}, isOwned: true)", value)
                }
                StdLibType::Vec(_ty) => {
                    format!("RustVec(ptr: {}, isOwned: true)", value)
                }
                StdLibType::Option(_) => {
                    format!("let val = {val}; if _get_option_return() {{ return val; }} else {{ return nil; }}", val = value)
                }
            },
            _ => todo!(),
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
        host_lang: HostLang,
    ) -> String {
        match self {
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                    Pointee::Void(_ty) => {
                        if ptr.kind == PointerKind::Const && host_lang.is_rust() {
                            format!("UnsafeMutableRawPointer(mutating: {})", value)
                        } else {
                            value.to_string()
                        }
                    }
                },
                StdLibType::Str => value.to_string(),
                StdLibType::String => {
                    format!(
                        "{{{value}.isOwned = false; return {value}.ptr}}()",
                        value = value
                    )
                }
                StdLibType::Vec(_) => {
                    format!("{}.ptr", value)
                }
                StdLibType::Option(_) => {
                    format!("if case let val? = {value} {{ return markReturnTypeSome(val); }} else {{ return markReturnTypeNone(); }}", value = value)
                }
            },
            _ => todo!(),
        }
    }

    pub fn c_include(&self) -> Option<&'static str> {
        match self {
            BuiltInType::StdLib(stdlib_type) => match stdlib_type {
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
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

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
                    ty: Box::new(BuiltInType::StdLib(StdLibType::U32)),
                }),
            ),
            (
                quote! { Option<u32>},
                StdLibType::Option(BuiltInOption {
                    ty: Box::new(BuiltInType::StdLib(StdLibType::U32)),
                }),
            ),
            (
                quote! {*const u8},
                StdLibType::Pointer(BuiltInPointer {
                    kind: PointerKind::Const,
                    pointee: Pointee::BuiltIn(Box::new(BuiltInType::StdLib(StdLibType::U8))),
                }),
            ),
            (
                quote! {*mut f64},
                StdLibType::Pointer(BuiltInPointer {
                    kind: PointerKind::Mut,
                    pointee: Pointee::BuiltIn(Box::new(BuiltInType::StdLib(StdLibType::F64))),
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
                BuiltInType::new_with_type(&ty, &TypeDeclarations::default())
                    .unwrap()
                    .unwrap_stdlib(),
                &expected,
                "{}",
                tokens.to_string()
            )
        }
    }
}
