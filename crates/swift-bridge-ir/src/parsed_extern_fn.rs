use crate::built_in_types::BuiltInType;
use crate::parse::{HostLang, TypeDeclarations};
use crate::{pat_type_pat_is_self, BridgedType, SharedType, SWIFT_BRIDGE_PREFIX};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{FnArg, ForeignItemFn, Lifetime, Path, ReturnType, Token, Type};

mod to_extern_c_fn;
mod to_extern_c_param_names_and_types;
mod to_rust_impl_call_swift;
mod to_swift_func;

/// A method or associated function associated with a type.
///
/// fn bar (&self);
/// fn buzz (self: &Foo) -> u8;
///
/// #\[swift_bridge(init)\]
/// fn new () -> Foo;
///
/// ... etc
pub(crate) struct ParsedExternFn {
    pub func: ForeignItemFn,
    pub associated_type: Option<BridgedType>,
    pub is_initializer: bool,
    pub host_lang: HostLang,
    pub swift_name_override: Option<syn::LitStr>,
}

impl ParsedExternFn {
    pub fn is_method(&self) -> bool {
        self.func.sig.receiver().is_some()
    }

    pub fn self_reference(&self) -> Option<(Token![&], Option<Lifetime>)> {
        match self.func.sig.receiver()? {
            FnArg::Receiver(receiver) => receiver.reference.clone(),
            FnArg::Typed(pat_ty) => match pat_ty.ty.deref() {
                Type::Reference(type_ref) => Some((type_ref.and_token, type_ref.lifetime.clone())),
                _ => None,
            },
        }
    }

    pub fn self_mutability(&self) -> Option<Token![mut]> {
        match self.func.sig.receiver()? {
            FnArg::Receiver(receiver) => receiver.mutability,
            FnArg::Typed(pat_ty) => match pat_ty.ty.deref() {
                Type::Reference(type_ref) => type_ref.mutability,
                _ => None,
            },
        }
    }

