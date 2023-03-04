use crate::bridged_type::BridgedType;
use crate::bridged_type::{BridgeableType, TypePosition, UnusedOptionNoneValue};
use crate::{TypeDeclarations, SWIFT_BRIDGE_PREFIX};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::fmt::{Debug, Formatter};
use syn::{Path, Type, TypeParam};

#[derive(Clone)]
pub(crate) struct CustomResultType {
    pub ty: Ident,
    pub ok_ty: TypeParam,
    pub err_ty: TypeParam,
}

impl CustomResultType {
    pub fn c_enum_name_string(&self) -> String {
        self.to_c_type()
    }
    pub fn c_tag_name_string(&self) -> String {
        format!("{}$Tag", self.to_c_type())
    }
    pub fn c_fields_name_string(&self) -> String {
        format!("{}$Fields", self.to_c_type())
    }
    pub fn c_ok_tag_name(&self) -> String {
        format!("{}$ResultOk", SWIFT_BRIDGE_PREFIX)
    }
    pub fn c_err_tag_name(&self) -> String {
        format!("{}$ResultErr", SWIFT_BRIDGE_PREFIX)
    }
    pub fn ffi_name_tokens(&self) -> TokenStream {
        let tokens = format_ident!(
            "Result{}And{}",
            self.ok_ty.to_token_stream().to_string(),
            self.err_ty.to_token_stream().to_string()
        );
        quote!(#tokens)
    }
}

impl BridgeableType for CustomResultType {
    fn is_built_in_type(&self) -> bool {
        todo!();
    }

    fn is_result(&self) -> bool {
        true
    }

    fn as_result(&self) -> Option<&super::bridgeable_result::BuiltInResult> {
        todo!();
    }

    fn is_passed_via_pointer(&self) -> bool {
        todo!();
    }

    fn generate_ffi_definition(&self, _swift_bridge_path: &Path, _types: &TypeDeclarations) -> Option<TokenStream> {
        None
    }

    fn to_rust_type_path(&self, _types: &TypeDeclarations) -> TokenStream {
        todo!();
    }

    fn to_swift_type(&self, type_pos: TypePosition, _types: &TypeDeclarations) -> String {
        match type_pos {
            TypePosition::FnArg(_, _) => todo!(),
            TypePosition::FnReturn(_) => self.ok_ty.to_token_stream().to_string(),
            TypePosition::SharedStructField => todo!(),
            TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => todo!(),
        }
    }

    fn to_c_type(&self) -> String {
        let ok_name = self.ok_ty.to_token_stream().to_string();
        let err_name = self.err_ty.to_token_stream().to_string();
        format!(
            "{}${}{}And{}",
            SWIFT_BRIDGE_PREFIX,
            self.ty.to_string(),
            ok_name,
            err_name
        )
    }

    fn to_c_include(&self) -> Option<&'static str> {
        None
    }

    fn generate_c_declaration(&self) -> Option<String> {
        todo!()
    }

