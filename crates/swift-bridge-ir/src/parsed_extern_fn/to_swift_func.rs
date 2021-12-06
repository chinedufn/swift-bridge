use crate::built_in_types::{pat_type_pat_is_self, BuiltInType, ForeignBridgedType, SharedType};
use crate::parse::TypeDeclarations;
use crate::parsed_extern_fn::ParsedExternFn;
use quote::ToTokens;
use std::ops::Deref;
use syn::{FnArg, ReturnType, Type};

impl ParsedExternFn {
    pub fn to_swift_param_names_and_types(
        &self,
        include_receiver_if_present: bool,
        types: &TypeDeclarations,
    ) -> String {
        let mut params: Vec<String> = vec![];

        for arg in &self.func.sig.inputs {
            let param = match arg {
                FnArg::Receiver(_receiver) => {
                    if include_receiver_if_present {
                        params.push(format!("_ this: UnsafeMutableRawPointer"));
                    }

                    continue;
                }
                FnArg::Typed(pat_ty) => {
                    if pat_type_pat_is_self(pat_ty) {
                        if include_receiver_if_present {
                            params.push(format!("_ this: UnsafeMutableRawPointer"));
                        }

                        continue;
                    }

                    let arg_name = pat_ty.pat.to_token_stream().to_string();

                    let ty = if let Some(built_in) = BuiltInType::new_with_type(&pat_ty.ty, types) {
                        built_in.to_swift_type(false)
                    } else {
                        let bridged_type = types.get_with_pat_type(&pat_ty).unwrap();

                        if self.host_lang.is_rust() {
                            match bridged_type {
                                ForeignBridgedType::Shared(SharedType::Struct(shared)) => {
                                    shared.swift_name_string()
                                }
                                ForeignBridgedType::Opaque(opaque) => opaque.ty.ident.to_string(),
                            }
                        } else {
                            match bridged_type {
                                ForeignBridgedType::Shared(SharedType::Struct(_shared)) => {
                                    todo!("Add a codegen test that hits this code path")
                                }
                                ForeignBridgedType::Opaque(opaque) => {
                                    if opaque.host_lang.is_rust() {
                                        "UnsafeMutableRawPointer".to_string()
                                    } else {
                                        "__private__PointerToSwiftType".to_string()
                                    }
                                }
                            }
                        }
                    };

                    format!("{}: {}", arg_name, ty)
                }
            };

            params.push(format!("_ {}", param))
        }

        params.join(", ")
    }

