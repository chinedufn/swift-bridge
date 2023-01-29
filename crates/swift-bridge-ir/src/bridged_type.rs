use std::fmt::Debug;
use std::ops::Deref;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::{quote, quote_spanned};
use syn::{FnArg, Pat, PatType, Path, ReturnType, Type};

pub(crate) use self::bridged_opaque_type::OpaqueForeignType;
use crate::bridged_type::boxed_fn::BridgeableBoxedFnOnce;
use crate::bridged_type::bridgeable_pointer::{BuiltInPointer, Pointee, PointerKind};
use crate::bridged_type::bridgeable_result::BuiltInResult;
use crate::bridged_type::bridgeable_string::BridgedString;
use crate::parse::{HostLang, TypeDeclaration, TypeDeclarations};
use crate::SWIFT_BRIDGE_PREFIX;

use self::bridged_option::BridgedOption;
pub(crate) use self::shared_enum::{EnumVariant, SharedEnum};
pub(crate) use self::shared_struct::{SharedStruct, StructFields, StructSwiftRepr};

pub(crate) mod boxed_fn;
mod bridgeable_pointer;
mod bridgeable_primitive;
mod bridgeable_result;
pub mod bridgeable_str;
pub mod bridgeable_string;
pub mod bridged_opaque_type;
mod bridged_option;
mod shared_enum;
pub(crate) mod shared_struct;

/// Represents a type that can be passed between Rust and Swift.
// TODO: Move away from `BridgedType` and instead use `Box<dyn BridgeableType>`.
//  Our patterns have more or less stabilized and every type ends up implementing the same methods
//  just in different ways.
//  So, instead of a big enum with lots of methods that each match on the `BridgedType`, we can
//  implement the BridgeableType trait for each type that we support.
//  For opaque types we would just `impl BridgeableType for OpaqueForeignType`.
//  We're essentially just moving a bunch of code around such that code for the same type lives
//  next to each other instead of being spread out all over a bunch of match statements.
//
// TODO: Debug bounds is only used for a couple of tests. Remove these bounds and refactor those
//  tests.
pub(crate) trait BridgeableType: Debug {
    /// Whether or not this is a built-in supported type such as `u8`.
    fn is_built_in_type(&self) -> bool;

    /// Whether or not this is a custom type such as `type SomeRustType`.
    fn is_custom_type(&self) -> bool {
        !self.is_built_in_type()
    }

    /// Whether or not this is a `Result<T,E>`.
    fn is_result(&self) -> bool;

    /// Get the Rust representation of this type.
    /// For a string this might be `std::string::String`.
    fn to_rust_type_path(&self) -> TokenStream;

    /// Get the Swift representation of this type.
    ///
    /// For example, `Result<String, ()>` would become `RustResult<String, ()>`
    fn to_swift_type(&self, type_pos: TypePosition, types: &TypeDeclarations) -> String;

    /// Get the C representation of this type.
    fn to_c_type(&self) -> String;

    /// Generate a C include statement to put in the C header.
    /// For example, for a `u8` we would generate a `#include <stdint.h>` line.
    fn to_c_include(&self) -> Option<&'static str>;

    /// Get the FFI compatible Rust type.
    ///
    /// For `String` this would be `*mut std::string::String`.
    /// For `u8` this would be `u8`.
    fn to_ffi_compatible_rust_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream;

    /// Get the FFI compatible Option<Self> representation.
    fn to_ffi_compatible_option_rust_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream;

    /// Get the FFI compatible Option<Self> representation.
    fn to_ffi_compatible_option_swift_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> String;

    /// Get the FFI compatible Option<Self> representation.
    fn to_ffi_compatible_option_c_type(&self) -> String;

