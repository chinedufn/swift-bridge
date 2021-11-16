use crate::{SwiftBridgeModule, SWIFT_BRIDGE_PREFIX};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::FnArg;

impl ToTokens for SwiftBridgeModule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mod_name = Ident::new(
            &format!("__swift_bridge__{}", self.name.to_string()),
            self.name.span(),
        );

        let mut generated = vec![];

        for rust_section in &self.extern_rust {
            for freefunc in &rust_section.freestanding_fns {
                let sig = &freefunc.func.sig;
                let export_name = format!("{}${}", SWIFT_BRIDGE_PREFIX, sig.ident.to_string());

                let fn_name = &sig.ident;

                let output = &sig.output;

                // foo: u8, bar: u32
                let args = &sig.inputs;
                // foo, bar
                let arg_var_names = sig
                    .inputs
                    .iter()
                    .map(|arg| {
                        match arg {
                            FnArg::Typed(pat_ty) => &pat_ty.pat,
                            FnArg::Receiver(_) => {
                                // When we parsed our freestanding functions we checked that they
                                // did not have a receiver.
                                unreachable!()
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                let t = quote! {
                    #[no_mangle]
                    #[export_name = #export_name]
                    pub extern "C" fn #fn_name (#args) #output {
                        super::#fn_name(#(#arg_var_names),*)
                    }
                };
                generated.push(t);
            }

            for ty in &rust_section.types {
                let export_name =
                    format!("{}${}$_free", SWIFT_BRIDGE_PREFIX, ty.ty.ident.to_string(),);
                let func_name = Ident::new(
                    &format!("{}__free", ty.ty.ident.to_string()),
                    ty.ty.ident.span(),
                );
                let this = &ty.ty.ident;

                let free = quote! {
                    #[no_mangle]
                    #[export_name = #export_name]
                    pub extern "C" fn #func_name (this: swift_bridge::OwnedPtrToRust<super::#this>) {
                        drop(this);
                    }
                };
                generated.push(free);

                for type_method in &ty.methods {
                    generated.push(type_method.extern_rust_tokens(&ty.ty.ident));
                }
            }
        }

        let t = quote! {
            mod #mod_name {
                #(#generated)*
            }
        };
        t.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::SwiftBridgeModuleAndErrors;
    use crate::test_utils::{assert_tokens_contain, assert_tokens_eq};
    use proc_macro2::Span;
    use quote::quote;
    use syn::parse_quote;

    /// Verify that we generate a function that frees the memory behind an opaque pointer to a Rust
    /// type.
    #[test]
    fn free_opaque_rust_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge__foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$SomeType$_free"]
                pub extern "C" fn SomeType__free (this: swift_bridge::OwnedPtrToRust<super::SomeType>) {
                    drop(this);
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with no arguments.
    #[test]
    fn freestanding_rust_function_no_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> *const u8;
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge__foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn some_function () -> *const u8 {
                    super::some_function()
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with no arguments.
    #[test]
    fn freestanding_one_arg() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function (bar: u8) -> *const u8;
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge__foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn some_function (bar: u8) -> *const u8 {
                    super::some_function(bar)
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with a return type.
    #[test]
    fn freestanding_with_return_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> *const u8;
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge__foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn some_function () -> *const u8 {
                    super::some_function()
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a static method that's associated to a type.
    #[test]
    fn associated_static_method_no_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    #[swift_bridge(associated_to = SomeType)]
                    fn new () -> SomeType;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$new"]
            pub extern "C" fn SomeType_new () -> swift_bridge::OwnedPtrToRust<super::SomeType> {
                let val = super::SomeType::new();
                let val = Box::into_raw(Box::new(val));
                swift_bridge::OwnedPtrToRust::new(val)
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .extern_rust_tokens(&Ident::new("SomeType", Span::call_site()));
        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate tokens for a static method that have arguments.
    #[test]
    fn associated_static_method_one_arg() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    #[swift_bridge(associated_to = SomeType)]
                    fn new (foo: u8) -> u8;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$new"]
            pub extern "C" fn SomeType_new (foo: u8) -> u8 {
                super::SomeType::new(foo)
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .extern_rust_tokens(&Ident::new("SomeType", Span::call_site()));
        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate tokens for a static method that does not have a return type.
    #[test]
    fn associated_static_method_no_return_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    #[swift_bridge(associated_to = SomeType)]
                    fn new ();
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$new"]
            pub extern "C" fn SomeType_new ()  {
                 super::SomeType::new()
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .extern_rust_tokens(&Ident::new("SomeType", Span::call_site()));
        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate the tokens for exposing an associated method.
    #[test]
    fn associated_method_no_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    fn increment (&mut self);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$increment"]
            pub extern "C" fn SomeType_increment (this: swift_bridge::OwnedPtrToRust<super::SomeType>) {
                let this = unsafe { &mut *this.ptr };
                this.increment()
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .extern_rust_tokens(&Ident::new("SomeType", Span::call_site()));
        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate the tokens for exposing an associated method.
    #[test]
    fn associated_method_one_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    fn message (&self, val: u8);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$message"]
            pub extern "C" fn SomeType_message (
                this: swift_bridge::OwnedPtrToRust<super::SomeType>,
                val: u8
            ) {
                let this = unsafe { &*this.ptr };
                this.message(val)
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .extern_rust_tokens(&Ident::new("SomeType", Span::call_site()));
        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that type method tokens get written into the final token stream.
    /// We have other tests that verify that the generated method tokens are correct.
    /// This test just verifies that we're actually making use of the generated function tokens.
    #[test]
    fn writes_method_tokens() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    fn new (&self) -> SomeType;
                }
            }
        };
        let module = parse_ok(start);
        let tokens = module.to_token_stream();

        assert_tokens_contain(&tokens, &quote! { SomeType_new });
    }

    #[test]
    fn todo() {
        todo!(
            r#"
Split the generating of the return type tokens into a submodule..
Then add tests for whether or not we convert the type (i.e. to a OwnedPtrToRust.. RustString.. or
something else)
"#
        )
    }

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        let module_and_errors: SwiftBridgeModuleAndErrors = parse_quote!(#tokens);
        module_and_errors.module
    }
}
