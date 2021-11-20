use crate::built_in_types::{BuiltInRefSlice, BuiltInType};
use crate::parse::HostLang;
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, ReturnType, Type};

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
    pub fn to_extern_c_function_tokens(&self, swift_bridge_path: &Path) -> TokenStream {
        let link_name = self.link_name();

        let params = self.to_extern_c_param_names_and_types(swift_bridge_path);

        let prefixed_fn_name = self.prefixed_fn_name();

        let ret = self.rust_return_type(swift_bridge_path);

        match self.host_lang {
            HostLang::Rust => {
                let call_fn = self.call_fn_tokens(swift_bridge_path);

                quote! {
                    #[no_mangle]
                    #[export_name = #link_name]
                    pub extern "C" fn #prefixed_fn_name ( #params ) #ret {
                        #call_fn
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

    fn call_fn_tokens(&self, swift_bridge_path: &Path) -> TokenStream {
        let sig = &self.func.sig;
        let fn_name = &sig.ident;

        let call_args = self.to_rust_call_args();

        let call_fn = quote! {
            #fn_name ( #call_args )
        };

        let mut call_fn = if self.is_method() {
            self.call_method_tokens(&call_fn)
        } else {
            self.call_function_tokens(&call_fn)
        };

        if let Some(ty) = self.return_ty_built_in() {
            if ty.is_ref_slice() {
                call_fn = quote! {
                    #swift_bridge_path::RustSlice::from_slice( #call_fn )
                };
            } else if ty.is_string() {
                call_fn = quote! {
                    #swift_bridge_path::string::RustString( #call_fn ).box_into_raw()
                };
            }
        }

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
        let host_type = self.associated_type.as_ref().map(|h| &h.ident);
        let sig = &self.func.sig;

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
                if let Some(_ty) = BuiltInType::with_type(&ty) {
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
    }

    /// If the function returns a slice we return that slice's type.
    /// So we return the the `T` in `&[T]`
    fn returned_slice(&self) -> Option<BuiltInRefSlice> {
        let sig = &self.func.sig;
        match &sig.output {
            ReturnType::Type(_arrow, ty) => {
                if let Some(ty) = BuiltInType::with_type(&ty) {
                    match ty {
                        BuiltInType::RefSlice(ref_slice) => Some(ref_slice),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// If the functions return type is a BuiltInType, return it.
    fn return_ty_built_in(&self) -> Option<BuiltInType> {
        BuiltInType::with_return_type(&self.func.sig.output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_tokens_eq, parse_ok};
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
            #[no_mangle]
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
            &function.to_extern_c_function_tokens(&syn::parse2(quote! {swift_bridge}).unwrap()),
            &expected_fn,
        );
    }
}
