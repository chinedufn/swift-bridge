use crate::build_in_types::BuiltInType;
use crate::{SelfRefMut, SWIFT_BRIDGE_PREFIX};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, ForeignItemFn, ReturnType};

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

        let params = self.to_rust_param_names_and_types();
        let call_args = self.to_rust_call_args();

        let call_fn = quote! {
            #fn_name ( #call_args )
        };

        let inner = if let Some(this) = this.as_ref() {
            if let Some(reference) = this.reference {
                let maybe_mut = &this.mutability;

                quote! {
                    let this = unsafe { #reference #maybe_mut *this.ptr };
                    this.#call_fn
                }
            } else {
                todo!()
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
                    if BuiltInType::with_type(&ty).is_some() {
                        quote! {
                            super:: #host_type_segment #call_fn
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

        let output = match &sig.output {
            ReturnType::Default => {
                quote! {}
            }
            ReturnType::Type(arrow, ty) => {
                if let Some(_supported) = BuiltInType::with_type(&ty) {
                    quote! {#arrow #ty}
                } else {
                    quote_spanned! {ty.span()=> -> *mut std::ffi::c_void }
                }
            }
        };

        quote! {
            #[no_mangle]
            #[export_name = #export_name]
            pub extern "C" fn #prefixed_fn_name ( #params ) #output {
                #inner
            }
        }
    }

    pub fn to_rust_param_names_and_types(&self) -> TokenStream {
        let mut params = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(receiver) => {
                    // FIXME: Change tests to not all use SomeType so that this fails...
                    // Needs to be based on  receiver.reference and receiver.mutability..
                    let this = quote! { this: swift_bridge::OwnedPtrToRust<super::SomeType> };
                    params.push(this);
                }
                FnArg::Typed(pat_ty) => {
                    params.push(quote! {#pat_ty});
                }
            };
        }

        quote! {
            #(#params),*
        }
    }

    // fn foo (&self, arg1: u8, arg2: u32)
    //  becomes..
    // arg1, arg2
    pub fn to_rust_call_args(&self) -> TokenStream {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => {}
                FnArg::Typed(pat_ty) => {
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

        if params.len() == 0 {
            "void".to_string()
        } else {
            params.join(", ")
        }
    }

    pub fn to_c_header_return(&self) -> &'static str {
        match &self.func.sig.output {
            ReturnType::Default => "void",
            ReturnType::Type(_, ty) => {
                if let Some(ty) = BuiltInType::with_type(&ty) {
                    ty.to_c()
                } else {
                    "void*"
                }
            }
        }
    }

    pub fn contains_ints(&self) -> bool {
        if let ReturnType::Type(_, ty) = &self.func.sig.output {
            if let Some(ty) = BuiltInType::with_type(&ty) {
                if ty.is_int() {
                    return true;
                }
            }
        }

        for param in &self.func.sig.inputs {
            if let FnArg::Typed(pat_ty) = param {
                if let Some(ty) = BuiltInType::with_type(&pat_ty.ty) {
                    if ty.is_int() {
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
