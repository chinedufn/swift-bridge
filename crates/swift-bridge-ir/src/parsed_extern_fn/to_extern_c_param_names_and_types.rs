use crate::bridged_type::{pat_type_pat_is_self, BridgedType};
use crate::parse::{HostLang, SharedTypeDeclaration, TypeDeclaration, TypeDeclarations};
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, Path, Type};

impl ParsedExternFn {
    pub fn to_extern_c_param_names_and_types(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let host_type = self.associated_type.as_ref().map(|h| match h {
            TypeDeclaration::Shared(_) => {
                todo!()
            }
            TypeDeclaration::Opaque(h) => &h.ident,
        });
        let mut params = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => match self.host_lang {
                    HostLang::Rust => {
                        let this = host_type.as_ref().unwrap();
                        let this = quote! { this: *mut super:: #this };
                        params.push(this);
                    }
                    HostLang::Swift => {
                        let this = quote! { this: #swift_bridge_path::PointerToSwiftType };
                        params.push(this);
                    }
                },
                FnArg::Typed(pat_ty) => {
                    let pat_ty_is_self = pat_type_pat_is_self(pat_ty);

                    let arg_name = if pat_ty_is_self {
                        if self.host_lang.is_rust() {
                            let this = Ident::new("this", pat_ty.span());
                            quote! {
                                #this
                            }
                        } else {
                            let this = quote! { this: #swift_bridge_path::PointerToSwiftType };
                            params.push(this);

                            continue;
                        }
                    } else if let Some(built_in) = BridgedType::new_with_type(&pat_ty.ty, types) {
                        let pat = &pat_ty.pat;
                        let ty = built_in.to_ffi_compatible_rust_type(swift_bridge_path);
                        params.push(quote! { #pat: #ty});
                        continue;
                    } else {
                        todo!("Push to ParsedErrors")
                    };

                    let mut bridged_type = &pat_ty.ty;

                    // `&Foo` becomes `Foo`
                    // `&mut Foo` beomces `Foo`
                    if let Type::Reference(ty_ref) = pat_ty.ty.deref() {
                        bridged_type = &ty_ref.elem;
                    };

                    if self.host_lang.is_rust() {
                        let arg_ty = match types
                            .get(&bridged_type.to_token_stream().to_string())
                            .unwrap()
                        {
                            TypeDeclaration::Shared(SharedTypeDeclaration::Struct(
                                shared_struct,
                            )) => {
                                let ty = &shared_struct.name;
                                quote! {
                                    #ty
                                }
                            }
                            TypeDeclaration::Shared(SharedTypeDeclaration::Enum(_shared_enum)) => {
                                //
                                todo!("Shared enum to type name")
                            }
                            TypeDeclaration::Opaque(opaque) => {
                                if opaque.host_lang.is_rust() {
                                    quote! {
                                        *mut super::#bridged_type
                                    }
                                } else {
                                    quote! {
                                        *mut std::ffi::c_void
                                    }
                                }
                            }
                        };

                        params.push(quote! {
                             #arg_name: #arg_ty
                        });
                    } else {
                        if pat_type_pat_is_self(&pat_ty) {
                            params.push(quote! {
                                 #arg_name: *mut std::ffi::c_void
                            });

                            continue;
                        }

                        let arg_ty = types
                            .get(&bridged_type.to_token_stream().to_string())
                            .unwrap();

                        let arg_ty_tokens = if self.host_lang.is_rust() {
                            match arg_ty {
                                TypeDeclaration::Shared(_) => {
                                    todo!("Add a test that hits this code path")
                                }
                                TypeDeclaration::Opaque(opaque) => {
                                    if opaque.host_lang.is_swift() {
                                        quote! { *mut std::ffi::c_void }
                                    } else {
                                        let ty = &opaque.ty.ident;
                                        quote! { *mut super::#ty }
                                    }
                                }
                            }
                        } else {
                            match arg_ty {
                                TypeDeclaration::Shared(_) => {
                                    todo!("Add a test that hits this code path")
                                }
                                TypeDeclaration::Opaque(opaque) => {
                                    let ty = &opaque.ty.ident;

                                    if opaque.host_lang.is_swift() {
                                        quote! { #ty }
                                    } else {
                                        quote! { *mut super::#ty }
                                    }
                                }
                            }
                        };

                        params.push(quote! {
                             #arg_name: #arg_ty_tokens
                        });
                    }
                }
            };
        }

        quote! {
            #(#params),*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_tokens_contain, assert_tokens_eq, parse_ok};

    /// Verify that we rename `self` parameters to `this`
    #[test]
    fn renames_self_to_this_in_params() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type Foo;
                    fn make1 (self);
                    fn make2 (&self);
                    fn make3 (&mut self);
                    fn make4 (self: Foo);
                    fn make5 (self: &Foo);
                    fn make6 (self: &mut Foo);
                }
            }
        };
        let module = parse_ok(tokens);
        let methods = &module.functions;
        assert_eq!(methods.len(), 6);

        for method in methods {
            assert_tokens_contain(
                &method.to_extern_c_param_names_and_types(&module.swift_bridge_path, &module.types),
                &quote! { this },
            );
        }
    }

    /// Verify that a String parameter gets turned into a *mut RustString
    #[test]
    fn converts_string_param_to_ruststring_pointer() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn foo (s: String);
                }
            }
        };
        let expected = quote! { s: *mut swift_bridge::string::RustString };
        assert_params_eq(tokens, &expected);
    }

    fn assert_params_eq(tokens: TokenStream, expected_params: &TokenStream) {
        let module = parse_ok(tokens);
        let funcs = &module.functions;

        assert_tokens_eq(
            &funcs[0].to_extern_c_param_names_and_types(&module.swift_bridge_path, &module.types),
            expected_params,
        );
    }
}
