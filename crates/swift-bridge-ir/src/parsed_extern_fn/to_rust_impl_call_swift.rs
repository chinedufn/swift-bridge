use crate::build_in_types::BuiltInType;
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::ops::Deref;
use syn::{FnArg, Type};

/// Generates the
///
/// `pub fn new () -> Foo { ... }`
///
/// in the following:
///
/// ```
/// # use std::ffi::c_void;
/// struct Foo(*mut c_void);
///
/// impl Foo {
///     pub fn new () -> Foo {
///         Foo(unsafe{ Foo_new() })
///     }
/// }
/// extern "C" {
///     #[link_name = "__swift_bridge__$Foo$new"]
///     fn Foo_new() -> *mut c_void;
/// }
/// ```
impl ParsedExternFn {
    pub fn to_impl_fn_calls_swift(&self) -> TokenStream {
        let sig = &self.func.sig;
        let fn_name = &sig.ident;
        let ty_name = &self.associated_type.as_ref().unwrap().ident;

        let ret = &sig.output;
        let params = self.params_with_normalized_self();
        let call_args = self.to_rust_call_args();
        let linked_fn_name = self.extern_swift_linked_fn_new();

        let mut inner = quote! {
            unsafe { #linked_fn_name(#call_args) }
        };

        if BuiltInType::with_return_type(ret).is_none() {
            inner = quote! {
                #ty_name ( #inner )
            }
        }

        quote! {
            pub fn #fn_name(#params) #ret {
                #inner
            }
        }
    }

    // All of the params but with explicit types removed from `self`.
    //
    // `self: Foo` becomes `self`,
    // `self: &Foo` -> `&self`,
    // `self: &mut Foo` -> `&mut self`
    fn params_with_normalized_self(&self) -> TokenStream {
        let params = self
            .sig
            .inputs
            .iter()
            .map(|arg| match arg {
                FnArg::Typed(pat_ty) if pat_ty.pat.to_token_stream().to_string() == "self" => {
                    match pat_ty.ty.deref() {
                        Type::Reference(reference) => {
                            let ref_token = reference.and_token;
                            let maybe_mut = reference.mutability;

                            quote! {
                                #ref_token #maybe_mut self
                            }
                        }
                        _ => {
                            quote! {
                                #arg
                            }
                        }
                    }
                }
                _ => {
                    quote! {
                        #arg
                    }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #(#params),*
        }
    }
}
