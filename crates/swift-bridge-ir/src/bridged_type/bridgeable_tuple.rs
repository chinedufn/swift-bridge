use crate::bridged_type::shared_struct::UnnamedStructFields;
use crate::bridged_type::BridgeableType;
use crate::bridged_type::OnlyEncoding;
use crate::bridged_type::BuiltInResult;
use crate::bridged_type::TypePosition;
use crate::bridged_type::UnusedOptionNoneValue;
use crate::parse::TypeDeclarations;
use std::fmt::Debug;

use std::ops::Deref;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::{quote, quote_spanned};
use syn::{FnArg, Pat, PatType, Path, ReturnType, Type};

#[derive(Debug)]
pub(crate) struct BuiltInTuple(UnnamedStructFields);

impl BridgeableType for BuiltInTuple {
    fn is_built_in_type(&self) -> bool {
        todo!();
    }

    fn only_encoding(&self) -> Option<OnlyEncoding> {
        todo!();
    }

    fn is_result(&self) -> bool {
        todo!();
    }

    fn as_result(&self) -> Option<&BuiltInResult> {
        todo!();
    }

    fn is_passed_via_pointer(&self) -> bool {
        todo!();
    }

    fn generate_custom_rust_ffi_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> Option<TokenStream> {
        todo!();
    }

    fn generate_custom_c_ffi_type(&self, types: &TypeDeclarations) -> Option<String> {
        todo!();
    }

    fn to_rust_type_path(&self, types: &TypeDeclarations) -> TokenStream {
        todo!();
    }

    fn to_swift_type(&self, type_pos: TypePosition, types: &TypeDeclarations) -> String {
        todo!();
    }

    fn to_c_type(&self, types: &TypeDeclarations) -> String {
        todo!();
    }

    fn to_c_include(&self) -> Option<&'static str> {
        todo!()
    }

    fn to_ffi_compatible_rust_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        todo!();
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
        span: Span,
    ) -> TokenStream {
        todo!();
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
        types: &TypeDeclarations,
        type_pos: TypePosition,
    ) -> String {
        todo!();
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
        todo!();
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
        todo!();
    }

    fn convert_ffi_result_err_value_to_rust_value(
        &self,
        err_ffi_value: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        todo!();
    }

    fn unused_option_none_val(&self, swift_bridge_path: &Path) -> UnusedOptionNoneValue {
        todo!();
    }

    fn can_parse_token_stream_str(_tokens: &str) -> bool
    where
        Self: Sized,
    {
        todo!();
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
        todo!();
    }

    fn is_str(&self) -> bool {
        todo!();
    }

    fn contains_owned_string_recursive(&self, types: &TypeDeclarations) -> bool {
        self.contains_owned_string_recursive(types)
    }

    fn contains_ref_string_recursive(&self) -> bool {
        todo!()
    }

    fn has_swift_bridge_copy_annotation(&self) -> bool {
        todo!();
    }

    fn to_alpha_numeric_underscore_name(&self) -> String {
        todo!();
    }
}
