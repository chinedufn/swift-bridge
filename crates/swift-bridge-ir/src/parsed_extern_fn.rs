use crate::build_in_types::BuiltInType;
use crate::{SelfRefMut, SWIFT_BRIDGE_PREFIX};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, ForeignItemFn, Pat, PatType, ReturnType, Type};

mod to_swift;

pub(crate) struct ParsedExternFn {
    pub func: ForeignItemFn,
}

impl ParsedExternFn {
    pub fn new(func: ForeignItemFn) -> Self {
        ParsedExternFn { func }
    }
}

impl ParsedExternFn {
    /// Generates:
    ///
    /// ```
    /// # type ReturnTypeHere = ();
    /// #[no_mangle]
    /// #[export_name = "..."]
    /// pub extern "C" fn fn_name () -> ReturnTypeHere {
    ///   // ...
    /// }
    /// ```
    // FIXME: Combine this and host_type into one struct
    pub fn to_extern_rust_function_tokens(
        &self,
        this: Option<SelfRefMut>,
        host_type: Option<&Ident>,
    ) -> TokenStream {
        let sig = &self.func.sig;
        let fn_name = &sig.ident;

        let export_name = self.link_name(host_type);

        let params = self.to_rust_param_names_and_types(host_type);
        let call_args = self.to_rust_call_args();

        let call_fn = quote! {
            #fn_name ( #call_args )
        };

        let inner = if let Some(this) = this.as_ref() {
            if let Some(reference) = this.reference {
                let maybe_mut = &this.mutability;

                quote! {
                    let #maybe_mut this = unsafe { Box::from_raw(this) };
                    this.#call_fn
                }
            } else {
                quote! {
                    let this = unsafe { Box::from_raw(this) };
                    (*this).#call_fn
                }
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
                        match ty {
                            BuiltInType::RefSlice(_ref_slice) => {
                                quote! {
                                    swift_bridge::RustSlice::from_slice(
                                        super:: #host_type_segment #call_fn
                                    )
                                }
                            }
                            _ => {
                                quote! {
                                    super:: #host_type_segment #call_fn
                                }
                            }
                        }
                    } else {
                        quote! {
                            let val = super:: #host_type_segment #call_fn;
                            Box::into_raw(Box::new(val)) as *mut std::ffi::c_void
                        }
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
                            let #arg_name = #maybe_ref #maybe_mut unsafe { Box::from_raw(#arg_name) };
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

        let host_type_prefix = host_type
            .map(|h| format!("{}_", h.to_string()))
            .unwrap_or_default();
        let prefixed_fn_name = Ident::new(
            &format!(
                "{}{}{}",
                SWIFT_BRIDGE_PREFIX,
                host_type_prefix,
                fn_name.to_string()
            ),
            fn_name.span(),
        );

        let ret = match &sig.output {
            ReturnType::Default => {
                quote! {}
            }
            ReturnType::Type(arrow, ty) => {
                if let Some(built_in) = BuiltInType::with_type(&ty) {
                    let ty = built_in.to_extern_rust_ident(ty.span());
                    quote! {#arrow #ty}
                } else {
                    quote_spanned! {ty.span()=> -> *mut std::ffi::c_void }
                }
            }
        };

        quote! {
            #[no_mangle]
            #[export_name = #export_name]
            pub extern "C" fn #prefixed_fn_name ( #params ) #ret {
                #inner
            }
        }
    }

    pub fn to_rust_param_names_and_types(&self, host_type: Option<&Ident>) -> TokenStream {
        let mut params = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(receiver) => {
                    let this = host_type.as_ref().unwrap();
                    let this = if receiver.reference.is_some() {
                        quote! { this: *mut std::mem::ManuallyDrop<super:: #this> }
                    } else {
                        quote! { this: *mut super:: #this }
                    };
                    params.push(this);
                }
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

                        let is_owned = match pat_ty.ty.deref() {
                            Type::Reference(ty_ref) => false,
                            _ => true,
                        };
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

                        if is_owned {
                            params.push(quote! {
                                 #arg_name: *mut super::<#declared_ty>
                            })
                        } else {
                            params.push(quote! {
                                 #arg_name: *mut std::mem::ManuallyDrop<super::#declared_ty>
                            })
                        }
                    }
                }
            };
        }

        quote! {
            #(#params),*
        }
    }

