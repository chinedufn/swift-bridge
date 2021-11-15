use crate::module::{ExternRustItem, Module, ModuleSection};
use quote::ToTokens;
use quote::__private::{Ident, TokenStream};
use quote::quote;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::ForeignItemFn;

#[derive(Default)]
struct ExternRustFnAttributes {
    associated_to: Option<Ident>,
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.sections.len() == 0 {
            return;
        }

        let mod_name = format!("__swift_bridge_{}", self.name.to_string());
        let mod_name = Ident::new(&mod_name, self.name.span());

        let mut module_inner_tokens: Vec<TokenStream2> = vec![];

        for section in self.sections.iter() {
            match section {
                ModuleSection::ExternRust(extern_rust) => {
                    for item in extern_rust {
                        match item {
                            ExternRustItem::TypeDeclaration(ty) => {
                                let ty_ident = &ty.ident;
                                let export_name =
                                    &format!("swift_bridge$unstable${}$free", ty.ident.to_string());
                                let fn_name = Ident::new(
                                    &format!("free_{}", ty_ident.to_string()),
                                    ty.ident.span(),
                                );

                                let free = quote! {
                                    #[no_mangle]
                                    #[export_name = #export_name]
                                    pub extern "C" fn #fn_name (this: swift_bridge::OwnedPtrToRust<super::#ty_ident>) {
                                        drop(this);
                                    }
                                };
                                module_inner_tokens.push(free);
                            }
                            ExternRustItem::Func(func) => {
                                let tokens = parse_rust_func(func);
                                module_inner_tokens.push(tokens);
                            }
                        }
                    }
                }
                ModuleSection::ExternSwift => {}
            };
        }

        let mod_tokens = quote! {
            mod #mod_name {
                #(#module_inner_tokens)*
            }
        };

        mod_tokens.to_tokens(tokens);
    }
}

fn parse_rust_func(func: &ForeignItemFn) -> TokenStream {
    let mut attrs = ExternRustFnAttributes::default();

    for attr in func.attrs.iter() {
        let attrib: RustFnAttrib = attr.parse_args().unwrap();
        match attrib {
            RustFnAttrib::Associated(ident) => {
                attrs.associated_to = Some(ident);
            }
        }
    }

    if let Some(ty) = attrs.associated_to {
        let func_name = &func.sig.ident;
        let export_name = format!(
            "swift_bridge$unstable${}${}",
            ty.to_string().trim(),
            func_name.to_string()
        );

        let static_func_tokens = quote! {
            #[no_mangle]
            #[export_name = #export_name]
            pub extern "C" fn #func_name () -> swift_bridge::OwnedPtrToRust<super::#ty> {
                let val: super::#ty = super::#ty::#func_name();
                let val = Box::into_raw(Box::new(val));
                swift_bridge::OwnedPtrToRust::new(val)
            }
        };

        return static_func_tokens;
    } else {
        quote! {}
    }
}

enum RustFnAttrib {
    Associated(Ident),
}

impl Parse for RustFnAttrib {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>()?;
        input.parse::<syn::Token![=]>()?;

        let associated_to: Ident = input.parse()?;

        Ok(RustFnAttrib::Associated(associated_to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_tokens_eq;
    use quote::quote;
    use syn::__private::Span;
    use syn::parse_quote;

    /// Verify that we do not generate any tokens if the module did not have any sections.
    #[test]
    fn generate_nothing_if_module_empty() {
        let module = empty_module();
        let mut tokens = TokenStream2::new();

        module.to_tokens(&mut tokens);

        assert_tokens_eq(&tokens, &TokenStream2::new());
    }

    /// Verify that we properly generate externs for a Rust function with no arguments.
    #[test]
    fn rust_free_fn_no_args() {
        let original = quote! {
            mod foo {
                extern "Rust" {
                    fn new () -> u8;
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge_foo {
                #[no_mangle]
                #[export_name = "swift_bridge$unstable$new"]
                pub extern "C" fn new () -> u8 {
                    super::new()
                }
            }
        };

        let module: Module = parse_quote!(#original);

        let mut generated = TokenStream2::new();

        module.to_tokens(&mut generated);
        assert_tokens_eq(&generated, &expected);
    }

    /// Verify that we properly handle the `#[associated = SomeType]` annotation.
    #[test]
    fn associated_function() {
        let original = quote! {
            mod foo {
                extern "Rust" {
                    type ARustStruct;

                    #[swift_bridge(associated_to = ARustStruct)]
                    fn new () -> ARustStruct;
                }
            }
        };
        let expected = quote! {
            mod __swift_bridge_foo {
                #[no_mangle]
                #[export_name = "swift_bridge$unstable$ARustStruct$free"]
                pub extern "C" fn free_ARustStruct (this: swift_bridge::OwnedPtrToRust<super::ARustStruct>) {
                    drop(this);
                }

                #[no_mangle]
                #[export_name = "swift_bridge$unstable$ARustStruct$new"]
                pub extern "C" fn new () -> swift_bridge::OwnedPtrToRust<super::ARustStruct> {
                    let val: super::ARustStruct = super::ARustStruct::new();
                    let val = Box::into_raw(Box::new(val));
                    swift_bridge::OwnedPtrToRust::new(val)
                }
            }
        };

        let module: Module = parse_quote!(#original);

        let mut generated = TokenStream2::new();

        module.to_tokens(&mut generated);
        assert_tokens_eq(&generated, &expected);
    }

    fn empty_module() -> Module {
        Module {
            name: Ident::new("Foo", Span::call_site()),
            sections: vec![],
            errors: vec![],
        }
    }
}