    /// Convert a Rust expression to an FFI compatible type.
    fn convert_rust_expression_to_ffi_type(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream;

    /// Convert a an `Option<Self>` Rust expression to an FFI compatible type.
    fn convert_option_rust_expression_to_ffi_type(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
    ) -> TokenStream;

    /// Convert a Swift expression into an FFI compatible type.
    fn convert_swift_expression_to_ffi_type(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String;

    /// Convert a an `Option<Self>` Rust expression to an FFI compatible type.
    fn convert_option_swift_expression_to_ffi_type(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String;

    /// Convert an FFI expression to this type's Rust representation.
    ///
    /// # Examples
    /// RustStr -> &str
    /// *mut RustString -> String
    /// FfiSlice<u8> -> &[u8]
    fn convert_ffi_expression_to_rust_type(
        &self,
        expression: &TokenStream,
        span: Span,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream;

    /// Convert an Option<Self> FFI representation to the Rust representation.
    fn convert_ffi_option_expression_to_rust_type(&self, expression: &TokenStream) -> TokenStream;

    /// Convert an FFI expression to this type's Swift representation.
    fn convert_ffi_expression_to_swift_type(
        &self,
        expression: &str,
        type_pos: TypePosition,
        types: &TypeDeclarations,
    ) -> String;

    /// Convert an Option<Self> FFI representation to the Rust representation.
    fn convert_ffi_option_expression_to_swift_type(&self, expression: &str) -> String;

    /// Convert an FFI Result::Ok value to Rust value.
    ///
    /// For example, for `Result<String, ()>` this would convert
    /// `swift_bridge::result::ResultPtrAndPtr.ok_or_err` into a `String`.
    fn convert_ffi_result_ok_value_to_rust_value(
        &self,
        ok_ffi_value: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream;

    /// Convert an FFI Result::Err value to Rust value.
    ///
    /// For example, for `Result<(), String>` this would convert
    /// `swift_bridge::result::ResultPtrAndPtr.ok_or_err` into a `String`.
    fn convert_ffi_result_err_value_to_rust_value(
        &self,
        err_ffi_value: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream;

    /// The value used to represent `Option<Self>::None` over FFI.
    fn unused_option_none_val(&self, swift_bridge_path: &Path) -> UnusedOptionNoneValue;

    /// Whether or not a string can be parsed by this type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(BridgeableVec::token_stream_str_is_of_type("Vec < u8 >"), true);
    /// assert_eq!(BridgeableVec::token_stream_str_is_of_type("u8"), false);
    /// ```
    fn can_parse_token_stream_str(tokens: &str) -> bool
    where
        Self: Sized;

    /// Convert the `Type` into this BridgeableType.
    fn from_type(ty: &Type, types: &TypeDeclarations) -> Option<Self>
    where
        Self: Sized;

    /// Parse the stringified token stream into this BridgeableType.
    fn parse_token_stream_str(tokens: &str, types: &TypeDeclarations) -> Option<Self>
    where
        Self: Sized;

    /// Whether or not this is the null type `()`.
    /// TODO: This is temporary as we move towards using this trait.. We should look at how
    ///  this is being used and create a trait method(s) that handles that particular case instead
    ///  of checking the type.
    fn is_null(&self) -> bool;

    /// Whether or not this is a `str`.
    /// TODO: This is temporary as we move towards using this trait.. We should look at how
    ///  this is being used and create a trait method(s) that handles that particular case instead
    ///  of checking the type.
    fn is_str(&self) -> bool;

    /// Whether or not the type is a `String`, or a type that contains an owned String such as
    /// `Option<String>` or `struct Foo { field: String }`
    /// TODO: This is temporary as we move towards using this trait.. We should look at how
    ///  this is being used and create a trait method(s) that handles that particular case instead
    ///  of checking the type.
    fn contains_owned_string_recursive(&self) -> bool;

    /// Whether or not the type is a `&str`, or a type that contains a &str such as
    /// `Option<&str>` or `struct Foo { field: &'static str } `
    /// TODO: This is temporary as we move towards using this trait.. We should look at how
    ///  this is being used and create a trait method(s) that handles that particular case instead
    ///  of checking the type.
    fn contains_ref_string_recursive(&self) -> bool;

    /// Parse the type from a `FnArg`.
    fn from_fn_arg(
        fn_arg: &FnArg,
        _associated_type: &Option<TypeDeclaration>,
        types: &TypeDeclarations,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        match fn_arg {
            FnArg::Receiver(_) => {
                //
                todo!()
            }
            FnArg::Typed(ty) => Self::from_type(ty.ty.deref(), types),
        }
    }

    /// Whether or not this type is annotated with `#[swift_bridge(Copy(..))]`
    fn has_swift_bridge_copy_annotation(&self) -> bool;
}

/// Parse a BridgeableType from a stringified token stream.
pub(crate) fn bridgeable_type_from_token_stream_str(
    tokens: &str,
    types: &TypeDeclarations,
) -> Option<Box<dyn BridgeableType>> {
    // TODO: Try all types before falling back to opaque types below

    if BridgedString::can_parse_token_stream_str(tokens) {
        return BridgedString::parse_token_stream_str(tokens, types).map(|o| Box::new(o) as _);
    }

    OpaqueForeignType::parse_token_stream_str(tokens, types).map(|o| Box::new(o) as _)
}

/// Parse a BridgeableType from a stringified token stream.
pub(crate) fn bridgeable_type_from_fn_arg(
    fn_arg: &FnArg,
    types: &TypeDeclarations,
) -> Option<Box<dyn BridgeableType>> {
    match fn_arg {
        FnArg::Receiver(_) => None,
        FnArg::Typed(pat_ty) => {
            BridgedType::new_with_type(&pat_ty.ty, types).map(|o| Box::new(o) as _)
        }
    }
}

// TODO: We're gradually replacing `BridgedType` with `Box<dyn BridgeableType>`.
//  So continue to move more functionality into that trait.
#[derive(Debug)]
pub(crate) enum BridgedType {
    StdLib(StdLibType),
    Foreign(CustomBridgedType),
    // TODO: Move all of the Self::StdLib and Self::Foreign variants into here.. then we can
    //  delete BridgedType entirely and just use `Box<dyn BridgeableType>` everywhere.
    Bridgeable(Box<dyn BridgeableType>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum CustomBridgedType {
    Shared(SharedType),
}

#[derive(Debug)]
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
    Vec(BuiltInVec),
    BoxedFnOnce(BridgeableBoxedFnOnce),
    Option(BridgedOption),
    Result(BuiltInResult),
}

/// TODO: Add this to `OpaqueForeignType`
#[derive(Debug, Copy, Clone)]
pub(crate) enum TypePosition {
    /// A function argument at the given index.
    FnArg(HostLang, usize),
    FnReturn(HostLang),
    SharedStructField,
    SwiftCallsRustAsyncOnCompleteReturnTy,
}

/// &[T]
#[derive(Debug)]
pub(crate) struct BuiltInRefSlice {
    pub ty: Box<BridgedType>,
}

/// Vec<T>
#[derive(Debug)]
pub(crate) struct BuiltInVec {
    pub ty: Box<BridgedType>,
}

impl BridgedType {
    pub fn is_null(&self) -> bool {
        matches!(self, BridgedType::StdLib(StdLibType::Null))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SharedType {
    Struct(SharedStruct),
    Enum(SharedEnum),
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

impl BridgeableType for BridgedType {
    fn is_built_in_type(&self) -> bool {
        !self.is_custom_type()
    }

    fn is_result(&self) -> bool {
        match self {
            BridgedType::StdLib(StdLibType::Result(_)) => true,
            BridgedType::Bridgeable(ty) => ty.is_result(),
            _ => false,
        }
    }

    fn to_rust_type_path(&self) -> TokenStream {
        self.to_rust_type_path()
    }

    fn to_swift_type(&self, type_pos: TypePosition, types: &TypeDeclarations) -> String {
        self.to_swift_type(type_pos, types)
    }

    fn to_c_type(&self) -> String {
        self.to_c()
    }

    fn to_c_include(&self) -> Option<&'static str> {
        todo!()
    }

    fn to_ffi_compatible_rust_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        self.to_ffi_compatible_rust_type(swift_bridge_path, types)
    }

    fn to_ffi_compatible_option_rust_type(
        &self,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> TokenStream {
        todo!()
    }

    fn to_ffi_compatible_option_swift_type(
        &self,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> String {
        todo!()
    }

    fn to_ffi_compatible_option_c_type(&self) -> String {
        todo!()
    }

    fn convert_rust_expression_to_ffi_type(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        self.convert_rust_expression_to_ffi_type(expression, swift_bridge_path, types)
    }

    fn convert_option_rust_expression_to_ffi_type(
        &self,
        _expression: &TokenStream,
        _swift_bridge_path: &Path,
    ) -> TokenStream {
        todo!()
    }

    fn convert_swift_expression_to_ffi_type(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String {
        self.convert_swift_expression_to_ffi_type(expression, type_pos)
    }

    fn convert_option_swift_expression_to_ffi_type(
        &self,
        _expression: &str,
        _type_pos: TypePosition,
    ) -> String {
        todo!()
    }

    fn convert_ffi_expression_to_rust_type(
        &self,
        _expression: &TokenStream,
        _span: Span,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> TokenStream {
        todo!()
    }

    fn convert_ffi_option_expression_to_rust_type(&self, _expression: &TokenStream) -> TokenStream {
        todo!()
    }

    fn convert_ffi_expression_to_swift_type(
        &self,
        expression: &str,
        type_pos: TypePosition,
        types: &TypeDeclarations,
    ) -> String {
        self.convert_ffi_value_to_swift_value(expression, type_pos, types)
    }

    fn convert_ffi_option_expression_to_swift_type(&self, _expression: &str) -> String {
        todo!()
    }

    fn convert_ffi_result_ok_value_to_rust_value(
        &self,
        ok_ffi_value: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        self.convert_ffi_result_ok_value_to_rust_value(ok_ffi_value, swift_bridge_path, types)
    }

    fn convert_ffi_result_err_value_to_rust_value(
        &self,
        err_ffi_value: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        self.convert_ffi_result_err_value_to_rust_value(err_ffi_value, swift_bridge_path, types)
    }

    fn unused_option_none_val(&self, swift_bridge_path: &Path) -> UnusedOptionNoneValue {
        self.unused_option_none_val(swift_bridge_path)
    }

    fn can_parse_token_stream_str(_tokens: &str) -> bool
    where
        Self: Sized,
    {
        true
    }

    fn from_type(_ty: &Type, _types: &TypeDeclarations) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn parse_token_stream_str(_tokens: &str, _types: &TypeDeclarations) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn is_null(&self) -> bool {
        self.is_null()
    }

    fn is_str(&self) -> bool {
        match self {
            BridgedType::StdLib(StdLibType::Str) => true,
            _ => false,
        }
    }

    fn contains_owned_string_recursive(&self) -> bool {
        self.contains_owned_string_recursive()
    }

    fn contains_ref_string_recursive(&self) -> bool {
        todo!()
    }

    fn has_swift_bridge_copy_annotation(&self) -> bool {
        match self {
            BridgedType::Bridgeable(b) => b.has_swift_bridge_copy_annotation(),
            _ => false,
        }
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
            Type::Tuple(tuple) if tuple.elems.len() == 0 => {
                Some(BridgedType::StdLib(StdLibType::Null))
            }
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

    pub fn new_with_str(tokens: &str, types: &TypeDeclarations) -> Option<BridgedType> {
        let tokens = tokens.replace("\n", " ");
        let tokens = tokens.as_str();

        if tokens.starts_with("Vec < ") {
            let inner = tokens.trim_start_matches("Vec < ");
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
        } else if tokens.starts_with("Option < ") {
            let last_bracket = tokens.rfind(">")?;

            let inner = &tokens[0..last_bracket];
            let inner = inner.trim_start_matches("Option < ");

            // Remove spaces from generics. i.e. "SomeType < u32 > " -> "SomeType<u32>"
            let inner = if inner.contains("<") {
                inner.replace(" ", "")
            } else {
                inner.to_string()
            };

            let inner: Type = syn::parse2(TokenStream::from_str(&inner).unwrap()).unwrap();
            let inner = BridgedType::new_with_type(&inner, types)?;

            return Some(BridgedType::StdLib(StdLibType::Option(BridgedOption {
                ty: Box::new(inner),
            })));
        } else if tokens.starts_with("Result < ") {
            return Some(BridgedType::StdLib(StdLibType::Result(
                BuiltInResult::from_str_tokens(&tokens, types)?,
            )));
        } else if tokens.starts_with("Box < dyn FnOnce") {
            return Some(BridgedType::StdLib(StdLibType::BoxedFnOnce(
                BridgeableBoxedFnOnce::from_str_tokens(&tokens, types)?,
            )));
        }

        let ty = match tokens {
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
            "bool" => BridgedType::StdLib(StdLibType::Bool),
            "()" => BridgedType::StdLib(StdLibType::Null),
            _ => {
                if let Some(b) = bridgeable_type_from_token_stream_str(tokens, types) {
                    return Some(BridgedType::Bridgeable(b));
                }

                let bridged_type = types.get(tokens)?;
                let bridged_type = bridged_type.to_bridged_type(false, false);
                bridged_type
            }
        };
        return Some(ty);
    }

    // Convert the BuiltInType to the corresponding Rust type.
    // U8 -> u8
    // Vec<U32> -> Vec<u32>
    // SomeOpaqueRustType -> super::SomeOpaqueRustType
    pub(crate) fn to_rust_type_path(&self) -> TokenStream {
        match self {
            BridgedType::Bridgeable(b) => b.to_rust_type_path(),
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
                                let ty = ty.to_rust_type_path();
                                quote! { #ptr_kind #ty}
                            }
                            Pointee::Void(_ty) => {
                                // quote! { * #ptr_kind #ty };
                                panic!("Add a test case that hits this branch, then make it pass")
                            }
                        }
                    }
                    StdLibType::RefSlice(ref_slice) => {
                        let ty = ref_slice.ty.to_rust_type_path();
                        quote! { &[#ty]}
                    }
                    StdLibType::Str => quote! { &str },
                    StdLibType::Vec(v) => {
                        let ty = v.ty.to_rust_type_path();
                        quote! { Vec<#ty> }
                    }
                    StdLibType::Option(opt) => {
                        let ty = opt.ty.to_rust_type_path();
                        quote! { Option<#ty> }
                    }
                    StdLibType::Result(result) => result.to_rust_type_path(),
                    StdLibType::BoxedFnOnce(fn_once) => fn_once.to_rust_type_path(),
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                let ty_name = &shared_struct.name;
                quote! {
                    #ty_name
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                let enum_name = &shared_enum.name;
                quote! {
                    #enum_name
                }
            }
        }
    }

    // Get the corresponding Rust type for this Built in type
    //
    // U8 -> u8
    // RefSlice(U8) -> FfiSlice
    // Str -> RustStr
    pub fn to_ffi_compatible_rust_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let ty = match self {
            BridgedType::Bridgeable(b) => b.to_ffi_compatible_rust_type(swift_bridge_path, types),
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
                        Pointee::BuiltIn(ty) => {
                            ty.to_ffi_compatible_rust_type(swift_bridge_path, types)
                        }
                        Pointee::Void(ty) => {
                            quote! { super::#ty }
                        }
                    };

                    quote! { #kind #ty}
                }
                StdLibType::RefSlice(slice) => {
                    let ty = slice
                        .ty
                        .to_ffi_compatible_rust_type(swift_bridge_path, types);
                    quote! {#swift_bridge_path::FfiSlice<#ty>}
                }
                StdLibType::Str => {
                    quote! {#swift_bridge_path::string::RustStr}
                }
                StdLibType::Null => {
                    quote! { () }
                }
                StdLibType::Vec(ty) => {
                    let ty = ty.ty.to_rust_type_path();
                    quote! { *mut Vec<#ty> }
                }
                StdLibType::Option(opt) => match opt.ty.deref() {
                    BridgedType::Bridgeable(b) => {
                        b.to_ffi_compatible_option_rust_type(swift_bridge_path, types)
                    }
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
                        StdLibType::Vec(ty) => {
                            let ty = ty.ty.to_rust_type_path();
                            quote! { *mut Vec<#ty> }
                        }
                        StdLibType::Option(_) => {
                            todo!("Option<Option<T>> is not yet supported")
                        }
                        StdLibType::Result(_) => {
                            todo!("Option<Result<T, E>> is not yet supported")
                        }
                        StdLibType::BoxedFnOnce(_) => {
                            todo!("Support Box<dyn FnOnce(A, B) -> C>")
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
                },
                StdLibType::Result(result) => result.to_ffi_compatible_rust_type(swift_bridge_path),
                StdLibType::BoxedFnOnce(fn_once) => fn_once.to_ffi_compatible_rust_type(),
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
                let ty_name = &shared_enum.name;

                if shared_enum.already_declared {
                    quote! { <super:: #ty_name as #swift_bridge_path::SharedEnum>::FfiRepr }
                } else {
                    let ffi_ty_name = shared_enum.ffi_name_tokens();
                    quote! { #ffi_ty_name }
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
            BridgedType::Bridgeable(b) => b.to_swift_type(type_pos, types),
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
                        TypePosition::FnArg(func_host_lang, _)
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
                    TypePosition::FnArg(func_host_lang, _) => {
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
                StdLibType::Vec(ty) => {
                    format!("RustVec<{}>", ty.ty.to_swift_type(type_pos, types))
                }
                StdLibType::Option(opt) => match type_pos {
                    TypePosition::FnArg(func_host_lang, _)
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
                StdLibType::Result(result) => result.to_swift_type(type_pos, types),
                StdLibType::BoxedFnOnce(boxed_fn) => boxed_fn.to_swift_type().to_string(),
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                match type_pos {
                    TypePosition::FnArg(func_host_lang, _)
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
                    TypePosition::FnArg(func_host_lang, _)
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
        }
    }

    pub fn to_c(&self) -> String {
        match self {
            BridgedType::Bridgeable(b) => b.to_c_type(),
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
                StdLibType::Vec(_) => "void*".to_string(),
                StdLibType::Option(opt) => opt.to_c(),
                StdLibType::Result(result) => result.to_c().to_string(),
                StdLibType::BoxedFnOnce(_) => "void*".to_string(),
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                format!("struct {}", shared_struct.ffi_name_string())
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                format!("struct {}", shared_enum.ffi_name_string())
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
                            self.to_rust_type_path()
                        }
                        Pointee::Void(_) => {
                            let pointer_kind = &pointer.kind;
                            let pointee = &pointer.pointee;

                            quote! { #pointer_kind super:: #pointee }
                        }
                    },
                    _ => self.to_rust_type_path(),
                }
            }
            _ => self.to_rust_type_path(),
        }
    }

    // Wrap an expression of this BuiltInType to be suitable to send from Rust to Swift.
    //
    // Examples:
    // If value foo is a String.. `foo` becomes `swiftbridge:string::RustString(foo)`
    // If value bar is a &str.. `bar` becomes `swiftbridge::string::RustStr::from_str(bar)`
    pub fn convert_rust_expression_to_ffi_type(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        match self {
            BridgedType::Bridgeable(b) => {
                b.convert_rust_expression_to_ffi_type(expression, swift_bridge_path, types)
            }
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
                StdLibType::Vec(_) => {
                    quote! { Box::into_raw(Box::new( #expression )) }
                }
                StdLibType::Option(opt) => {
                    opt.convert_rust_expression_to_ffi_type(expression, swift_bridge_path)
                }
                StdLibType::Result(result) => {
                    result.convert_rust_expression_to_ffi_type(expression, swift_bridge_path, types)
                }
                StdLibType::BoxedFnOnce(fn_once) => {
                    fn_once.convert_rust_value_to_ffi_compatible_value(expression)
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
        }
    }

    // Wrap an argument of this BuiltInType to convert it from it's FFI format to it's Rust type.
    //
    // Examples:
    // RustStr -> &str
    // *mut RustString -> String
    // FfiSlice<u8> -> &[u8]
    pub fn convert_ffi_expression_to_rust_type(
        &self,
        value: &TokenStream,
        span: Span,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        match self {
            BridgedType::Bridgeable(b) => {
                b.convert_ffi_expression_to_rust_type(value, span, swift_bridge_path, types)
            }
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
                StdLibType::Vec(_) => {
                    quote_spanned! {span=>
                        unsafe { * Box::from_raw(#value) }
                    }
                }
                StdLibType::Option(bridged_option) => {
                    bridged_option.convert_ffi_expression_to_rust_type(value)
                }
                StdLibType::Result(result) => {
                    result.convert_ffi_value_to_rust_value(value, span, swift_bridge_path, types)
                }
                StdLibType::BoxedFnOnce(_) => {
                    todo!("Support Box<dyn FnOnce(A, B) -> C>")
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
        expression: &str,
        type_pos: TypePosition,
        types: &TypeDeclarations,
    ) -> String {
        match self {
            BridgedType::Bridgeable(b) => {
                b.convert_ffi_expression_to_swift_type(expression, type_pos, types)
            }
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
                | StdLibType::Bool => expression.to_string(),
                StdLibType::Pointer(ptr) => match &ptr.pointee {
                    Pointee::BuiltIn(_) => expression.to_string(),
                    Pointee::Void(_ty) => match ptr.kind {
                        PointerKind::Const => match type_pos {
                            TypePosition::FnArg(func_host_lang, _) => {
                                if func_host_lang.is_rust() {
                                    format!("UnsafeRawPointer({}!)", expression)
                                } else {
                                    expression.to_string()
                                }
                            }
                            TypePosition::FnReturn(_) => {
                                format!("UnsafeRawPointer({}!)", expression)
                            }
                            TypePosition::SharedStructField => {
                                format!("UnsafeRawPointer({}!)", expression)
                            }
                            TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                                unimplemented!()
                            }
                        },
                        PointerKind::Mut => expression.to_string(),
                    },
                },
                StdLibType::RefSlice(ty) => {
                    format!(
                        "let slice = {value}; return UnsafeBufferPointer(start: slice.start.assumingMemoryBound(to: {ty}.self), count: Int(slice.len));",
                        value = expression,
                        ty = ty.ty.to_swift_type(type_pos,types)
                       )
                }
                StdLibType::Str => expression.to_string(),
                StdLibType::Vec(_ty) => {
                    format!("RustVec(ptr: {})", expression)
                }
                StdLibType::Option(opt) => opt.convert_ffi_expression_to_swift_type(expression),
                StdLibType::Result(result) => {
                    result.convert_ffi_value_to_swift_value(expression, type_pos, types)
                }
                StdLibType::BoxedFnOnce(fn_once) => {
                    fn_once.convert_ffi_value_to_swift_value(type_pos)
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                format!("{}.intoSwiftRepr()", expression)
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                format!("{}.intoSwiftRepr()", expression)
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
    pub fn convert_swift_expression_to_ffi_type(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String {
        match self {
            BridgedType::Bridgeable(b) => {
                b.convert_swift_expression_to_ffi_type(expression, type_pos)
            }
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
                | StdLibType::Bool => expression.to_string(),
                StdLibType::RefSlice(_) => {
                    format!("{}.toFfiSlice()", expression)
                }
                StdLibType::Pointer(ptr) => match &ptr.pointee {
                    Pointee::BuiltIn(_) => expression.to_string(),
                    Pointee::Void(_ty) => match type_pos {
                        TypePosition::FnArg(func_host_lang, _)
                        | TypePosition::FnReturn(func_host_lang) => {
                            if ptr.kind == PointerKind::Const && func_host_lang.is_rust() {
                                format!("UnsafeMutableRawPointer(mutating: {})", expression)
                            } else {
                                expression.to_string()
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
                    TypePosition::FnArg(func_host_lang, _)
                    | TypePosition::FnReturn(func_host_lang) => {
                        if func_host_lang.is_rust() {
                            format!("{val}AsRustStr", val = expression)
                        } else {
                            expression.to_string()
                        }
                    }
                    TypePosition::SharedStructField => {
                        todo!("&str in shared struct fields is not yet supported")
                    }
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        unimplemented!()
                    }
                },
                StdLibType::Vec(_) => {
                    format!(
                        "{{ let val = {value}; val.isOwned = false; return val.ptr }}()",
                        value = expression
                    )
                }
                StdLibType::Option(option) => {
                    option.convert_swift_expression_to_ffi_type(expression, type_pos)
                }
                StdLibType::Result(result) => {
                    result.convert_swift_expression_to_ffi_compatible(expression, type_pos)
                }
                StdLibType::BoxedFnOnce(_) => {
                    todo!("Support Box<dyn FnOnce(A, B) -> C>")
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                format!("{}.intoFfiRepr()", expression)
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                format!("{}.intoFfiRepr()", expression)
            }
        }
    }

    fn convert_ffi_result_ok_value_to_rust_value(
        &self,
        ok_ffi_value: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        match self {
            BridgedType::Bridgeable(b) => {
                b.convert_ffi_result_ok_value_to_rust_value(ok_ffi_value, swift_bridge_path, types)
            }
            _ => unimplemented!(),
        }
    }

    fn convert_ffi_result_err_value_to_rust_value(
        &self,
        err_ffi_value: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        match self {
            BridgedType::Bridgeable(b) => b.convert_ffi_result_err_value_to_rust_value(
                err_ffi_value,
                swift_bridge_path,
                types,
            ),
            _ => unimplemented!(),
        }
    }

    pub fn to_c_include(&self) -> Option<&'static str> {
        match self {
            BridgedType::Bridgeable(b) => b.to_c_include(),
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
                    Pointee::BuiltIn(ty) => ty.to_c_include(),
                    Pointee::Void(_) => None,
                },
                StdLibType::RefSlice(slice) => slice.ty.to_c_include(),
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
        }
    }

    /// When we want to return an Option::None we still need to return a dummy value to appease the
    /// type checker, even though it never gets used by the caller.
    fn unused_option_none_val(&self, swift_bridge_path: &Path) -> UnusedOptionNoneValue {
        match self {
            BridgedType::Bridgeable(b) => b.unused_option_none_val(swift_bridge_path),
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
                StdLibType::Vec(_) => {
                    todo!("Support Option<Vec<T>>")
                }
                StdLibType::Option(_) => {
                    todo!("Support nested Option<Option<T>>")
                }
                StdLibType::Result(_) => {
                    todo!("Result<T, E> is not yet supported")
                }
                StdLibType::BoxedFnOnce(_) => {
                    todo!("Support Box<dyn FnOnce(A, B) -> C>")
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
        }
    }

    /// Whether or not the type is a `String`, or a type that contains an owned String such as
    /// `Option<String>` or `struct Foo { field: String } `
    pub fn contains_owned_string_recursive(&self) -> bool {
        match self {
            BridgedType::Bridgeable(b) => b.contains_owned_string_recursive(),
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Vec(inner) => inner.ty.contains_owned_string_recursive(),
                StdLibType::Option(inner) => inner.ty.contains_owned_string_recursive(),
                StdLibType::Result(inner) => {
                    inner.ok_ty.contains_owned_string_recursive()
                        || inner.err_ty.contains_owned_string_recursive()
                }
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
        }
    }

    /// Whether or not the type is a `&str`, or a type that contains a &str such as
    /// `Option<&str>` or `struct Foo { field: &'static str } `
    pub fn contains_ref_string_recursive(&self) -> bool {
        match self {
            BridgedType::Bridgeable(b) => b.contains_ref_string_recursive(),
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
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                let enum_name = &shared_enum.name;

                let maybe_super = if shared_enum.already_declared {
                    quote! { super:: }
                } else {
                    quote! {}
                };

                quote! {
                    { let val: #maybe_super #enum_name = #expression.into(); val }
                }
            }
            // TODO: Instead of this catchall.. explicitly match on all variants and use
            //  a similar approach to how we handle shared structs
            _ => {
                quote! { #expression.into() }
            }
        }
    }

    pub fn is_custom_type(&self) -> bool {
        match self {
            BridgedType::StdLib(_) => false,
            BridgedType::Foreign(_) => true,
            BridgedType::Bridgeable(b) => b.is_custom_type(),
        }
    }
}

pub(crate) struct UnusedOptionNoneValue {
    rust: TokenStream,
    #[allow(unused)]
    swift: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Verify that we treat newline characters as spaces when parsing a type from string.
    /// Not sure what can lead a stringified token stream to have newline characters in it but
    /// we've observed it in the wild so this test guards against mishandling that scenario.
    #[test]
    fn treats_newline_characters_as_spaces() {
        let tokens = "Box < dyn\nFnOnce(Result < String, String\n>) >";

        let parsed = BridgedType::new_with_str(tokens, &TypeDeclarations::default()).unwrap();
        match parsed {
            BridgedType::StdLib(StdLibType::BoxedFnOnce(_)) => {}
            _ => panic!(),
        };
    }
}
