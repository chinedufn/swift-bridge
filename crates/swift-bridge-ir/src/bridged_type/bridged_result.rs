use crate::bridged_type::{BridgedType, CustomBridgedType, SharedType, StdLibType, TypePosition};
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;
use syn::Path;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BridgedResult {
    pub ty_ok: Box<BridgedType>,
    pub ty_err: Box<BridgedType>,
}
impl BridgedResult {
    pub(super) fn convert_rust_value_to_ffi_value(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
    ) -> TokenStream {
        todo!();
    }
    pub(super) fn convert_ffi_value_to_rust_value(&self, value: &TokenStream) -> TokenStream {
        todo!();
    }
    pub(super) fn convert_ffi_expression_to_swift(&self, expression: &str) -> String {
        todo!();
    }
    pub fn convert_swift_expression_to_ffi_compatible(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String {
        todo!();
    }
    pub fn to_c(&self) -> String {
        todo!();
    }
}
