use crate::bridged_type::{BridgedType, CustomBridgedType, SharedType, StdLibType, TypePosition};
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;
use syn::Path;

/// Option<T>
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BridgedOption {
    pub ty: Box<BridgedType>,
}

impl BridgedOption {
    pub(super) fn convert_rust_value_to_ffi_value(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
    ) -> TokenStream {
        let unused_none_value = self.ty.rust_unused_option_none_val(swift_bridge_path).rust;

        let option_rust_primitive_to_ffi_primitive =
            move |ffi_option_name: TokenStream, unused_none: TokenStream| {
                quote! {
                    if let Some(val) = #expression {
                        #swift_bridge_path::option::#ffi_option_name { val, is_some: true }
                    } else {
                        #swift_bridge_path::option::#ffi_option_name { val: #unused_none, is_some: false }
                    }
                }
            };

        match self.ty.deref() {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null => {
                    todo!("Option<()> is not yet supported")
                }
                StdLibType::U8 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionU8}, quote! {123})
                }
                StdLibType::I8 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionI8}, quote! {123})
                }
                StdLibType::U16 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionU16}, quote! {123})
                }
                StdLibType::I16 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionI16}, quote! {123})
                }
                StdLibType::U32 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionU32}, quote! {123})
                }
                StdLibType::I32 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionI32}, quote! {123})
                }
                StdLibType::U64 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionU64}, quote! {123})
                }
                StdLibType::I64 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionI64}, quote! {123})
                }
                StdLibType::Usize => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionUsize}, quote! {123})
                }
                StdLibType::Isize => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionIsize}, quote! {123})
                }
                StdLibType::F32 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionF32}, quote! {123.4})
                }
                StdLibType::F64 => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionF64}, quote! {123.4})
                }
                StdLibType::Bool => {
                    option_rust_primitive_to_ffi_primitive(quote! {OptionBool}, quote! {false})
                }
                StdLibType::Pointer(_) => {
                    todo!("Support Option<*const T> and Option<*mut T>")
                }
                StdLibType::RefSlice(_) => {
                    todo!("Support Option<&[T]> and Option<&mut [T]>")
                }
                StdLibType::Str => {
                    quote! {
                        if let Some(val) = #expression {
                            #swift_bridge_path::string::RustStr::from_str(val)
                        } else {
                            #swift_bridge_path::string::RustStr { start: std::ptr::null::<u8>(), len: 0}
                        }
                    }
                }
                StdLibType::String => {
                    quote! {
                        if let Some(val) = #expression {
                            #swift_bridge_path::string::RustString(val).box_into_raw()
                        } else {
                            #unused_none_value
                        }
                    }
                }
                StdLibType::Vec(_) => {
                    todo!("Support Option<Vec<T>>")
                }
                StdLibType::Option(_) => {
                    todo!("Support Option<Option<T>>")
                }
                StdLibType::Result(_) => {
                    todo!("Support Option<Result<T,U>>")
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                let option_name = shared_struct.ffi_option_name_tokens();
                quote! {
                    #option_name::from_rust_repr(#expression)
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                let option_name = shared_enum.ffi_option_name_tokens();
                quote! {
                    #option_name::from_rust_repr(#expression)
                }
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(_opaque_type)) => {
                quote! {
                    if let Some(val) = #expression {
                        Box::into_raw(Box::new(val))
                    } else {
                        std::ptr::null_mut()
                    }
                }
            }
        }
    }

    pub(super) fn convert_ffi_value_to_rust_value(&self, value: &TokenStream) -> TokenStream {
        match self.ty.deref() {
            BridgedType::StdLib(stdlib_ty) => match stdlib_ty {
                StdLibType::Null => {
                    todo!("Option<()> is not yet supported")
                }
                StdLibType::U8
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
                    quote! { if #value.is_some { Some(#value.val) } else { None } }
                }
                StdLibType::Pointer(_) => {
                    todo!("Option<*const T> and Option<*mut T> are not yet supported.")
                }
                StdLibType::RefSlice(_) => {
                    todo!("Option<*const T> and Option<*mut T> are not yet supported.")
                }
                StdLibType::Str => {
                    quote! {
                        if #value.start.is_null() { None } else { Some(#value.to_str()) }
                    }
                }
                StdLibType::String => {
                    quote! {
                        if #value.is_null() {
                            None
                        } else {
                            Some(unsafe { Box::from_raw(#value).0 } )
                        }
                    }
                }
                StdLibType::Vec(_) => {
                    todo!("Option<Vec<T>> is not yet supported")
                }
                StdLibType::Option(_) => {
                    todo!("Option<Option<T>> is not yet supported")
                }
                StdLibType::Result(_) => {
                    todo!("Option<Result<T, U>> is not yet supported")
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                quote! {
                    #value.into_rust_repr()
                }
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                quote! {
                    #value.into_rust_repr()
                }
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(_opaque)) => {
                quote! {
                    if #value.is_null() {
                        None
                    } else {
                        Some(unsafe { * Box::from_raw(#value) } )
                    }
                }
            }
        }
    }

    pub(super) fn convert_ffi_expression_to_swift(&self, expression: &str) -> String {
        match self.ty.deref() {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null => {
                    todo!("Option<()> is not yet supported")
                }
                StdLibType::U8
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
                    format!("{{ let val = {expression}; if val.is_some {{ return val.val }} else {{ return nil }} }}()", expression = expression)
                }
                StdLibType::Pointer(_) => {
                    todo!("Support Option<*const T> and Option<*mut T>")
                }
                StdLibType::RefSlice(_) => {
                    todo!("Support Option<&[T]>")
                }
                StdLibType::Str => {
                    format!(
                            "{{ let val = {val}; if val.start != nil {{ return val; }} else {{ return nil; }} }}()",
                            val = expression,
                        )
                }
                StdLibType::String => {
                    format!("{{ let val = {expression}; if val != nil {{ return RustString(ptr: val!) }} else {{ return nil }} }}()", expression = expression,)
                }
                StdLibType::Vec(_) => {
                    todo!("Support Option<Vec<T>>")
                }
                StdLibType::Option(_) => {
                    todo!("Support Option<Option<T>>")
                }
                StdLibType::Result(_) => {
                    todo!("Support Option<Result<T,U>>")
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(_shared_struct))) => {
                format!("{expression}.intoSwiftRepr()", expression = expression)
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(_shared_enum))) => {
                format!("{expression}.intoSwiftRepr()", expression = expression)
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(opaque)) => {
                let type_name = opaque.swift_name();
                format!(
                    "{{ let val = {expression}; if val != nil {{ return {type_name}(ptr: val!) }} else {{ return nil }} }}()",
                        expression = expression,
                        type_name = type_name
                )
            }
        }
    }

    pub fn convert_swift_expression_to_ffi_compatible(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String {
        let convert_primitive = move |primitive_kind: &str, unused_none: &str| {
            format!(
                "{{ let val = {expression}; return __private__Option{primitive_kind}(val: val ?? {unused_none}, is_some: val != nil); }}()",
                primitive_kind = primitive_kind,
                expression = expression,
                unused_none = unused_none
            )
        };

        match self.ty.deref() {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null => {
                    todo!("Option<()> is not yet supported")
                }
                StdLibType::U8 => convert_primitive("U8", "123"),
                StdLibType::I8 => convert_primitive("I8", "123"),
                StdLibType::U16 => convert_primitive("U16", "123"),
                StdLibType::I16 => convert_primitive("I16", "123"),
                StdLibType::U32 => convert_primitive("U32", "123"),
                StdLibType::I32 => convert_primitive("I32", "123"),
                StdLibType::U64 => convert_primitive("U64", "123"),
                StdLibType::I64 => convert_primitive("I64", "123"),
                StdLibType::Usize => convert_primitive("Usize", "123"),
                StdLibType::Isize => convert_primitive("Isize", "123"),
                StdLibType::F32 => convert_primitive("F32", "123.4"),
                StdLibType::F64 => convert_primitive("F64", "123.4"),
                StdLibType::Bool => convert_primitive("Bool", "false"),
                StdLibType::Pointer(_) => {
                    todo!("Option<*const T> and Option<*mut T> are not yet supported")
                }
                StdLibType::RefSlice(_) => {
                    todo!("Option<&[T]> is not yet supported")
                }
                StdLibType::Str => {
                    format!("{expression}AsRustStr", expression = expression)
                }
                StdLibType::String => match type_pos {
                    TypePosition::FnArg(_func_host_lang) => {
                        format!(
                                "{{ if let rustString = optionalStringIntoRustString({expression}) {{ rustString.isOwned = false; return rustString.ptr }} else {{ return nil }} }}()",
                                expression = expression
                            )
                    }
                    TypePosition::FnReturn(_) => {
                        todo!("Need to come back and think through what should happen here...")
                    }
                    TypePosition::SharedStructField => {
                        todo!("Option<String> fields in structs are not yet supported.")
                    }
                    TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                        unimplemented!()
                    }
                },
                StdLibType::Vec(_) => {
                    todo!("Option<Vec<T> is not yet supported")
                }
                StdLibType::Option(_) => {
                    todo!("Option<Option<T> is not yet supported")
                }
                StdLibType::Result(_) => {
                    todo!("Option<Result<T, U>> is not yet supported")
                }
            },
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(shared_struct))) => {
                let ffi_name = shared_struct.ffi_option_name_string();
                format!(
                    "{ffi_name}.fromSwiftRepr({expression})",
                    ffi_name = ffi_name,
                    expression = expression
                )
            }
            BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(shared_enum))) => {
                let ffi_name = shared_enum.ffi_option_name_string();
                format!(
                    "{ffi_name}.fromSwiftRepr({expression})",
                    ffi_name = ffi_name,
                    expression = expression
                )
            }
            BridgedType::Foreign(CustomBridgedType::Opaque(_opaque)) => {
                format!("{{ if let val = {expression} {{ val.isOwned = false; return val.ptr }} else {{ return nil }} }}()", expression = expression,)
            }
        }
    }
}

