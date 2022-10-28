use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Generate the functions that Swift calls uses inside of the corresponding class for a
/// transparent enum's Vectorizable implementation.
///
/// So inside of `extension SomeTransparentEnum: Vectorizable {}` on the Swift side.
pub(in super::super) fn generate_vec_of_transparent_enum_functions(
    enum_name: &Ident,
) -> TokenStream {
    // examples:
    // "__swift_bridge__$Vec_SomeTransparentEnum$new"
    // "__swift_bridge__$Vec_SomeTransparentEnum$drop"
    let make_export_name = |fn_name| format!("__swift_bridge__$Vec_{}${}", enum_name, fn_name);
    let export_name_new = make_export_name("new");
    let export_name_drop = make_export_name("drop");
    let export_name_len = make_export_name("len");
    let export_name_get = make_export_name("get");
    let export_name_get_mut = make_export_name("get_mut");
    let export_name_push = make_export_name("push");
    let export_name_pop = make_export_name("pop");
    let export_name_as_ptr = make_export_name("as_ptr");

    let ffi_enum_repr = Ident::new(&format!("__swift_bridge__{}", enum_name), enum_name.span());
    let ffi_option_enum_repr = Ident::new(
        &format!("__swift_bridge__Option_{}", enum_name),
        enum_name.span(),
    );

    quote! {
        const _: () = {
            #[doc(hidden)]
            #[export_name = #export_name_new]
            pub extern "C" fn _new() -> *mut Vec<#enum_name> {
                Box::into_raw(Box::new(Vec::new()))
            }

            #[doc(hidden)]
            #[export_name = #export_name_drop]
            pub extern "C" fn _drop(vec: *mut Vec<#enum_name>) {
                let vec = unsafe { Box::from_raw(vec) };
                drop(vec)
            }

            #[doc(hidden)]
            #[export_name = #export_name_len]
            pub extern "C" fn _len(vec: *const Vec<#enum_name>) -> usize {
                unsafe { &*vec }.len()
            }

            #[doc(hidden)]
            #[export_name = #export_name_get]
            pub extern "C" fn _get(vec: *const Vec<#enum_name>, index: usize) -> #ffi_option_enum_repr {
                let vec = unsafe { &*vec };
                let val = vec.get(index).map(|v| *v);
                #ffi_option_enum_repr::from_rust_repr(val)
            }

            #[doc(hidden)]
            #[export_name = #export_name_get_mut]
            pub extern "C" fn _get_mut(vec: *mut Vec<#enum_name>, index: usize) -> #ffi_option_enum_repr {
                let vec = unsafe { &mut *vec };
                let val = vec.get_mut(index).map(|v| *v);
                #ffi_option_enum_repr::from_rust_repr(val)
            }

            #[doc(hidden)]
            #[export_name = #export_name_push]
            pub extern "C" fn _push(vec: *mut Vec<#enum_name>, val: #ffi_enum_repr) {
                unsafe { &mut *vec }.push( val.into_rust_repr() )
            }

            #[doc(hidden)]
            #[export_name = #export_name_pop]
            pub extern "C" fn _pop(vec: *mut Vec<#enum_name>) -> #ffi_option_enum_repr {
                let vec = unsafe { &mut *vec };
                let val = vec.pop();
                #ffi_option_enum_repr::from_rust_repr(val)
            }

            #[doc(hidden)]
            #[export_name = #export_name_as_ptr]
            pub extern "C" fn _as_ptr(vec: *const Vec<#enum_name>) -> *const #enum_name {
                unsafe { & *vec }.as_ptr()
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_tokens_eq;
    use proc_macro2::Span;

    /// Verify that we can generate the functions for an opaque Rust type that get exposed to Swift
    /// in order to power the `extension MyRustType: Vectorizable { }` implementation on the Swift
    /// side.
    #[test]
    fn generates_vectorizable_impl_for_opaque_rust_type() {
        let expected = quote! {
            const _: () = {
                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$new"]
                pub extern "C" fn _new() -> *mut Vec<AnEnum> {
                    Box::into_raw(Box::new(Vec::new()))
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$drop"]
                pub extern "C" fn _drop(vec: *mut Vec<AnEnum>) {
                    let vec = unsafe { Box::from_raw(vec) };
                    drop(vec)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$len"]
                pub extern "C" fn _len(vec: *const Vec<AnEnum>) -> usize {
                    unsafe { &*vec }.len()
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$get"]
                pub extern "C" fn _get(vec: *const Vec<AnEnum>, index: usize) -> __swift_bridge__Option_AnEnum {
                    let vec = unsafe { &*vec };
                    let val = vec.get(index).map(|v| *v);
                    __swift_bridge__Option_AnEnum::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$get_mut"]
                pub extern "C" fn _get_mut(vec: *mut Vec<AnEnum>, index: usize) -> __swift_bridge__Option_AnEnum {
                    let vec = unsafe { &mut *vec };
                    let val = vec.get_mut(index).map(|v| *v);
                    __swift_bridge__Option_AnEnum::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$push"]
                pub extern "C" fn _push(vec: *mut Vec<AnEnum>, val: __swift_bridge__AnEnum) {
                    unsafe { &mut *vec }.push(val.into_rust_repr())
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$pop"]
                pub extern "C" fn _pop(vec: *mut Vec<AnEnum>) -> __swift_bridge__Option_AnEnum {
                    let vec = unsafe { &mut *vec };
                    let val = vec.pop();
                    __swift_bridge__Option_AnEnum::from_rust_repr(val)
                }

                #[doc(hidden)]
                #[export_name = "__swift_bridge__$Vec_AnEnum$as_ptr"]
                pub extern "C" fn _as_ptr(vec: *const Vec<AnEnum>) -> *const AnEnum {
                    unsafe { & *vec }.as_ptr()
                }
            };
        };

        assert_tokens_eq(
            &generate_vec_of_transparent_enum_functions(&Ident::new("AnEnum", Span::call_site())),
            &expected,
        );
    }
}
