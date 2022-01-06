use crate::bridged_type::BridgedType;
use crate::errors::{ParseError, ParseErrors};
use crate::parse::parse_extern_mod::function_attributes::{FunctionAttr, FunctionAttributes};
use crate::parse::parse_extern_mod::opaque_type_attributes::{
    OpaqueTypeAttr, OpaqueTypeAttributes,
};
use crate::parse::type_declarations::{
    OpaqueForeignTypeDeclaration, TypeDeclaration, TypeDeclarations,
};
use crate::parse::HostLang;
use crate::ParsedExternFn;
use quote::ToTokens;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Deref;
use syn::{FnArg, ForeignItem, ForeignItemFn, ItemForeignMod, Pat, ReturnType, Type};

mod function_attributes;
mod opaque_type_attributes;

pub(super) struct ForeignModParser<'a> {
    pub errors: &'a mut ParseErrors,
    /// All of the type declarations across all of the extern "..." foreign modules in the
    /// `mod` module that this foreign module is in.
    pub type_declarations: &'a mut TypeDeclarations,
    pub functions: &'a mut Vec<ParsedExternFn>,
    pub unresolved_types: &'a mut Vec<Type>,
}

impl<'a> ForeignModParser<'a> {
    pub fn parse(mut self, mut foreign_mod: ItemForeignMod) -> Result<(), syn::Error> {
        if foreign_mod.abi.name.is_none() {
            self.errors.push(ParseError::AbiNameMissing {
                extern_token: foreign_mod.abi.extern_token,
            });
            return Ok(());
        }

        let abi_name = foreign_mod.abi.name.unwrap();

        let host_lang = match abi_name.value().as_str() {
            "Rust" => HostLang::Rust,
            "Swift" => HostLang::Swift,
            _ => {
                self.errors.push(ParseError::AbiNameInvalid { abi_name });
                return Ok(());
            }
        };

        foreign_mod.items.sort_by(|a, _b| {
            if matches!(a, ForeignItem::Type(_)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let mut local_type_declarations = HashMap::new();
        for foreign_mod_item in foreign_mod.items {
            match foreign_mod_item {
                ForeignItem::Type(foreign_ty) => {
                    let ty_name = foreign_ty.ident.to_string();

                    if let Some(_builtin) = BridgedType::with_str(
                        &foreign_ty.ident.to_string(),
                        &self.type_declarations,
                    ) {
                        self.errors.push(ParseError::DeclaredBuiltInType {
                            ty: foreign_ty.clone(),
                        });
                    }

                    let mut attributes = OpaqueTypeAttributes::default();

                    for attr in foreign_ty.attrs.iter() {
                        let attr: OpaqueTypeAttr = attr.parse_args()?;
                        attributes.store_attrib(attr);
                    }

                    let foreign_type = OpaqueForeignTypeDeclaration {
                        ty: foreign_ty.clone(),
                        host_lang,
                        already_declared: attributes.already_declared,
                    };
                    self.type_declarations.insert(
                        ty_name.clone(),
                        TypeDeclaration::Opaque(foreign_type.clone()),
                    );
                    local_type_declarations.insert(ty_name, foreign_type);
                }
                ForeignItem::Fn(func) => {
                    let mut attributes = FunctionAttributes::default();

                    for attr in func.attrs.iter() {
                        let attr: FunctionAttr = attr.parse_args()?;
                        attributes.store_attrib(attr);
                    }

                    for arg in func.sig.inputs.iter() {
                        if let FnArg::Typed(pat_ty) = arg {
                            let ty = &pat_ty.ty;
                            if BridgedType::new_with_type(&ty, &self.type_declarations).is_none() {
                                self.unresolved_types.push(ty.deref().clone());
                            }
                        }
                    }

                    let return_type = &func.sig.output;
                    if let ReturnType::Type(_, return_ty) = return_type {
                        if BridgedType::new_with_type(return_ty.deref(), &self.type_declarations)
                            .is_none()
                        {
                            self.unresolved_types.push(return_ty.deref().clone());
                        }
                    }

                    let first_input = func.sig.inputs.iter().next();

                    let associated_type = self.get_associated_type(
                        first_input,
                        func.clone(),
                        &attributes,
                        &mut local_type_declarations,
                    )?;

                    self.functions.push(ParsedExternFn {
                        func,
                        associated_type,
                        is_initializer: attributes.is_initializer,
                        host_lang,
                        rust_name_override: attributes.rust_name,
                        swift_name_override: attributes.swift_name,
                        into_return_type: attributes.into_return_type,
                    });
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn get_associated_type(
        &mut self,
        first: Option<&FnArg>,
        func: ForeignItemFn,
        attributes: &FunctionAttributes,
        local_type_declarations: &mut HashMap<String, OpaqueForeignTypeDeclaration>,
    ) -> syn::Result<Option<TypeDeclaration>> {
        let associated_type = match first {
            Some(FnArg::Receiver(recv)) => {
                if local_type_declarations.len() == 1 {
                    let ty = local_type_declarations.iter_mut().next().unwrap().1;
                    let associated_type = Some(TypeDeclaration::Opaque(ty.clone()));
                    associated_type
                } else {
                    self.errors.push(ParseError::AmbiguousSelf {
                        self_: recv.clone(),
                    });
                    return Ok(None);
                }
            }
            Some(FnArg::Typed(arg)) => match arg.pat.deref() {
                Pat::Ident(pat_ident) => {
                    if pat_ident.ident.to_string() == "self" {
                        let self_ty = match arg.ty.deref() {
                            Type::Path(ty_path) => ty_path.path.segments.to_token_stream(),
                            Type::Reference(type_ref) => type_ref.elem.deref().to_token_stream(),
                            _ => {
                                todo!("Add a test that hits this branch")
                            }
                        };

                        let self_ty_string = self_ty.to_string();
                        let ty = self.type_declarations.get(&self_ty_string).unwrap();
                        let associated_type = Some(ty.clone());
                        associated_type
                    } else {
                        let associated_type = self.get_associated_type(
                            None,
                            func.clone(),
                            attributes,
                            local_type_declarations,
                        )?;
                        associated_type
                    }
                }
                _ => {
                    todo!("Add test that hits this block")
                }
            },
            None => {
                let associated_type = if let Some(associated_to) = &attributes.associated_to {
                    let ty = self
                        .type_declarations
                        .get(&associated_to.to_string())
                        .unwrap();
                    Some(ty.clone())
                } else if attributes.is_initializer {
                    let ty_string = match &func.sig.output {
                        ReturnType::Default => {
                            todo!("Push error if initializer does not return a type")
                        }
                        ReturnType::Type(_, ty) => ty.deref().to_token_stream().to_string(),
                    };

                    let ty = self.type_declarations.get(&ty_string);

                    ty.map(|ty| ty.clone())
                } else {
                    None
                };

                associated_type
            }
        };

        Ok(associated_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::ParseError;
    use crate::test_utils::{parse_errors, parse_ok};
    use crate::SwiftBridgeModule;
    use quote::{quote, ToTokens};
    use syn::parse_quote;

    /// Verify that we can parse a SwiftBridgeModule from an empty module.
    #[test]
    fn parse_empty_module() {
        let tokens = quote! {
            mod foo { }
        };
        let module: SwiftBridgeModule = parse_quote!(#tokens);

        assert_eq!(module.name.to_string(), "foo");
    }

    /// Verify that we store an error if no abi name was provided.
    #[test]
    fn error_if_no_abi_name_provided_for_an_extern_block() {
        let tokens = quote! {
            mod foo {
                extern {}
            }
        };
        let errors = parse_errors(tokens);

        assert_eq!(errors.len(), 1);
        match errors[0] {
            ParseError::AbiNameMissing { .. } => {}
            _ => panic!(),
        }
    }

    /// Verify that we store an error if the abi name isn't Rust or Swift.
    #[test]
    fn error_if_invalid_abi_name() {
        let tokens = quote! {
            mod foo {
                extern "SomeAbi" {}
            }
        };
        let errors = parse_errors(tokens);

        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ParseError::AbiNameInvalid { abi_name } => {
                assert_eq!(abi_name.value(), "SomeAbi");
            }
            _ => panic!(),
        }
    }

    /// Verify that we can parse a Rust type declaration.
    #[test]
    fn rust_type_declaration() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(
            module.types.types()[0].unwrap_opaque().ident.to_string(),
            "Foo"
        );
    }

    /// Verify that we return an error if the declared type is a built in type.
    #[test]
    fn error_if_declared_built_in_type() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type u8;
                }
            }
        };

        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 1);
    }

    /// Verify that we can parse a Rust type's methods.
    /// We test all of the possible ways we can specify self.
    #[test]
    fn parses_rust_self_methods() {
        let tests = vec![
            quote! { fn bar (self); },
            quote! { fn bar (&self); },
            quote! { fn bar (&mut self); },
            quote! { fn bar (self: Foo); },
            quote! { fn bar (self: &Foo); },
            quote! { fn bar (self: &mut Foo); },
        ];

        for fn_definition in tests {
            let tokens = quote! {
                mod foo {
                    extern "Rust" {
                        type Foo;

                        #fn_definition
                    }
                }
            };

            let module = parse_ok(tokens);

            let ty = &module.types.types()[0].unwrap_opaque();
            assert_eq!(ty.ident.to_string(), "Foo");

            assert_eq!(
                module.functions.len(),
                1,
                "Failed not parse {} into an associated method.",
                quote! {#fn_definition}.to_string()
            );
        }
    }

    /// Verify that we can parse the into_return_type attribute from extern "Rust" blocks.
    #[test]
    fn parse_extern_rust_into_return_type_attribute() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(into_return_type)]
                    fn some_function () -> Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        assert!(module.functions[0].into_return_type);
    }

    /// Verify that we can parse an associated function.
    #[test]
    fn parse_associated_function() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn bar ();
                }
            }
        };

        let module = parse_ok(tokens);

        let ty = &module.types.types()[0].unwrap_opaque();
        assert_eq!(ty.ident.to_string(), "Foo");

        assert_eq!(module.functions.len(), 1,);
    }

    /// Verify that we can parse an associated function that has arguments.
    #[test]
    fn associated_function_with_args() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn bar (arg: u8);
                }
            }
        };

        let module = parse_ok(tokens);

        let ty = &module.types.types()[0].unwrap_opaque();
        assert_eq!(ty.ident.to_string(), "Foo");

        assert_eq!(module.functions.len(), 1,);
    }

    /// Verify that we can parse an init function.
    #[test]
    fn initializer() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn bar () -> Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        let func = &module.functions[0];
        assert!(func.is_initializer);
    }

    /// Verify that we can parse an init function that takes inputs.
    #[test]
    fn initializer_with_inputs() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    #[swift_bridge(init)]
                    fn bar (bazz: u8) -> Foo;
                }
            }
        };

        let module = parse_ok(tokens);

        let func = &module.functions[0];
        assert!(func.is_initializer);
    }

    /// Verify that we push an error if the initialize type is not defined.
    #[test]
    fn error_if_initialized_type_not_defined() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    #[swift_bridge(init)]
                    fn bar () -> Foo;
                }
            }
        };

        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 1);

        match &errors[0] {
            ParseError::UndeclaredType { ty } => {
                assert_eq!(ty.to_token_stream().to_string(), "Foo")
            }
            _ => panic!(),
        }
    }

    /// Verify that if a a type is defined in another foreign module block we can still use it.
    #[test]
    fn type_defined_in_another_foreign_module() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn foo () -> AnotherType;
                }

                extern "Rust" {
                    type AnotherType;
                }
            }
        };

        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 0,);
    }

    /// Verify that we can parse a freestanding Rust function declaration.
    #[test]
    fn rust_freestanding_function_no_args() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn bar () -> u8;
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.functions.len(), 1);
    }

    /// Verify that we can parse a freestanding Rust function declaration that has one arg.
    #[test]
    fn rust_freestanding_function_one_arg() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn bar (bazz: u32);
                }
            }
        };

        let module = parse_ok(tokens);

        assert_eq!(module.functions.len(), 1);
    }

    /// Verify that if a freestanding function has argument types that were not declared in the
    /// module we return an error.
    #[test]
    fn freestanding_function_argument_undeclared_type() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    fn a (bar: Bar);
                    fn b (bar: &Bar);
                    fn c (bar: &mut Bar);
                    // Counts as two errors.
                    fn d (multiple: Bar, args: Bar);
                }
            }
        };
        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 5);

        for error in errors.iter() {
            match error {
                ParseError::UndeclaredType { ty } => {
                    let ty_name = ty.to_token_stream().to_string();
                    // "& Bar" -> "Bar"
                    let ty_name = ty_name.split_whitespace().last().unwrap();

                    assert_eq!(ty_name, "Bar");
                }
                _ => panic!(),
            }
        }
    }

    /// Verify that a freestanding function can return a declared type.
    #[test]
    fn freestanding_function_return_declared_type() {
        let tokens = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Rust" {
                    type Bar;

                    fn a () -> Bar;
                }
            }
        };
        let module = parse_ok(tokens);
        assert_eq!(module.functions.len(), 1);
    }

    /// Verify that if a freestanding function returns a type that was not declared in the module
    /// we return an error.
    #[test]
    fn freestanding_function_returns_undeclared_type() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type Foo;

                    fn a () -> Bar;
                    fn a () -> &Bar;
                    fn a () -> &mut Bar;
                }
            }
        };
        let errors = parse_errors(tokens);
        assert_eq!(errors.len(), 3);

        for error in errors.iter() {
            match error {
                ParseError::UndeclaredType { ty } => {
                    let ty_name = ty.to_token_stream().to_string();
                    // "& Bar" -> "Bar"
                    let ty_name = ty_name.split_whitespace().last().unwrap();

                    assert_eq!(ty_name, "Bar");
                }
                _ => panic!(),
            }
        }
    }

    /// Verify that if an extern Rust block has more than one type, we push errors for any methods
    /// that have an ambiguous self.
    #[test]
    fn error_if_method_has_ambiguous_self() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    type AnotherType;

                    fn a (self);
                    fn b (&self);
                    fn c (&mut self);
                }
            }
        };

        let errors = parse_errors(tokens);

        assert_eq!(errors.len(), 3);

        for idx in 0..3 {
            match &errors[idx] {
                ParseError::AmbiguousSelf { self_: _ } => {}
                _ => panic!(),
            };
        }
    }

    /// Verify that annotated self methods get parsed.
    #[test]
    fn disambiguate_method() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    type AnotherType;

                    fn a (self: SomeType);
                    fn b (self: &SomeType);
                    fn c (self: &mut AnotherType);
                }
            }
        };

        let module = parse_ok(tokens);

        let functions = &module.functions;

        for (ty_name, expected_count) in vec![("SomeType", 2), ("AnotherType", 1)] {
            assert_eq!(
                functions
                    .iter()
                    .filter(
                        |f| f.associated_type.as_ref().unwrap().unwrap_opaque().ident == ty_name
                    )
                    .count(),
                expected_count
            );
        }
    }

    /// Verify that if we have multiple externs types can be inferred within each.
    #[test]
    fn infer_type_with_multiple_externs() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    fn a (&self);
                }

                extern "Rust" {
                    type AnotherType;

                    fn b (&self);
                }
            }
        };

        let module = parse_ok(tokens);

        let functions = &module.functions;

        for (ty_name, expected_count) in vec![("SomeType", 1), ("AnotherType", 1)] {
            assert_eq!(
                functions
                    .iter()
                    .filter(
                        |f| f.associated_type.as_ref().unwrap().unwrap_opaque().ident == ty_name
                    )
                    .count(),
                expected_count
            );
        }
    }

    /// Verify that we we do not get any parsing errors when we use a type that is declared in
    /// an extern block that comes after the block that it is used in.
    #[test]
    fn type_declared_in_separate_extern_block_after_use() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    fn a () -> AnotherType;
                    fn b () -> Vec<AnotherType>;
                    // TODO: (Dec 2021) Uncomment this when we support Option<OpaqueRustType>
                    // fn c () -> Option<AnotherType>;
                    fn d (arg: AnotherType);
                }

                extern "Rust" {
                    type AnotherType;
                }
            }
        };

        assert_eq!(parse_errors(tokens).len(), 0,);
    }

    /// Verify that we can parse the `already_declared` attribute.
    #[test]
    fn parse_already_declared_attribute() {
        let tokens = quote! {
            mod foo {
                extern "Rust" {
                    #[swift_bridge(already_declared)]
                    type AnotherType;
                }
            }
        };

        let module = parse_ok(tokens);

        assert!(
            module
                .types
                .get("AnotherType")
                .unwrap()
                .unwrap_opaque()
                .already_declared
        );
    }
}
