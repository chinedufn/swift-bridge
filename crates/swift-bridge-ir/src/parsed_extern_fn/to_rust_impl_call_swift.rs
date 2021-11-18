use crate::build_in_types::BuiltInType;
use crate::parsed_extern_fn::ParsedExternFn;
use proc_macro2::TokenStream;
use quote::quote;

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
        let params = &sig.inputs;
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
}
