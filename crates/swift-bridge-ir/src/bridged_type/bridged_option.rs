use crate::bridged_type::{BridgedType, StdLibType};
use crate::parse::HostLang;
use std::ops::Deref;

/// Option<T>
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BridgedOption {
    pub ty: Box<BridgedType>,
}

impl BridgedOption {
    pub(super) fn convert_ffi_value_to_swift_value(&self, func_host_lang: HostLang) -> String {
        let inner_val_var_name = match self.ty.deref() {
            BridgedType::StdLib(std_lib_type) => match std_lib_type {
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
                | StdLibType::Bool
                | StdLibType::Pointer(_) => "val",
                StdLibType::RefSlice(_) => {
                    todo!()
                }
                StdLibType::Str => {
                    todo!()
                }
                StdLibType::String => "val!",
                StdLibType::Vec(_) => {
                    todo!()
                }
                StdLibType::Option(_) => {
                    todo!()
                }
            },
            BridgedType::Foreign(_) => {
                todo!("Support Option<ForeignType>")
            }
        };

        self.ty
            .convert_ffi_value_to_swift_value(func_host_lang, inner_val_var_name)
    }
}
