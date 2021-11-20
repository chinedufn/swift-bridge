use crate::built_in_types::BuiltInType;
use crate::parsed_extern_fn::ParsedExternFn;
use crate::pat_type_pat_is_self;
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;
use syn::{FnArg, PatType, Type, TypeReference};

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
///     // We're generating this function.
///     pub fn new () -> Foo {
///         Foo(unsafe{ __swift_bridge__Foo_new() })
///     }
/// }
/// extern "C" {
///     #[link_name = "__swift_bridge__$Foo$new"]
///     fn __swift_bridge__Foo_new() -> *mut c_void;
/// }
/// ```
impl ParsedExternFn {
    pub fn to_impl_fn_calls_swift(&self) -> TokenStream {
        let sig = &self.func.sig;
        let fn_name = &sig.ident;
        let ty_name = &self.associated_type.as_ref().unwrap().ident;

        let ret = &sig.output;
        let params = self.params_without_self_type_removd();
        let call_args = self.to_rust_call_args();
        let linked_fn_name = self.extern_swift_linked_fn_new();

        let mut inner = quote! {
            unsafe { #linked_fn_name(#call_args) }
        };

        if let Some(built_in) = BuiltInType::with_return_type(ret) {
            match built_in {
                BuiltInType::RefSlice(_) => {
                    inner = quote! {
                        #inner.as_slice()
                    };
                }
                _ => {}
            }
        } else {
            inner = quote! {
                #ty_name ( #inner )
            };
        }

        quote! {
            pub fn #fn_name(#params) #ret {
                #inner
            }
        }
    }

    // All of the params but with explicit types removed from `self`.
    //
    // `self: Foo` becomes `self`,
    // `self: &Foo` -> `&self`,
    // `self: &mut Foo` -> `&mut self`
    fn params_without_self_type_removd(&self) -> TokenStream {
        let params = self
            .sig
            .inputs
            .iter()
            .map(|arg| {
                if let Some(reference) = pat_ty_type_reference_if_arg_self(arg) {
                    let ref_token = reference.and_token;
                    let maybe_mut = reference.mutability;

                    quote! {
                        #ref_token #maybe_mut self
                    }
                } else {
                    quote! {
                        #arg
                    }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #(#params),*
        }
    }
}

// self: &Foo would return &Foo
// _foo: &Foo would not return &Foo
fn pat_ty_type_reference_if_arg_self(fn_arg: &FnArg) -> Option<&TypeReference> {
    match fn_arg {
        FnArg::Typed(pat_ty) if pat_type_pat_is_self(pat_ty) => {
            if let Some(reference) = pat_ty_type_reference(pat_ty) {
                Some(reference)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn pat_ty_type_reference(pat_ty: &PatType) -> Option<&TypeReference> {
    match pat_ty.ty.deref() {
        Type::Reference(reference) => Some(reference),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_tokens_eq, parse_ok};

    /// Verify that we generate a Rust associated function that calls a Swift static class method.
    #[test]
    fn static_class_method() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn message (val: u8);
                }
            }
        };
        let expected = quote! {
            pub fn message (val: u8) {
                unsafe { __swift_bridge__Foo_message(val) }
            }
        };

        assert_impl_fn_tokens_eq(start, &expected);
    }

    /// Verify that we generate a Rust associated function for a Swift class init function.
    #[test]
    fn class_initializer() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn new () -> Foo;
                }
            }
        };
        let expected = quote! {
            pub fn new () -> Foo {
                Foo( unsafe { __swift_bridge__Foo_new() } )
            }
        };

        assert_impl_fn_tokens_eq(start, &expected);
    }

    /// Verify that we convert RustSlice<T> -> &[T]
    #[test]
    fn converts_slice() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    type Foo;

                    fn as_slice (&self) -> &[u8];
                }
            }
        };
        let expected = quote! {
            pub fn as_slice (&self) -> &[u8] {
                unsafe { __swift_bridge__Foo_as_slice(self.0) }.as_slice()
            }
        };

        assert_impl_fn_tokens_eq(start, &expected);
    }

    // impl Foo {
    //    // We're testing to make sure that we generated this function or method properly.
    //    fn some_function() {
    //        ...
    //    }
    // }
    fn assert_impl_fn_tokens_eq(module: TokenStream, expected_impl_fn_tokens: &TokenStream) {
        let module = parse_ok(module);
        let tokens = module.functions[0].to_impl_fn_calls_swift();
        assert_tokens_eq(&tokens, &expected_impl_fn_tokens);
    }
}
