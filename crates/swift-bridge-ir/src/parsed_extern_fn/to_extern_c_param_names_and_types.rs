use crate::built_in_types::BuiltInType;
use crate::parse::HostLang;
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, Pat, Path, Type};

impl ParsedExternFn {
    pub fn to_extern_c_param_names_and_types(&self, swift_bridge_path: &Path) -> TokenStream {
        let host_type = self.associated_type.as_ref().map(|h| &h.ident);
        let mut params = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(receiver) => match self.host_lang {
                    HostLang::Rust => {
                        let this = host_type.as_ref().unwrap();
                        let this = quote! { this: *mut super:: #this };
                        params.push(this);
                    }
                    HostLang::Swift => {
                        let this = quote! { this: *mut std::ffi::c_void };
                        params.push(this);
                    }
                },
                FnArg::Typed(pat_ty) => {
                    if let Some(built_in) = BuiltInType::with_type(&pat_ty.ty) {
                        params.push(quote! {#pat_ty});
                    } else {
                        let arg_name = match pat_ty.pat.deref() {
                            Pat::Ident(this) if this.ident.to_string() == "self" => {
                                let this = Ident::new("this", this.span());
                                quote! {
                                    #this
                                }
                            }
                            _ => {
                                let arg_name = &pat_ty.pat;
                                quote! {
                                    #arg_name
                                }
                            }
                        };

                        if self.host_lang.is_rust() {
                            let declared_ty = match pat_ty.ty.deref() {
                                Type::Reference(ty_ref) => {
                                    let ty = &ty_ref.elem;
                                    quote! {#ty}
                                }
                                Type::Path(path) => {
                                    quote! {#path}
                                }
                                _ => todo!(),
                            };

                            params.push(quote! {
                                 #arg_name: *mut super::#declared_ty
                            });
                        } else {
                            params.push(quote! {
                                 #arg_name: *mut std::ffi::c_void
                            });
                        }
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
    use crate::errors::ParseErrors;
    use crate::parse::SwiftBridgeModuleAndErrors;
    use crate::test_utils::{assert_tokens_contain, assert_tokens_eq};
    use crate::SwiftBridgeModule;

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
                &method.to_extern_c_param_names_and_types(&syn::parse2(quote! { abc123 }).unwrap()),
                &quote! { this },
            );
        }
    }

    /// Verify that a String parameter gets turned into a *mut RustString
    #[test]
    fn converts_string_param_to_ruststring_pointer() {
        todo!(
            r#"
Refactor this file's implementation.. Then it should be easier to convert params
"#
        );
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
            &funcs[0]
                .to_extern_c_param_names_and_types(&syn::parse2(quote! { swift_bridge }).unwrap()),
            expected_params,
        );
    }

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        let module_and_errors: SwiftBridgeModuleAndErrors = syn::parse2(tokens).unwrap();
        module_and_errors.module
    }

    fn parse_errors(tokens: TokenStream) -> ParseErrors {
        let parsed: SwiftBridgeModuleAndErrors = syn::parse2(tokens).unwrap();
        parsed.errors
    }
}