    // fn foo (&self, arg1: u8, arg2: u32, &SomeType)
    //  becomes..
    // arg1, arg2, & unsafe { Box::from_raw(bar }
    pub fn to_rust_call_args(&self) -> TokenStream {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => {}
                FnArg::Typed(pat_ty) => {
                    match pat_ty.pat.deref() {
                        Pat::Ident(this) if this.ident.to_string() == "self" => {
                            continue;
                        }
                        _ => {}
                    };

                    let pat = &pat_ty.pat;

                    args.push(quote! {#pat});
                }
            };
        }

        quote! {
            #(#args),*
        }
    }

    // fn foo (&self, arg1: u8, arg2: u32)
    //  becomes..
    // void* self, uint8_t u8, uint32_t arg2
    pub fn to_c_header_params(&self) -> String {
        let mut params = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => params.push("void* self".to_string()),
                FnArg::Typed(pat_ty) => {
                    let pat = &pat_ty.pat;

                    match pat.deref() {
                        Pat::Ident(pat_ident) if pat_ident.ident.to_string() == "self" => {
                            params.push("void* self".to_string());
                        }
                        _ => {
                            let ty = if let Some(built_in) = BuiltInType::with_type(&pat_ty.ty) {
                                built_in.to_c().to_string()
                            } else {
                                pat.to_token_stream().to_string()
                            };

                            let arg_name = pat_ty.pat.to_token_stream().to_string();
                            params.push(format!("{} {}", ty, arg_name));
                        }
                    };
                }
            };
        }

        if params.len() == 0 {
            "void".to_string()
        } else {
            params.join(", ")
        }
    }

    pub fn to_c_header_return(&self) -> String {
        match &self.func.sig.output {
            ReturnType::Default => "void".to_string(),
            ReturnType::Type(_, ty) => {
                if let Some(ty) = BuiltInType::with_type(&ty) {
                    ty.to_c()
                } else {
                    "void*".to_string()
                }
            }
        }
    }

    pub fn contains_ints(&self) -> bool {
        if let ReturnType::Type(_, ty) = &self.func.sig.output {
            if let Some(ty) = BuiltInType::with_type(&ty) {
                if ty.needs_include_int_header() {
                    return true;
                }
            }
        }

        for param in &self.func.sig.inputs {
            if let FnArg::Typed(pat_ty) = param {
                if let Some(ty) = BuiltInType::with_type(&pat_ty.ty) {
                    if ty.needs_include_int_header() {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl ParsedExternFn {
    pub fn link_name(&self, host_type: Option<&Ident>) -> String {
        let host_type = host_type
            .map(|h| format!("${}", h.to_string()))
            .unwrap_or("".to_string());

        format!(
            "{}{}${}",
            SWIFT_BRIDGE_PREFIX,
            host_type,
            self.func.sig.ident.to_string()
        )
    }
}

impl Deref for ParsedExternFn {
    type Target = ForeignItemFn;

    fn deref(&self) -> &Self::Target {
        &self.func
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::SwiftBridgeModuleAndErrors;
    use crate::test_utils::{assert_tokens_contain, assert_tokens_eq};
    use crate::SwiftBridgeModule;
    use proc_macro2::Span;

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
        let methods = &module.extern_rust[0].types[0].methods;
        assert_eq!(methods.len(), 6);

        for method in methods {
            assert_tokens_contain(
                &method
                    .func
                    .to_rust_param_names_and_types(Some(&Ident::new("Foo", Span::call_site()))),
                &quote! { this },
            );
        }
    }

    /// Verify that when generating rust call args we do not include the receiver.
    #[test]
    fn does_not_include_self_in_rust_call_args() {
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
        let methods = &module.extern_rust[0].types[0].methods;
        assert_eq!(methods.len(), 6);

        for method in methods {
            let rust_call_args = &method.func.to_rust_call_args();
            assert_eq!(
                rust_call_args.to_string(),
                "",
                "{:#?}",
                method.func.to_token_stream()
            );
        }
    }

    /// Verify that we convert &[T] -> swift_bridge::RustSlice<T>
    #[test]
    fn converts_slice_to_rust_slice() {
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
        let function = &module.extern_rust[0].freestanding_fns[0];

        assert_tokens_eq(
            &function.to_extern_rust_function_tokens(None, None),
            &expected_fn,
        );
    }

    #[test]
    fn todo() {
        todo!(
            r#"
Refactor this file
Make the wrapping of the return type pass through the samw machinery.. so that we always wrap
slices.
        "#
        )
    }

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        let module_and_errors: SwiftBridgeModuleAndErrors = syn::parse2(tokens).unwrap();
        module_and_errors.module
    }
}