    // fn foo (&self, arg1: u8, arg2: u32)
    //  might become (depending on whether we're including the receiver and/or the var name)
    //  - arg1, arg2
    //  - ptr, arg1, arg2
    //  - arg1: arg1, arg2: arg2
    //  - self: ptr, arg1: arg1, arg2: arg2
    pub fn to_swift_call_args(
        &self,
        include_receiver_if_present: bool,
        include_var_name: bool,
        types: &TypeDeclarations,
    ) -> String {
        let mut args = vec![];
        let inputs = &self.func.sig.inputs;
        for arg in inputs {
            match arg {
                FnArg::Receiver(receiver) => {
                    if include_receiver_if_present {
                        let arg = if receiver.reference.is_some() {
                            "ptr"
                        } else {
                            "{isOwned = false; return ptr;}()"
                        };

                        args.push(arg.to_string());
                    }
                }
                FnArg::Typed(pat_ty) => {
                    let is_reference = match pat_ty.ty.deref() {
                        Type::Reference(_) => true,
                        _ => false,
                    };

                    if pat_type_pat_is_self(pat_ty) {
                        if include_receiver_if_present {
                            let arg = if is_reference {
                                "ptr"
                            } else {
                                "{isOwned = false; return ptr;}()"
                            };

                            args.push(arg.to_string());
                        }

                        continue;
                    }

                    let pat = &pat_ty.pat;
                    let arg = pat.to_token_stream().to_string();
                    let arg_name = arg.clone();

                    let arg = if let Some(built_in) = BuiltInType::new_with_type(&pat_ty.ty, types)
                    {
                        built_in.convert_swift_expression_to_ffi_compatible(&arg, self.host_lang)
                    } else {
                        if is_reference {
                            format!("{}.ptr", arg)
                        } else {
                            let bridged_type = types.get_with_pat_type(&pat_ty).unwrap();

                            if self.host_lang.is_rust() {
                                match bridged_type {
                                    ForeignBridgedType::Shared(_) => arg,
                                    ForeignBridgedType::Opaque(opaque) => {
                                        if opaque.host_lang.is_rust() {
                                            format!(
                                                "{{{arg}.isOwned = false; return {arg}.ptr;}}()",
                                                arg = arg
                                            )
                                        } else {
                                            // TODO: passUnretained if we're taking the argument by
                                            //  reference. passRetained if owned
                                            format!(
                                                "Unmanaged.passRetained({arg}).toOpaque()",
                                                arg = arg
                                            )
                                        }
                                    }
                                }
                            } else {
                                match bridged_type {
                                    ForeignBridgedType::Shared(_) => arg,
                                    ForeignBridgedType::Opaque(opaque) => {
                                        let ty = &opaque.ty.ident;

                                        if opaque.host_lang.is_rust() {
                                            format!(
                                                "{ty}(ptr: {arg}, isOwned: true)",
                                                ty = ty.to_string(),
                                                arg = arg
                                            )
                                        } else {
                                            format!(
                                                "Unmanaged<{ty}>.fromOpaque({arg}.ptr).takeRetainedValue()",
                                                ty = ty.to_string(),
                                                arg = arg
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    };

                    let arg = if include_var_name {
                        format!("{}: {}", arg_name, arg)
                    } else {
                        arg
                    };

                    args.push(arg);
                }
            };
        }

        args.join(", ")
    }

    pub fn to_swift_return_type(
        &self,
        must_be_c_compatible: bool,
        types: &TypeDeclarations,
    ) -> String {
        match &self.func.sig.output {
            ReturnType::Default => "".to_string(),
            ReturnType::Type(_, ty) => {
                if let Some(built_in) = BuiltInType::new_with_type(&ty, types) {
                    format!(" -> {}", built_in.to_swift_type(must_be_c_compatible))
                } else {
                    if self.host_lang.is_swift() {
                        format!(" -> UnsafeMutableRawPointer")
                    } else {
                        let ty = match ty.deref() {
                            Type::Reference(reference) => &reference.elem,
                            _ => ty,
                        };
                        let ty = ty.to_token_stream().to_string();

                        let ty = match types.get(&ty).unwrap() {
                            ForeignBridgedType::Shared(SharedType::Struct(shared)) => {
                                shared.swift_name_string()
                            }
                            ForeignBridgedType::Opaque(_) => ty,
                        };

                        format!(" -> {}", ty)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::SwiftBridgeModuleAndErrors;
    use crate::SwiftBridgeModule;
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Verify that if we are returning a declared type (non built-in) we return it as a Swift class
    /// instance.
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
        let functions = &module.functions;
        assert_eq!(functions.len(), 3);

        for idx in 0..3 {
            assert_eq!(
                functions[idx].to_swift_return_type(false, &module.types),
                " -> Foo"
            );
        }
    }

    /// Verify that we ignore self when generating Swift function params.
    #[test]
    fn excludes_self_from_params() {
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
            assert_eq!(
                method.to_swift_param_names_and_types(false, &module.types),
                ""
            );
        }
    }

    /// Verify that we always use the corresponding class name for an argument of a custom type.
    #[test]
    fn strips_references_from_params_with_declared_type() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type Foo;
                    fn make1 (other: Foo);
                    fn make2 (other: &Foo);
                    fn make3 (other: &mut Foo);
                }
            }
        };
        let module = parse_ok(tokens);
        let functions = &module.functions;
        assert_eq!(functions.len(), 3);

        for idx in 0..3 {
            assert_eq!(
                functions[idx].to_swift_param_names_and_types(false, &module.types),
                "_ other: Foo"
            );
        }
    }

    /// Verify that we use the `.ptr` field on a class instance when calling a Rust function from
    /// Swift.
    #[test]
    fn call_args_uses_pointer_from_class_instances() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type Foo;
                    fn make2 (other: &Foo);
                    fn make3 (other: &mut Foo);
                }
            }
        };
        let module = parse_ok(tokens);
        let functions = &module.functions;
        assert_eq!(functions.len(), 2);

        for idx in 0..2 {
            assert_eq!(
                functions[idx].to_swift_call_args(true, false, &module.types),
                "other.ptr"
            );
        }
    }

    /// Verify that if we pass an owned value to Rust the class instance is marked as no longer
    /// owned, since Rust now owns it.
    #[test]
    fn call_args_marks_instance_no_longer_owned_if_passed_owned_to_rust() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    type Foo;
                    fn make1(self);
                    fn make2(self: Foo);
                    fn make3(other: Foo);
                }
            }
        };
        let module = parse_ok(tokens);
        let functions = &module.functions;

        assert_eq!(
            functions[0].to_swift_call_args(true, false, &module.types),
            "{isOwned = false; return ptr;}()"
        );

        assert_eq!(
            functions[1].to_swift_call_args(true, false, &module.types),
            "{isOwned = false; return ptr;}()"
        );

        assert_eq!(
            functions[2].to_swift_call_args(true, false, &module.types),
            "{other.isOwned = false; return other.ptr;}()"
        );
    }

    /// Verify that we are calling .convert_swift_expression_to_ffi_compatible() on Swift -> FFI
    /// arguments.
    #[test]
    fn converts_unsafe_buffer_pointer_to_ffi_slice() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    fn some_function (someArg: &[u8]);
                }
            }
        };
        let module = parse_ok(tokens);
        let functions = &module.functions;

        assert_eq!(
            functions[0].to_swift_call_args(false, false, &module.types),
            "someArg.toFfiSlice()"
        );
    }

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        let module_and_errors: SwiftBridgeModuleAndErrors = syn::parse2(tokens).unwrap();
        module_and_errors.module
    }
}