impl BridgedOption {
    pub fn to_c(&self) -> String {
        match self.ty.deref() {
            BridgedType::StdLib(stdlib_type) => match stdlib_type {
                StdLibType::Null => {
                    todo!("Option<()> is not yet supported")
                }
                StdLibType::U8 => "struct __private__OptionU8".to_string(),
                StdLibType::I8 => "struct __private__OptionI8".to_string(),
                StdLibType::U16 => "struct __private__OptionU16".to_string(),
                StdLibType::I16 => "struct __private__OptionI16".to_string(),
                StdLibType::U32 => "struct __private__OptionU32".to_string(),
                StdLibType::I32 => "struct __private__OptionI32".to_string(),
                StdLibType::U64 => "struct __private__OptionU64".to_string(),
                StdLibType::I64 => "struct __private__OptionI64".to_string(),
                StdLibType::Usize => "struct __private__OptionUsize".to_string(),
                StdLibType::Isize => "struct __private__OptionIsize".to_string(),
                StdLibType::F32 => "struct __private__OptionF32".to_string(),
                StdLibType::F64 => "struct __private__OptionF64".to_string(),
                StdLibType::Bool => "struct __private__OptionBool".to_string(),
                StdLibType::Pointer(_) => {
                    todo!("Option<*const T> and Option<*mut T> are not yet supported")
                }
                StdLibType::RefSlice(_) => {
                    todo!("Option<&[T]> is not yet supported")
                }
                StdLibType::Str => "struct RustStr".to_string(),
                StdLibType::String => "void*".to_string(),
                StdLibType::Vec(_) => {
                    todo!("Option<Vec<T>> is not yet supported")
                }
                StdLibType::Option(_) => {
                    todo!("Option<Option<T>> is not yet supported")
                }
                StdLibType::Result(_) => {
                    todo!("Option<Result<T,U>> is not yet supported")
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
