use crate::bridged_type::{pat_type_pat_is_self, BridgedType, TypePosition};
use crate::parse::{HostLang, SharedTypeDeclaration, TypeDeclaration, TypeDeclarations};
use crate::SWIFT_BRIDGE_PREFIX;
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
    pub associated_type: Option<TypeDeclaration>,
    pub is_initializer: bool,
    pub host_lang: HostLang,
    pub rust_name_override: Option<syn::LitStr>,
    pub swift_name_override: Option<syn::LitStr>,
    /// If true, we call `.into()` on the expression that the function returns before returning it.
    ///
    /// ```no_run,ignore
    /// // Declaration
    /// fn some_function() -> SomeType;
    ///
    /// // Approximate generated Code
    /// extern "C" fn some_function() -> SomeType {
    ///     super::some_function().into()
    /// }
    /// ```
    pub into_return_type: bool,
    /// Call `.into()` before passing this argument to the function that handles it.
    ///
    /// ```no_run,ignore
    /// // Declaration
    /// #[swift_bridge(args_into = (some_arg, another_arg)]
    /// fn some_function(some_arg: u8, another_arg: MyStruct);
    ///
    /// // Approximate generated code
    /// extern "C" fn some_function(some_arg: u8, another_arg: MyStruct) {
    ///     super::some_function(some_arg, another_arg.into())
    /// }
    /// ```
    pub args_into: Option<Vec<Ident>>,
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
        func_host_lang: HostLang,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let sig = &self.func.sig;

        if let Some(ret) = BridgedType::new_with_return_type(&sig.output, types) {
            let ty = ret.to_ffi_compatible_rust_type(func_host_lang, swift_bridge_path);
            if ty.to_string() == "()" {
                quote! {}
            } else {
                quote! { -> #ty }
            }
        } else {
            todo!("Push to ParseErrors")
        }
    }

    pub fn extern_swift_linked_fn_new(&self) -> Ident {
        let sig = &self.func.sig;

        let prefix = if let Some(associated_ty) = self.associated_type.as_ref() {
            match associated_ty {
                TypeDeclaration::Shared(_) => {
                    todo!()
                }
                TypeDeclaration::Opaque(associated_ty) => {
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

    pub fn args_into_contains_arg(&self, arg: &FnArg) -> bool {
        if self.args_into.is_none() {
            return false;
        }
        let args_into = self.args_into.as_ref().unwrap();

        match arg {
            FnArg::Receiver(_) => false,
            FnArg::Typed(arg) => {
                let arg_string = arg.pat.to_token_stream().to_string();

                args_into
                    .iter()
                    .any(|arg_name| &arg_name.to_string() == &arg_string)
            }
        }
    }
}

impl ParsedExternFn {
    // extern Rust:
    // fn foo (&self, arg1: u8, arg2: u32, bar: &SomeType)
    //  becomes..
    // arg1, arg2, & unsafe { Box::from_raw(bar) }
    //
    // extern Swift:
    // fn foo (&self, arg1: u8, arg2: u32, &SomeType)
    //  becomes..
    // self.0, arg1, arg2, & unsafe { Box::from_raw(bar) }
    pub fn to_call_rust_args(
        &self,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for fn_arg in inputs {
            match fn_arg {
                FnArg::Receiver(_receiver) => {
                    if self.host_lang.is_swift() {
                        args.push(quote! {#swift_bridge_path::PointerToSwiftType(self.0)});
                    }
                }
                FnArg::Typed(pat_ty) => {
                    if pat_type_pat_is_self(pat_ty) {
                        if self.host_lang.is_swift() {
                            args.push(quote! {#swift_bridge_path::PointerToSwiftType(self.0)});
                        }

                        continue;
                    }

                    let pat = &pat_ty.pat;

                    let mut arg = quote! {#pat};

                    if let Some(built_in) = BridgedType::new_with_type(&pat_ty.ty, types) {
                        if self.host_lang.is_rust() {
                            arg = built_in.convert_ffi_value_to_rust_value(
                                &arg,
                                TypePosition::FnArg(self.host_lang),
                                pat_ty.ty.span(),
                            );

                            if self.args_into_contains_arg(fn_arg) {
                                arg = quote_spanned! {pat_ty.span()=>
                                    #arg.into()
                                };
                            }
                        } else {
                            arg = built_in.convert_rust_value_to_ffi_compatible_value(
                                swift_bridge_path,
                                &arg,
                            );
                        };
                    } else {
                        todo!("Push to ParsedErrors")
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
                        let built_in = BridgedType::new_with_type(&pat_ty.ty, types).unwrap();
                        let ty = built_in.to_c();

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
                if let Some(ty) = BridgedType::new_with_type(&ty, types) {
                    ty.to_c()
                } else {
                    let ty_string = match ty.deref() {
                        Type::Reference(reference) => reference.elem.to_token_stream().to_string(),
                        Type::Path(path) => path.path.to_token_stream().to_string(),
                        _ => todo!(),
                    };

                    match types.get(&ty_string).unwrap() {
                        TypeDeclaration::Shared(SharedTypeDeclaration::Struct(shared_struct)) => {
                            format!("struct {}", shared_struct.swift_name_string())
                        }
                        TypeDeclaration::Opaque(opaque) => {
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

    pub fn c_includes(&self, types: &TypeDeclarations) -> Option<Vec<&'static str>> {
        let mut includes = vec![];

        if let ReturnType::Type(_, ty) = &self.func.sig.output {
            if let Some(ty) = BridgedType::new_with_type(&ty, types) {
                if let Some(include) = ty.c_include() {
                    includes.push(include);
                }
            }
        }

        for param in &self.func.sig.inputs {
            if let FnArg::Typed(pat_ty) = param {
                if let Some(ty) = BridgedType::new_with_type(&pat_ty.ty, types) {
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
                    TypeDeclaration::Shared(_) => {
                        //
                        todo!()
                    }
                    TypeDeclaration::Opaque(h) => {
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
                    TypeDeclaration::Shared(_) => {
                        //
                        todo!()
                    }
                    TypeDeclaration::Opaque(h) => {
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
