use crate::bridged_type::BridgedType;
use crate::parse::{HostLang, TypeDeclaration, TypeDeclarations};
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

impl ParsedExternFn {
    /// Generates:
    ///
    /// ```
    /// # type ReturnTypeHere = ();
    /// // Functions in extern "Rust" blocks become
    /// #[export_name = "..."]
    /// pub extern "C" fn a_function_name () -> ReturnTypeHere {
    ///   // ...
    /// }
    ///
    /// // Functions in extern "Swift" blocks become
    /// extern "C" {
    ///     #[link_name = "..."]
    ///     fn some_function() -> ReturnTypeHere;
    /// }
    /// ```
    pub fn to_extern_c_function_tokens(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let link_name = self.link_name();

        let params = self.to_extern_c_param_names_and_types(swift_bridge_path, types);

        let prefixed_fn_name = self.prefixed_fn_name();

        let ret = self.rust_fn_sig_return_tokens(swift_bridge_path, types);

        match self.host_lang {
            HostLang::Rust => {
                let call_fn = self.call_fn_tokens(swift_bridge_path, types);

                let maybe_return_ty = self.maybe_async_rust_fn_return_ty(swift_bridge_path, types);

                if self.sig.asyncness.is_none() {
                    quote! {
                        #[export_name = #link_name]
                        pub extern "C" fn #prefixed_fn_name ( #params ) #ret {
                            #call_fn
                        }
                    }
                } else {
                    let (call_fn, call_callback) = if maybe_return_ty.is_some() {
                        (
                            quote! {
                                let val = #call_fn.await;
                            },
                            quote! {
                                (callback)(callback_wrapper, val)
                            },
                        )
                    } else {
                        (
                            quote! {
                                #call_fn.await;
                            },
                            quote! {
                                (callback)(callback_wrapper)
                            },
                        )
                    };

                    quote! {
                        #[export_name = #link_name]
                        pub extern "C" fn #prefixed_fn_name (
                            callback_wrapper: *mut std::ffi::c_void,
                            callback: extern "C" fn(*mut std::ffi::c_void #maybe_return_ty) -> ()
                        ) {
                            let callback_wrapper = swift_bridge::async_support::SwiftCallbackWrapper(callback_wrapper);
                            let task = async move {
                                #call_fn

                                let callback_wrapper = callback_wrapper;
                                let callback_wrapper = callback_wrapper.0;

                                #call_callback
                            };
                            swift_bridge::async_support::ASYNC_RUNTIME.spawn_task(Box::pin(task))
                        }
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

    fn call_fn_tokens(&self, swift_bridge_path: &Path, types: &TypeDeclarations) -> TokenStream {
        let sig = &self.func.sig;
        let fn_name = if let Some(fn_name) = self.rust_name_override.as_ref() {
            let span = fn_name.span();
            let fn_name = Ident::new(&fn_name.value(), span);

            quote! { #fn_name }
        } else {
            let fn_name = &sig.ident;
            quote! {
                #fn_name
            }
        };

        let call_args = self.to_call_rust_args(swift_bridge_path, types);

        let call_fn = quote! {
            #fn_name ( #call_args )
        };

        let mut call_fn = if self.is_method() {
            self.call_method_tokens(&call_fn)
        } else {
            self.call_function_tokens(&call_fn)
        };

        let return_ty = self.return_ty_built_in(types).unwrap();

        if self.into_return_type {
            call_fn = return_ty.rust_expression_into(&call_fn);
        }

        if let Some(return_with) = self.return_with.as_ref() {
            call_fn = quote! {
                super:: #return_with ( #call_fn )
            }
        }

        call_fn = return_ty.convert_rust_value_to_ffi_compatible_value(&call_fn, swift_bridge_path);

        call_fn
    }

    /// Generate tokens for calling a method.
    fn call_method_tokens(&self, call_fn: &TokenStream) -> TokenStream {
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
    }

    /// Generate tokens for calling a freestanding or an associated function.
    fn call_function_tokens(&self, call_fn: &TokenStream) -> TokenStream {
        let maybe_associated_type = self.associated_type.as_ref().map(|ty| {
            match ty {
                TypeDeclaration::Shared(_) => {
                    //
                    todo!()
                }
                TypeDeclaration::Opaque(ty) => {
                    let ty = &ty.ident;
                    quote! {#ty::}
                }
            }
        });

        quote! {
            super:: #maybe_associated_type #call_fn
        }
    }

    /// If the functions return type is a BuiltInType, return it.
    pub(crate) fn return_ty_built_in(&self, types: &TypeDeclarations) -> Option<BridgedType> {
        BridgedType::new_with_return_type(&self.func.sig.output, types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_tokens_eq, parse_ok};
    use quote::quote;

    /// Verify that we convert &[T] -> swift_bridge::FfiSlice<T>
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
            #[export_name = "__swift_bridge__$make_slice"]
            pub extern "C" fn __swift_bridge__make_slice() -> swift_bridge::FfiSlice<u8> {
                swift_bridge::FfiSlice::from_slice(super::make_slice())
            }
        };

        assert_to_extern_c_function_tokens(tokens, &expected_fn);
    }

    /// Verify that we convert String -> swift_bridge::RustString
    #[test]
    fn wraps_string_in_rust_string() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn make_string () -> String;
                }
            }
        };
        let expected_fn = quote! {
            #[export_name = "__swift_bridge__$make_string"]
            pub extern "C" fn __swift_bridge__make_string() -> *mut swift_bridge::string::RustString {
                swift_bridge::string::RustString(super::make_string()).box_into_raw()
            }
        };

        assert_to_extern_c_function_tokens(tokens, &expected_fn);
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

        assert_tokens_eq(
            &function.to_extern_c_function_tokens(&module.swift_bridge_path, &module.types),
            &expected_fn,
        );
    }
}
