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
        todo!("convert rust value to ffi value");
    }
    pub(super) fn convert_ffi_value_to_rust_value(&self, value: &TokenStream) -> TokenStream {
        todo!("convert ffi value to rust value");
    }
    pub(super) fn convert_ffi_expression_to_swift(&self, expression: &str) -> String {
        todo!("result convert ffi value to swift");
    }
    pub fn convert_swift_expression_to_ffi_compatible(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String {
        todo!("convert swift expression to ffi compatible");
    }
    pub fn to_c(&self) -> String {
        match self.ty_ok.deref() {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null => {
                    todo!("Result<()> is not yet supported")
                }
                StdLibType::U8 => "struct __private__ResultU8".to_string(),
                StdLibType::I8 => "struct __private__ResultI8".to_string(),
                StdLibType::U16 => "struct __private__ResultU16".to_string(),
                StdLibType::I16 => "struct __private__ResultI16".to_string(),
                StdLibType::U32 => "struct __private__ResultU32".to_string(),
                StdLibType::I32 => "struct __private__ResultI32".to_string(),
                StdLibType::U64 => "struct __private__ResultU64".to_string(),
                StdLibType::I64 => "struct __private__ResultI64".to_string(),
                StdLibType::Usize => "struct __private__ResultUsize".to_string(),
                StdLibType::Isize => "struct __private__ResultIsize".to_string(),
                StdLibType::F32 => "struct __private__ResultF32".to_string(),
                StdLibType::F64 => "struct __private__ResultF64".to_string(),
                StdLibType::Bool => "struct __private__ResultBool".to_string(),
                StdLibType::Pointer(_) => {
                    todo!("Result<*const T> and Result<*mut T> are not yet supported")
                }
                StdLibType::RefSlice(_) => {
                    todo!("Result<&[T]> is not yet supported")
                }
                StdLibType::Str => "struct RustStr".to_string(),
                StdLibType::String => "void*".to_string(),
                StdLibType::Vec(_) => {
                    todo!("Result<Vec<T>> is not yet supported")
                }
                StdLibType::Option(_) => {
                    todo!("Result<Option<T>> is not yet supported")
                }
                StdLibType::Result(_) => {
                    todo!("Result<Result<T,U>,K> is not yet supported")
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                format!("struct {}", shared_struct.ffi_option_name_string())
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                format!("struct {}", shared_enum.ffi_option_name_string())
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(_opaque)) => "void*".to_string(),
        }
    }
}
