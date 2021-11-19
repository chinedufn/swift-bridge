use crate::built_in_types::BuiltInType;
use crate::parse::HostLang;
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, Pat, ReturnType, Type};

impl ParsedExternFn {
    /// Generates:
    ///
    /// ```
    /// # type ReturnTypeHere = ();
    /// // Functions in extern "Rust" blocks become
    /// #[no_mangle]
    /// #[export_name = "..."]
    /// pub extern "C" fn fn_name () -> ReturnTypeHere {
    ///   // ...
    /// }
    ///
    /// // Functions in extern "Swift" blocks become
    /// extern "C" {
    ///     #[link_name = "..."]
    ///     fn fn_name() -> ReturnTypeHere;
    /// }
    /// ```
    // FIXME: Combine this and host_type into one struct
    pub fn to_extern_c_function_tokens(&self) -> TokenStream {
        let link_name = self.link_name();

        let params = self.to_extern_c_param_names_and_types();

        let prefixed_fn_name = self.prefixed_fn_name();

        let ret = self.rust_return_type();

        match self.host_lang {
            HostLang::Rust => {
                let inner = self.inner_tokens();

                quote! {
                    #[no_mangle]
                    #[export_name = #link_name]
                    pub extern "C" fn #prefixed_fn_name ( #params ) #ret {
                        #inner
                    }
                }
            }
            HostLang::Swift => {
                quote! {
                    #[link_name = #link_name]
                    fn #prefixed_fn_name ( #params ) #ret;
                }
            }
        }
    }

    fn inner_tokens(&self) -> TokenStream {
        let sig = &self.func.sig;
        let host_type = self.associated_type.as_ref().map(|h| &h.ident);
        let fn_name = &sig.ident;

        let call_args = self.to_rust_call_args();

        let call_fn = quote! {
            #fn_name ( #call_args )
        };

        let call_fn = if self.is_method() {
            let this = if let Some(reference) = self.self_reference() {
                let maybe_ref = reference.0;
                let maybe_mut = self.self_mutability();

                quote! {
                    (unsafe { #maybe_ref #maybe_mut *this } )
                }
            } else {
                quote! {
                    ( * unsafe { Box::from_raw(this) } )
                }
            };

            quote! {
                    #this.#call_fn
            }
        } else {
            let host_type_segment = if let Some(h) = &host_type {
                quote! {#h::}
            } else {
                quote! {}
            };

            match &sig.output {
                ReturnType::Default => {
                    quote! {
                        super:: #host_type_segment #call_fn
                    }
                }
                ReturnType::Type(_arrow, ty) => {
                    if let Some(ty) = BuiltInType::with_type(&ty) {
                        quote! {
                            super:: #host_type_segment #call_fn
                        }
                    } else {
                        quote! {
                            Box::into_raw( Box::new( super:: #host_type_segment #call_fn )) as *mut std::ffi::c_void
                        }
                    }
                }
            }
        };

        let inner = match &sig.output {
            ReturnType::Default => {
                quote! {
                    #call_fn
                }
            }
            ReturnType::Type(_arrow, ty) => {
                if let Some(ty) = BuiltInType::with_type(&ty) {
                    match ty {
                        BuiltInType::RefSlice(_ref_slice) => {
                            quote! {
                                swift_bridge::RustSlice::from_slice(
                                    #call_fn
                                )
                            }
                        }
                        _ => {
                            quote! {
                                #call_fn
                            }
                        }
                    }
                } else {
                    quote! {
                        #call_fn
                    }
                }
            }
        };

        let mut unbox_arg_ptrs = vec![];

        for arg in &sig.inputs {
            match arg {
                FnArg::Receiver(_) => {}
                FnArg::Typed(pat_ty) => {
                    if BuiltInType::with_type(&pat_ty.ty).is_none() {
                        let (maybe_ref, maybe_mut) = match pat_ty.ty.deref() {
                            Type::Reference(ty_ref) => (Some(ty_ref.and_token), ty_ref.mutability),
                            _ => (None, None),
                        };
                        let arg_name = match pat_ty.pat.deref() {
                            Pat::Ident(ident) if ident.ident.to_string() == "self" => {
                                let this = Ident::new("this", ident.span());
                                quote! { #this }
                            }
                            _ => {
                                let arg_name = &pat_ty.pat;
                                quote! { #arg_name }
                            }
                        };

                        let unbox = quote! {
                            let #arg_name = unsafe { #maybe_ref #maybe_mut * #arg_name };
                        };
                        unbox_arg_ptrs.push(unbox);
                    }
                }
            }
        }

        let inner = quote! {
            #(#unbox_arg_ptrs)*
            #inner
        };
        inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ParseErrors;
    use crate::parse::SwiftBridgeModuleAndErrors;
    use crate::test_utils::{assert_tokens_eq, parse_ok};
    use crate::SwiftBridgeModule;
    use quote::quote;

    /// Verify that we convert &[T] -> swift_bridge::RustSlice<T>
    #[test]
    fn wraps_extern_rust_slice() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn make_slice () -> &'static [u8];
                }
            }
        };
        let expected_fn = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$make_slice"]
            pub extern "C" fn __swift_bridge__make_slice() -> swift_bridge::RustSlice<u8> {
                swift_bridge::RustSlice::from_slice(super::make_slice())
            }
        };

        let module = parse_ok(tokens);
        let function = &module.functions[0];

        assert_tokens_eq(&function.to_extern_c_function_tokens(), &expected_fn);
    }

    /// Verify that we can link to a Swift function.
    #[test]
    fn link_to_swift_freestanding_function() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Swift" {
                    fn count () -> u8;
                }
            }
        };
        let expected_fn = quote! {
            #[link_name = "__swift_bridge__$count"]
            fn __swift_bridge__count() -> u8;
        };

        assert_to_extern_c_function_tokens(tokens, &expected_fn);
    }

    fn assert_to_extern_c_function_tokens(module: TokenStream, expected_fn: &TokenStream) {
        let module = parse_ok(module);
        let function = &module.functions[0];

        assert_tokens_eq(&function.to_extern_c_function_tokens(), &expected_fn);
    }
}