    pub(crate) fn rust_return_type(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let sig = &self.func.sig;

        let ret = match &sig.output {
            ReturnType::Default => {
                quote! {}
            }
            ReturnType::Type(arrow, ty) => {
                if let Some(built_in) = BuiltInType::new_with_type(&ty) {
                    let ty = built_in.to_ffi_compatible_rust_type(swift_bridge_path);
                    quote! {#arrow #ty}
                } else {
                    if self.host_lang.is_rust() {
                        let (is_const_ptr, ty) = match ty.deref() {
                            Type::Reference(reference) => {
                                (reference.mutability.is_none(), reference.elem.deref())
                            }
                            _ => (false, ty.deref()),
                        };

                        let ty_string = ty.to_token_stream().to_string();
                        match types.get(&ty_string).unwrap() {
                            BridgedType::Shared(SharedType::Struct(shared_struct)) => {
                                let name = &shared_struct.name;
                                quote! { #arrow #name }
                            }
                            BridgedType::Opaque(opaque) => {
                                if opaque.host_lang.is_rust() {
                                    let ptr = if is_const_ptr {
                                        quote! { *const }
                                    } else {
                                        quote! { *mut }
                                    };

                                    quote_spanned! {ty.span()=> #arrow #ptr super::#ty }
                                } else {
                                    quote! { #arrow #ty }
                                }
                            }
                        }
                    } else {
                        quote_spanned! {ty.span()=> #arrow *mut std::ffi::c_void }
                    }
                }
            }
        };

        ret
    }

    pub fn extern_swift_linked_fn_new(&self) -> Ident {
        let sig = &self.func.sig;

        let prefix = if let Some(associated_ty) = self.associated_type.as_ref() {
            match associated_ty {
                BridgedType::Shared(_) => {
                    todo!()
                }
                BridgedType::Opaque(associated_ty) => {
                    format!("{}_", associated_ty.ident)
                }
            }
        } else {
            "".to_string()
        };

        Ident::new(
            &format!("{}{}{}", SWIFT_BRIDGE_PREFIX, prefix, sig.ident),
            sig.ident.span(),
        )
    }
}

impl ParsedExternFn {
    // extern Rust:
    // fn foo (&self, arg1: u8, arg2: u32, &SomeType)
    //  becomes..
    // arg1, arg2, & unsafe { Box::from_raw(bar }
    //
    // extern Swift:
    // fn foo (&self, arg1: u8, arg2: u32, &SomeType)
    //  becomes..
    // self.0, arg1, arg2, & unsafe { Box::from_raw(bar }
    pub fn to_call_rust_args(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => {
                    if self.host_lang.is_swift() {
                        args.push(quote! {self.0});
                    }
                }
                FnArg::Typed(pat_ty) => {
                    if pat_type_pat_is_self(pat_ty) {
                        if self.host_lang.is_swift() {
                            args.push(quote! {self.0});
                        }

                        continue;
                    }

                    let pat = &pat_ty.pat;

                    let mut arg = quote! {#pat};

                    if let Some(built_in) = BuiltInType::new_with_type(&pat_ty.ty) {
                        if self.host_lang.is_rust() {
                            arg = built_in.convert_ffi_value_to_rust_value(
                                swift_bridge_path,
                                &arg,
                                false,
                            );
                        } else {
                            arg = built_in.convert_rust_value_to_ffi_compatible_value(
                                swift_bridge_path,
                                &arg,
                            );
                        };
                    } else {
                        if self.host_lang.is_rust() {
                            match types.get_with_pat_type(pat_ty).unwrap() {
                                BridgedType::Shared(SharedType::Struct(_)) => {}
                                BridgedType::Opaque(opaque) => {
                                    if opaque.host_lang.is_rust() {
                                        let (maybe_ref, maybe_mut) = match pat_ty.ty.deref() {
                                            Type::Reference(ty_ref) => {
                                                (Some(ty_ref.and_token), ty_ref.mutability)
                                            }
                                            _ => (None, None),
                                        };

                                        let dereferenced = if maybe_ref.is_some() {
                                            quote! { unsafe { #maybe_ref #maybe_mut * #arg } }
                                        } else {
                                            quote! { unsafe { *Box::from_raw(#arg) } }
                                        };

                                        arg = dereferenced;
                                    } else {
                                        let ty = &opaque.ty.ident;
                                        arg = quote! { #ty(#arg) };
                                    }
                                }
                            };
                        } else {
                            match types.get_with_pat_type(pat_ty).unwrap() {
                                BridgedType::Shared(SharedType::Struct(_)) => {
                                    todo!("Add a test that hits this code path")
                                }
                                BridgedType::Opaque(opaque) => {
                                    if opaque.host_lang.is_rust() {
                                        arg = quote! { Box::into_raw(Box::new(#arg)) };
                                    } else {
                                        arg = quote! { #arg };
                                    }
                                }
                            };
                        }
                    }

                    args.push(arg);
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
    pub fn to_c_header_params(&self, types: &TypeDeclarations) -> String {
        let mut params = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => params.push("void* self".to_string()),
                FnArg::Typed(pat_ty) => {
                    let pat = &pat_ty.pat;

                    if pat_type_pat_is_self(pat_ty) {
                        params.push("void* self".to_string());
                    } else {
                        let ty = if let Some(built_in) = BuiltInType::new_with_type(&pat_ty.ty) {
                            built_in.to_c()
                        } else {
                            let bridged_type = types.get_with_pat_type(&pat_ty).unwrap();

                            match bridged_type {
                                BridgedType::Shared(SharedType::Struct(shared_struct)) => {
                                    format!("struct {}", shared_struct.swift_name_string())
                                }
                                BridgedType::Opaque(_) => "void*".to_string(),
                            }
                        };

                        let arg_name = pat.to_token_stream().to_string();
                        params.push(format!("{} {}", ty, arg_name));
                    }
                }
            };
        }

        if params.len() == 0 {
            "void".to_string()
        } else {
            params.join(", ")
        }
    }

    pub fn to_c_header_return(&self, types: &TypeDeclarations) -> String {
        match &self.func.sig.output {
            ReturnType::Default => "void".to_string(),
            ReturnType::Type(_, ty) => {
                if let Some(ty) = BuiltInType::new_with_type(&ty) {
                    ty.to_c()
                } else {
                    let ty_string = match ty.deref() {
                        Type::Reference(reference) => reference.elem.to_token_stream().to_string(),
                        Type::Path(path) => path.path.to_token_stream().to_string(),
                        _ => todo!(),
                    };

                    match types.get(&ty_string).unwrap() {
                        BridgedType::Shared(SharedType::Struct(shared_struct)) => {
                            format!("struct {}", shared_struct.swift_name_string())
                        }
                        BridgedType::Opaque(opaque) => {
                            if opaque.host_lang.is_rust() {
                                "void*".to_string()
                            } else {
                                "struct __private__PointerToSwiftType".to_string()
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn c_includes(&self) -> Option<Vec<&'static str>> {
        let mut includes = vec![];

        if let ReturnType::Type(_, ty) = &self.func.sig.output {
            if let Some(ty) = BuiltInType::new_with_type(&ty) {
                if let Some(include) = ty.c_include() {
                    includes.push(include);
                }
            }
        }

        for param in &self.func.sig.inputs {
            if let FnArg::Typed(pat_ty) = param {
                if let Some(ty) = BuiltInType::new_with_type(&pat_ty.ty) {
                    if let Some(include) = ty.c_include() {
                        includes.push(include);
                    }
                }
            }
        }

        if includes.len() > 0 {
            Some(includes)
        } else {
            None
        }
    }
}

impl ParsedExternFn {
    pub fn link_name(&self) -> String {
        let host_type = self
            .associated_type
            .as_ref()
            .map(|h| {
                match h {
                    BridgedType::Shared(_) => {
                        //
                        todo!()
                    }
                    BridgedType::Opaque(h) => {
                        format!("${}", h.ident.to_string())
                    }
                }
            })
            .unwrap_or("".to_string());

        format!(
            "{}{}${}",
            SWIFT_BRIDGE_PREFIX,
            host_type,
            self.func.sig.ident.to_string()
        )
    }

    pub fn prefixed_fn_name(&self) -> Ident {
        let host_type_prefix = self
            .associated_type
            .as_ref()
            .map(|h| {
                match h {
                    BridgedType::Shared(_) => {
                        //
                        todo!()
                    }
                    BridgedType::Opaque(h) => {
                        format!("{}_", h.ident.to_token_stream().to_string())
                    }
                }
            })
            .unwrap_or_default();
        let fn_name = &self.func.sig.ident;
        let prefixed_fn_name = Ident::new(
            &format!(
                "{}{}{}",
                SWIFT_BRIDGE_PREFIX,
                host_type_prefix,
                fn_name.to_string()
            ),
            fn_name.span(),
        );

        prefixed_fn_name
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
    use crate::test_utils::{assert_tokens_contain, assert_tokens_eq, parse_ok};

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
        let methods = &module.functions;
        assert_eq!(methods.len(), 6);

        for method in methods {
            let rust_call_args =
                &method.to_call_rust_args(&module.swift_bridge_path, &module.types);
            assert_eq!(
                rust_call_args.to_string(),
                "",
                "\n Function Tokens:\n{:#?}",
                method.func.to_token_stream()
            );
        }
    }

    /// Verify that when we get an owned opaque type as an argument we unbox it.
    #[test]
    fn unboxes_owned_opaque_type() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type Foo;
                    fn some_function (arg: Foo);
                }
            }
        };
        let module = parse_ok(tokens);
        assert_tokens_eq(
            &module.functions[0].to_call_rust_args(&module.swift_bridge_path, &module.types),
            &quote! {unsafe { *Box::from_raw(arg) }},
        );
    }

    /// Verify that if a foreign type is marked as enabled we allow taking owned foreign type args.
    #[test]
    fn allow_foreign_type_arg_if_type_marked_enabled_or_enabled_unchecked() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(owned_arg = "enabled")]
                    type Foo;
                    #[swift_bridge(owned_arg = "enabled_unchecked")]
                    type Bar;

                    fn a (arg: Foo);
                    fn b (arg: Bar);
                }
            }
        };
        let module = parse_ok(tokens);
        assert_eq!(module.functions.len(), 2);
    }

    /// Verify that we properly take and return String arguments
    #[test]
    fn extern_rust_strings() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn some_function (arg1: String) -> String;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function (
                arg1: *mut swift_bridge::string::RustString
            ) -> *mut swift_bridge::string::RustString {
                swift_bridge::string::RustString(super::some_function(
                    unsafe { Box::from_raw(arg1).0 }
                )).box_into_raw()
            }
        };

        assert_tokens_contain(&parse_ok(tokens).to_token_stream(), &expected);
    }

    /// Verify that we properly take and return &str arguments
    #[test]
    fn extern_rust_strs() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn some_function (arg: &str) -> &str;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function (
                arg: swift_bridge::string::RustStr
            ) -> swift_bridge::string::RustStr {
                swift_bridge::string::RustStr::from_str(
                    super::some_function(arg.to_str())
                )
            }
        };

        assert_tokens_contain(&parse_ok(tokens).to_token_stream(), &expected);
    }
}
