use crate::bridged_type::{SharedStruct};
use proc_macro2::{TokenStream};
use quote::quote;


/// Generate the functions that Swift calls uses inside of the corresponding class for a
/// transparent struct's Vectorizable implementation.
///
/// So inside of `extension SomeTransparentStruct: Vectorizable {}` on the Swift side.
pub(in super::super) fn generate_vec_of_transparent_struct_functions(
    shared_struct: &SharedStruct
) -> TokenStream {
    if can_generate_vec_of_transparent_struct_functions(&shared_struct) {
        let struct_name = &shared_struct.name;
    
        // examples:
        // "__swift_bridge__$Vec_SomeTransparentStruct$new"
        // "__swift_bridge__$Vec_SomeTransparentStruct$drop"
        let make_export_name = |fn_name| {
            format!(
                "__swift_bridge__$Vec_{}${}",
                shared_struct.swift_name_string(),
                fn_name
            )
        };
        let export_name_new = make_export_name("new");
        let export_name_drop = make_export_name("drop");
        let export_name_len = make_export_name("len");
        let export_name_get = make_export_name("get");
        let export_name_get_mut = make_export_name("get_mut");
        let export_name_push = make_export_name("push");
        let export_name_pop = make_export_name("pop");
        let export_name_as_ptr = make_export_name("as_ptr");
    
        let ffi_struct_repr = &shared_struct.ffi_name_tokens();
        let ffi_option_struct_repr = shared_struct.ffi_option_name_tokens();
        // TODO: Check for trait implementation instead of derives
        let derives: Vec<String> = shared_struct.derives.as_ref().unwrap().iter().map(|derive| derive.to_string()).collect();
        let vec_map = if derives.contains(&"Copy".to_string()) {
            quote! { *v }
        } else {
            quote! { v.clone() }
        };
    
        quote! {
            const _: () = {
                #[doc(hidden)]
                #[export_name = #export_name_new]
                pub extern "C" fn _new() -> *mut Vec<#struct_name> {
                    Box::into_raw(Box::new(Vec::new()))
                }
    
                #[doc(hidden)]
                #[export_name = #export_name_drop]
                pub extern "C" fn _drop(vec: *mut Vec<#struct_name>) {
                    let vec = unsafe { Box::from_raw(vec) };
                    drop(vec)
                }
    
                #[doc(hidden)]
                #[export_name = #export_name_len]
                pub extern "C" fn _len(vec: *const Vec<#struct_name>) -> usize {
                    unsafe { &*vec }.len()
                }
    
                #[doc(hidden)]
                #[export_name = #export_name_get]
                pub extern "C" fn _get(vec: *const Vec<#struct_name>, index: usize) -> #ffi_option_struct_repr {
                    let vec = unsafe { &*vec };
                    let val = vec.get(index).map(|v|#vec_map);
                    #ffi_option_struct_repr::from_rust_repr(val)
                }
    
                #[doc(hidden)]
                #[export_name = #export_name_get_mut]
                pub extern "C" fn _get_mut(vec: *mut Vec<#struct_name>, index: usize) -> #ffi_option_struct_repr {
                    let vec = unsafe { &mut *vec };
                    let val = vec.get_mut(index).map(|v|#vec_map);
                    #ffi_option_struct_repr::from_rust_repr(val)
                }
    
                #[doc(hidden)]
                #[export_name = #export_name_push]
                pub extern "C" fn _push(vec: *mut Vec<#struct_name>, val: #ffi_struct_repr) {
                    unsafe { &mut *vec }.push( val.into_rust_repr() )
                }
    
                #[doc(hidden)]
                #[export_name = #export_name_pop]
                pub extern "C" fn _pop(vec: *mut Vec<#struct_name>) -> #ffi_option_struct_repr {
                    let vec = unsafe { &mut *vec };
                    let val = vec.pop();
                    #ffi_option_struct_repr::from_rust_repr(val)
                }
    
                #[doc(hidden)]
                #[export_name = #export_name_as_ptr]
                pub extern "C" fn _as_ptr(vec: *const Vec<#struct_name>) -> *const #struct_name {
                    unsafe { & *vec }.as_ptr()
                }
            };
        }
    } else {
        quote! {}
    }
}

pub(crate) fn can_generate_vec_of_transparent_struct_functions(shared_struct: &SharedStruct) -> bool {
    // TODO: Check for trait implementation instead of derives
    if let Some(derives) = &shared_struct.derives {
        let derives: Vec<String> = derives.iter().map(|derive| derive.to_string()).collect();
        derives.contains(&"Copy".to_string()) || derives.contains(&"Clone".to_string())
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_tokens_eq;
    use proc_macro2::{Ident, Span};
    use crate::bridged_type::{StructSwiftRepr, StructFields};

    /// Verify that we can generate the functions for an opaque Rust type that get exposed to Swift
    /// in order to power the `extension MyRustType: Vectorizable { }` implementation on the Swift
    /// side.
    #[test]
    fn generates_vectorizable_impl_for_shared_struct_with_copy() {
        let expected = quote! {
            const _: () = {
                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$new"]
                pub extern "C" fn _new() -> *mut Vec<SomeStruct> {
                    Box::into_raw(Box::new(Vec::new()))
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$drop"]
                pub extern "C" fn _drop(vec: *mut Vec<SomeStruct>) {
                    let vec = unsafe { Box::from_raw(vec) };
                    drop(vec)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$len"]
                pub extern "C" fn _len(vec: *const Vec<SomeStruct>) -> usize {
                    unsafe { &*vec }.len()
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$get"]
                pub extern "C" fn _get(vec: *const Vec<SomeStruct>, index: usize) -> __swift_bridge__Option_SomeStruct {
                    let vec = unsafe { &*vec };
                    let val = vec.get(index).map(|v| *v );
                    __swift_bridge__Option_SomeStruct::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$get_mut"]
                pub extern "C" fn _get_mut(vec: *mut Vec<SomeStruct>, index: usize) -> __swift_bridge__Option_SomeStruct {
                    let vec = unsafe { &mut *vec };
                    let val = vec.get_mut(index).map(|v| *v );
                    __swift_bridge__Option_SomeStruct::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$push"]
                pub extern "C" fn _push(vec: *mut Vec<SomeStruct>, val: __swift_bridge__SomeStruct) {
                    unsafe { &mut *vec }.push(val.into_rust_repr())
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$pop"]
                pub extern "C" fn _pop(vec: *mut Vec<SomeStruct>) -> __swift_bridge__Option_SomeStruct {
                    let vec = unsafe { &mut *vec };
                    let val = vec.pop();
                    __swift_bridge__Option_SomeStruct::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$as_ptr"]
                pub extern "C" fn _as_ptr(vec: *const Vec<SomeStruct>) -> *const SomeStruct {
                    unsafe { & *vec }.as_ptr()
                }
            };
        };

        let shared_struct = SharedStruct {
            name: Ident::new("SomeStruct", Span::call_site()),
            swift_repr: StructSwiftRepr::Structure,
            fields: StructFields::Named(vec![]),
            swift_name: None,
            already_declared: false,
            derives: Some(vec![quote! {Copy}, quote! {Clone}]),
        };
        assert_tokens_eq(
            &generate_vec_of_transparent_struct_functions(&shared_struct),
            &expected,
        );
    }

    /// Verify that we can generate the functions for an opaque Rust type that get exposed to Swift
    /// in order to power the `extension MyRustType: Vectorizable { }` implementation on the Swift
    /// side.
    #[test]
    fn generates_vectorizable_impl_for_shared_struct_with_clone() {
        let expected = quote! {
            const _: () = {
                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$new"]
                pub extern "C" fn _new() -> *mut Vec<SomeStruct> {
                    Box::into_raw(Box::new(Vec::new()))
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$drop"]
                pub extern "C" fn _drop(vec: *mut Vec<SomeStruct>) {
                    let vec = unsafe { Box::from_raw(vec) };
                    drop(vec)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$len"]
                pub extern "C" fn _len(vec: *const Vec<SomeStruct>) -> usize {
                    unsafe { &*vec }.len()
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$get"]
                pub extern "C" fn _get(vec: *const Vec<SomeStruct>, index: usize) -> __swift_bridge__Option_SomeStruct {
                    let vec = unsafe { &*vec };
                    let val = vec.get(index).map(|v| v.clone() );
                    __swift_bridge__Option_SomeStruct::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$get_mut"]
                pub extern "C" fn _get_mut(vec: *mut Vec<SomeStruct>, index: usize) -> __swift_bridge__Option_SomeStruct {
                    let vec = unsafe { &mut *vec };
                    let val = vec.get_mut(index).map(|v| v.clone() );
                    __swift_bridge__Option_SomeStruct::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$push"]
                pub extern "C" fn _push(vec: *mut Vec<SomeStruct>, val: __swift_bridge__SomeStruct) {
                    unsafe { &mut *vec }.push(val.into_rust_repr())
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$pop"]
                pub extern "C" fn _pop(vec: *mut Vec<SomeStruct>) -> __swift_bridge__Option_SomeStruct {
                    let vec = unsafe { &mut *vec };
                    let val = vec.pop();
                    __swift_bridge__Option_SomeStruct::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_SomeStruct$as_ptr"]
                pub extern "C" fn _as_ptr(vec: *const Vec<SomeStruct>) -> *const SomeStruct {
                    unsafe { & *vec }.as_ptr()
                }
            };
        };

        let shared_struct = SharedStruct {
            name: Ident::new("SomeStruct", Span::call_site()),
            swift_repr: StructSwiftRepr::Structure,
            fields: StructFields::Named(vec![]),
            swift_name: None,
            already_declared: false,
            derives: Some(vec![quote! {Clone}]),
        };
        assert_tokens_eq(
            &generate_vec_of_transparent_struct_functions(&shared_struct),
            &expected,
        );
    }
}