    fn to_ffi_compatible_rust_type(
        &self,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> TokenStream {
        let ty = format_ident!(
            "Result{}And{}",
            self.ok_ty.to_token_stream().to_string(),
            self.err_ty.to_token_stream().to_string()
        );
        quote! {#ty}
    }

    fn to_ffi_compatible_option_rust_type(
        &self,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> TokenStream {
        todo!();
    }

    fn to_ffi_compatible_option_swift_type(
        &self,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> String {
        todo!()
    }

    fn to_ffi_compatible_option_c_type(&self) -> String {
        todo!();
    }

    fn convert_rust_expression_to_ffi_type(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
        span: Span,
    ) -> TokenStream {
        let ffi_enum_name = self.ffi_name_tokens();
        let ok_ty =
            BridgedType::new_with_str(&self.ok_ty.to_token_stream().to_string(), types).unwrap();
        let err_ty =
            BridgedType::new_with_str(&self.err_ty.to_token_stream().to_string(), types).unwrap();
        let ok_ffi =
            ok_ty.convert_rust_expression_to_ffi_type(&quote!(ok), swift_bridge_path, types, span);
        let err_ffi = err_ty.convert_rust_expression_to_ffi_type(
            &quote!(err),
            swift_bridge_path,
            types,
            span,
        );
        quote! {
            match #expression {
                Ok(ok) => #ffi_enum_name::Ok(#ok_ffi),
                Err(err) => #ffi_enum_name::Err(#err_ffi),
            }
        }
    }

    fn convert_option_rust_expression_to_ffi_type(
        &self,
        _expression: &TokenStream,
        _swift_bridge_path: &Path,
    ) -> TokenStream {
        todo!();
    }

    fn convert_swift_expression_to_ffi_type(
        &self,
        _expression: &str,
        _type_pos: TypePosition,
    ) -> String {
        todo!();
    }

    fn convert_option_swift_expression_to_ffi_type(
        &self,
        _expression: &str,
        _type_pos: TypePosition,
    ) -> String {
        todo!();
    }

    fn convert_ffi_expression_to_rust_type(
        &self,
        _expression: &TokenStream,
        _span: Span,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> TokenStream {
        todo!();
    }

    fn convert_ffi_option_expression_to_rust_type(&self, _expression: &TokenStream) -> TokenStream {
        todo!();
    }

    fn convert_ffi_expression_to_swift_type(
        &self,
        expression: &str,
        type_pos: TypePosition,
        types: &TypeDeclarations,
    ) -> String {
        let c_ok_name = self.c_ok_tag_name();
        let c_err_name = self.c_err_tag_name();
        let ok_ty =
            BridgedType::new_with_str(&self.ok_ty.to_token_stream().to_string(), types).unwrap();
        let err_ty =
            BridgedType::new_with_str(&self.err_ty.to_token_stream().to_string(), types).unwrap();
        let ok_swift_type =
            ok_ty.convert_ffi_expression_to_swift_type("val.payload.ok", type_pos, types);
        let err_swift_type =
            err_ty.convert_ffi_expression_to_swift_type("val.payload.err", type_pos, types);

        match type_pos {
            TypePosition::FnArg(_, _) => todo!(),
            TypePosition::FnReturn(_) => format!(
                "try {{
        let val = {expression};
        switch val.tag {{
        case {c_ok_name}:
            return {ok_swift_type}
        case {c_err_name}:
            throw {err_swift_type}
        default:
            fatalError()
    }} }}()",
                expression = expression,
                c_ok_name = c_ok_name,
                c_err_name = c_err_name,
                ok_swift_type = ok_swift_type,
                err_swift_type = err_swift_type
            ),
            TypePosition::SharedStructField => todo!(),
            TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => todo!(),
        }
    }

    fn convert_ffi_option_expression_to_swift_type(&self, _expression: &str) -> String {
        todo!();
    }

    fn convert_ffi_result_ok_value_to_rust_value(
        &self,
        _result: &TokenStream,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> TokenStream {
        todo!();
    }

    fn convert_ffi_result_err_value_to_rust_value(
        &self,
        _result: &TokenStream,
        _swift_bridge_path: &Path,
        _types: &TypeDeclarations,
    ) -> TokenStream {
        todo!();
    }

    fn unused_option_none_val(&self, _swift_bridge_path: &Path) -> UnusedOptionNoneValue {
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
        todo!();
    }

    fn parse_token_stream_str(_tokens: &str, _types: &TypeDeclarations) -> Option<Self>
    where
        Self: Sized,
    {
        todo!();
    }

    fn is_null(&self) -> bool {
        todo!();
    }

    fn is_str(&self) -> bool {
        todo!();
    }

    fn contains_owned_string_recursive(&self) -> bool {
        todo!();
    }

    fn contains_ref_string_recursive(&self) -> bool {
        todo!();
    }

    fn has_swift_bridge_copy_annotation(&self) -> bool {
        todo!();
    }

    fn only_encoding(&self) -> Option<super::OnlyEncoding> {
        None
    }
}

impl Debug for CustomResultType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpaqueForeignType")
            .field("ty", &self.ty.to_token_stream())
            .field("ok_ty", &self.ok_ty.to_token_stream())
            .field("err_ty", &self.err_ty.to_token_stream())
            .finish()
    }
}
