use crate::build_in_types::BuiltInType;
use crate::parsed_extern_fn::ParsedExternFn;
use quote::ToTokens;
use syn::{FnArg, ReturnType};

impl ParsedExternFn {
    pub fn to_swift_param_names_and_types(&self) -> String {
        let mut params: Vec<String> = vec![];

        for arg in &self.func.sig.inputs {
            match arg {
                FnArg::Receiver(receiver) => {
                    // FIXME: Change tests to not all use SomeType so that this fails...
                    // Needs to be based on  receiver.reference and receiver.mutability..
                    // let this = quote! { this: swift_bridge::OwnedPtrToRust<super::SomeType> };
                    // params.push(this);
                }
                FnArg::Typed(pat_ty) => {
                    let arg_name = pat_ty.pat.to_token_stream().to_string();

                    if let Some(built_in) = BuiltInType::with_type(&pat_ty.ty) {
                        params.push(format!("{}: {}", arg_name, built_in.to_swift()));
                    } else {
                        todo!("Add tests for generating functions for unsupported types")
                    };
                }
            };
        }

        params.join(", ")
    }

    // fn foo (&self, arg1: u8, arg2: u32)
    //  becomes..
    // ptr, arg1, arg2
    pub fn to_swift_call_args(&self) -> String {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(_receiver) => args.push("ptr".to_string()),
                FnArg::Typed(pat_ty) => {
                    let pat = &pat_ty.pat;
                    args.push(pat.to_token_stream().to_string());
                }
            };
        }

        args.join(", ")
    }

    pub fn to_swift_return(&self) -> String {
        match &self.func.sig.output {
            ReturnType::Default => "".to_string(),
            ReturnType::Type(_, ty) => {
                if let Some(built_in) = BuiltInType::with_type(&ty) {
                    format!(" -> {}", built_in.to_swift())
                } else {
                    format!("*mut c_void")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::SwiftBridgeModuleAndErrors;
    use crate::SwiftBridgeModule;
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Verify that if we are returning a declared type (non built-in) we return it as a pointer.
    #[test]
    fn return_declared_type() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type Foo;
                    fn make1 () -> Foo;
                    fn make2 () -> &Foo;
                    fn make3 () -> &mut Foo;
                }
            }
        };
        let module = parse_ok(tokens);
        let functions = &module.extern_rust[0].freestanding_fns;
        assert_eq!(functions.len(), 3);

        for idx in 0..3 {
            assert_eq!(functions[idx].to_swift_return(), "*mut c_void");
        }
    }

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        let module_and_errors: SwiftBridgeModuleAndErrors = syn::parse2(tokens).unwrap();
        module_and_errors.module
    }
}
