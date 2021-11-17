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
                generated.push(freefunc.to_extern_rust_function_tokens(None));
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
                    pub extern "C" fn #func_name (this: *mut super::#this) {
                        let this = unsafe { Box::from_raw(this) };
                        drop(this);
                    }
                };
                generated.push(free);

                for type_method in &ty.methods {
                    generated.push(type_method.to_extern_rust_function_tokens(Some(&ty.ty.ident)));
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
                pub extern "C" fn SomeType__free (
                    this: *mut super::SomeType
                ) {
                    let this = unsafe { Box::from_raw(this) };
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
                    fn some_function ();
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge__foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn __swift_bridge__some_function () {
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
                    fn some_function (bar: u8);
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge__foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn __swift_bridge__some_function (bar: u8) {
                    super::some_function(bar)
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with an argument of a
    /// declared type.
    #[test]
    fn freestanding_func_with_declared_type_arg() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type MyType;
                    fn some_function (bar: &MyType);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function (
                bar: *mut std::mem::ManuallyDrop<super::MyType>
            ) {
                let bar = & unsafe { Box::from_raw(bar) };
                super::some_function(bar)
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with a return type.
    #[test]
    fn freestanding_with_built_in_return_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> u8;
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge__foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn __swift_bridge__some_function () -> u8 {
                    super::some_function()
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding function that returns a declared type.
    #[test]
    fn freestanding_function_with_declared_return_type() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Rust" {
                    type Foo;
                    fn some_function () -> Foo;
                }
            }
        };
        let expected_func = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function () -> *mut std::ffi::c_void {
                let val = super::some_function();
                Box::into_raw(Box::new(val)) as *mut std::ffi::c_void
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
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
            pub extern "C" fn __swift_bridge__SomeType_new () -> *mut std::ffi::c_void {
                let val = super::SomeType::new();
                 Box::into_raw(Box::new(val)) as *mut std::ffi::c_void
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .to_extern_rust_function_tokens(Some(&Ident::new("SomeType", Span::call_site())));
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
            pub extern "C" fn __swift_bridge__SomeType_new (foo: u8) -> u8 {
                super::SomeType::new(foo)
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .to_extern_rust_function_tokens(Some(&Ident::new("SomeType", Span::call_site())));
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
            pub extern "C" fn __swift_bridge__SomeType_new ()  {
                 super::SomeType::new()
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .to_extern_rust_function_tokens(Some(&Ident::new("SomeType", Span::call_site())));
        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we generate the tokens for exposing an associated method.
    #[test]
    fn associated_method_no_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type MyType;
                    fn increment (&mut self);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$MyType$increment"]
            pub extern "C" fn __swift_bridge__MyType_increment (
                this: *mut std::mem::ManuallyDrop<super::MyType>
            ) {
                let mut this = unsafe { Box::from_raw(this) };
                this.increment()
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .to_extern_rust_function_tokens(Some(&Ident::new("MyType", Span::call_site())));
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
            pub extern "C" fn __swift_bridge__SomeType_message (
                this: *mut std::mem::ManuallyDrop<super::SomeType>,
                val: u8
            ) {
                let this = unsafe { Box::from_raw(this) };
                this.message(val)
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .to_extern_rust_function_tokens(Some(&Ident::new("SomeType", Span::call_site())));
        assert_tokens_eq(&tokens, &expected);
    }

    /// Verify that we do not wrap the type in manually drop if the method uses owned self.
    #[test]
    fn associated_method_drops_owned_self() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    fn consume (self);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$consume"]
            pub extern "C" fn __swift_bridge__SomeType_consume (
                this: *mut super::SomeType
            ) {
                let this = unsafe { Box::from_raw(this) };
                (*this).consume()
            }
        };

        let module = parse_ok(start);
        let tokens = module.extern_rust[0].types[0].methods[0]
            .to_extern_rust_function_tokens(Some(&Ident::new("SomeType", Span::call_site())));
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

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        let module_and_errors: SwiftBridgeModuleAndErrors = syn::parse2(tokens).unwrap();
        module_and_errors.module
    }
}
