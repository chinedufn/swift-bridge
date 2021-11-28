use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::ToTokens;

use crate::parse::HostLang;
use crate::{BridgedType, FieldsFormat, SharedType, SwiftBridgeModule, SWIFT_BRIDGE_PREFIX};

mod option;

impl ToTokens for SwiftBridgeModule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mod_name = &self.name;

        let mut extern_rust_fn_tokens = vec![];

        let mut structs_for_swift_classes = vec![];

        let mut shared_struct_definitions = vec![];
        let mut impl_fn_tokens: HashMap<String, Vec<TokenStream>> = HashMap::new();
        let mut freestanding_rust_call_swift_fn_tokens = vec![];
        let mut extern_swift_fn_tokens = vec![];

        for func in &self.functions {
            match func.host_lang {
                HostLang::Rust => {
                    extern_rust_fn_tokens.push(
                        func.to_extern_c_function_tokens(&self.swift_bridge_path, &self.types),
                    );
                }
                HostLang::Swift => {
                    let tokens = func
                        .to_rust_fn_that_calls_a_swift_extern(&self.swift_bridge_path, &self.types);

                    if let Some(ty) = func.associated_type.as_ref() {
                        match ty {
                            BridgedType::Shared(_) => {
                                //

                                todo!()
                            }
                            BridgedType::Opaque(ty) => {
                                impl_fn_tokens
                                    .entry(ty.ident.to_string())
                                    .or_default()
                                    .push(tokens);
                            }
                        };
                    } else {
                        freestanding_rust_call_swift_fn_tokens.push(tokens);
                    }

                    extern_swift_fn_tokens.push(
                        func.to_extern_c_function_tokens(&self.swift_bridge_path, &self.types),
                    );
                }
            };
        }

        for ty in &self.types.types() {
            match ty {
                BridgedType::Shared(SharedType::Struct(shared_struct)) => {
                    let name = &shared_struct.name;

                    let fields: Vec<TokenStream> = shared_struct
                        .fields
                        .iter()
                        .map(|f| {
                            let ty = &f.ty;

                            match f.name.as_ref() {
                                Some(name) => {
                                    quote! {
                                        pub #name: #ty
                                    }
                                }
                                None => {
                                    quote! { pub #ty }
                                }
                            }
                        })
                        .collect();

                    let fields = match shared_struct.fields_format {
                        FieldsFormat::Named => {
                            quote! { { #(#fields),* } }
                        }
                        FieldsFormat::Unnamed => {
                            quote! { ( #(#fields),*  ); }
                        }
                        FieldsFormat::Unit => {
                            quote! { ; }
                        }
                    };

                    let definition = quote! {
                        #[repr(C)]
                        pub struct #name #fields
                    };
                    shared_struct_definitions.push(definition);
                }
                BridgedType::Opaque(ty) => {
                    let link_name =
                        format!("{}${}$_free", SWIFT_BRIDGE_PREFIX, ty.ident.to_string(),);
                    let free_mem_func_name = Ident::new(
                        &format!("{}{}__free", SWIFT_BRIDGE_PREFIX, ty.ident.to_string()),
                        ty.ident.span(),
                    );
                    let this = &ty.ident;

                    match ty.host_lang {
                        HostLang::Rust => {
                            let free = quote! {
                                #[no_mangle]
                                #[export_name = #link_name]
                                pub extern "C" fn #free_mem_func_name (this: *mut super::#this) {
                                    let this = unsafe { Box::from_raw(this) };
                                    drop(this);
                                }
                            };
                            extern_rust_fn_tokens.push(free);
                        }
                        HostLang::Swift => {
                            let ty_name = &ty.ident;

                            let impls = match impl_fn_tokens.get(&ty_name.to_string()) {
                                Some(impls) if impls.len() > 0 => {
                                    quote! {
                                        impl #ty_name {
                                            #(#impls)*
                                        }
                                    }
                                }
                                _ => {
                                    quote! {}
                                }
                            };

                            let struct_tokens = quote! {
                                #[repr(C)]
                                pub struct #ty_name(*mut std::ffi::c_void);

                                #impls

                                impl Drop for #ty_name {
                                    fn drop (&mut self) {
                                        unsafe { #free_mem_func_name(self.0) }
                                    }
                                }
                            };
                            structs_for_swift_classes.push(struct_tokens);

                            let free = quote! {
                                #[link_name = #link_name]
                                fn #free_mem_func_name (this: *mut std::ffi::c_void);
                            };
                            extern_swift_fn_tokens.push(free);
                        }
                    };
                }
            }
        }

        let extern_swift_fn_tokens = if extern_swift_fn_tokens.len() > 0 {
            quote! {
                extern "C" {
                    #(#extern_swift_fn_tokens)*
                }
            }
        } else {
            quote! {}
        };

        let module_inner = quote! {
            #(#shared_struct_definitions)*

            #(#extern_rust_fn_tokens)*

            #(#freestanding_rust_call_swift_fn_tokens)*

            #(#structs_for_swift_classes)*

            #extern_swift_fn_tokens
        };

        let t = quote! {
            mod #mod_name {
                #module_inner
            }
        };
        t.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::parse::SwiftBridgeModuleAndErrors;
    use crate::test_utils::{assert_tokens_contain, assert_tokens_eq};

    use super::*;

    /// Verify that we generate a function that frees the memory behind an opaque pointer to a Rust
    /// type.
    #[test]
    fn free_opaque_rust_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                }
            }
        };
        let expected = quote! {
            mod foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$SomeType$_free"]
                pub extern "C" fn __swift_bridge__SomeType__free (
                    this: *mut super::SomeType
                ) {
                    let this = unsafe { Box::from_raw(this) };
                    drop(this);
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with no arguments.
    #[test]
    fn freestanding_rust_function_no_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function ();
                }
            }
        };
        let expected = quote! {
            mod foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn __swift_bridge__some_function () {
                    super::some_function()
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate an extern function for a freestanding extern Swift function.
    #[test]
    fn freestanding_swift_function_no_args() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    fn some_function ();
                }
            }
        };
        let expected = quote! {
            mod foo {
                pub fn some_function() {
                    unsafe { __swift_bridge__some_function() }
                }

                extern "C" {
                    #[link_name = "__swift_bridge__$some_function"]
                    fn __swift_bridge__some_function ();
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate functions for calling a freestanding extern Swift function with
    /// one argument.
    #[test]
    fn freestanding_swift_function_one_arg() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    fn some_function (start: bool);
                }
            }
        };
        let expected = quote! {
            mod foo {
                pub fn some_function(start: bool) {
                    unsafe { __swift_bridge__some_function(start) }
                }

                extern "C" {
                    #[link_name = "__swift_bridge__$some_function"]
                    fn __swift_bridge__some_function (start: bool);
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with no arguments.
    #[test]
    fn freestanding_rust_one_arg() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function (bar: u8);
                }
            }
        };
        let expected = quote! {
            mod foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn __swift_bridge__some_function (bar: u8) {
                    super::some_function(bar)
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with an argument of a
    /// declared type.
    #[test]
    fn freestanding_rust_func_with_declared_type_arg() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type MyType;
                    fn some_function (bar: &MyType);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function (
                bar: *mut super::MyType
            ) {
                super::some_function(unsafe { & * bar } )
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with an argument of a
    /// declared type.
    #[test]
    fn freestanding_rust_func_with_declared_swift_type_arg() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function (bar: MyType);
                }

                extern "Swift" {
                    type MyType;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function (
                bar: *mut std::ffi::c_void
            ) {
                super::some_function(
                    MyType(bar)
                )
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function that returns an opaque
    /// Swift type.
    #[test]
    fn freestanding_rust_func_returns_opaque_swift_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> MyType;
                }

                extern "Swift" {
                    type MyType;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function () -> MyType {
                super::some_function()
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding Rust function with a return type.
    #[test]
    fn freestanding_rust_with_built_in_return_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn some_function () -> u8;
                }
            }
        };
        let expected = quote! {
            mod foo {
                #[no_mangle]
                #[export_name = "__swift_bridge__$some_function"]
                pub extern "C" fn __swift_bridge__some_function () -> u8 {
                    super::some_function()
                }
            }
        };

        assert_tokens_eq(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a freestanding function that returns a declared type.
    #[test]
    fn freestanding_rust_function_with_declared_return_type() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Rust" {
                    type Foo;
                    fn some_function () -> Foo;
                }
            }
        };
        let expected_func = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function () -> *mut super::Foo {
                Box::into_raw(Box::new(super::some_function())) as *mut super::Foo
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
    }

    /// Verify that an associated function we can return a reference to a declared type.
    #[test]
    fn associated_fn_return_reference_to_bridged_type() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Rust" {
                    type Foo;
                    fn some_function () -> &'static Foo;
                }
            }
        };
        let expected_func = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function () -> *const super::Foo {
                super::some_function() as *const super::Foo
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
    }

    /// Verify that a method can return a reference to a declared type.
    #[test]
    fn method_return_reference_to_bridged_type() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Rust" {
                    type Foo;
                    fn some_function (&mut self) -> &mut Foo;
                }
            }
        };
        let expected_func = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$Foo$some_function"]
            pub extern "C" fn __swift_bridge__Foo_some_function (
                this: *mut super::Foo
            ) -> *mut super::Foo {
                (unsafe { &mut * this }).some_function() as *mut super::Foo
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
    }

    /// Verify that we generate a struct for an extern Swift type.
    #[test]
    fn generates_struct_for_swift_type() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Swift" {
                    type Foo;
                }
            }
        };
        let expected = quote! {
            #[repr(C)]
            pub struct Foo(*mut std::ffi::c_void);

            impl Drop for Foo {
                fn drop (&mut self) {
                    unsafe { __swift_bridge__Foo__free(self.0) }
                }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$Foo$_free"]
                fn __swift_bridge__Foo__free (this: *mut std::ffi::c_void);
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a static method that's associated to a type.
    #[test]
    fn associated_rust_static_method_no_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    #[swift_bridge(associated_to = SomeType)]
                    fn new () -> SomeType;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$new"]
            pub extern "C" fn __swift_bridge__SomeType_new () -> *mut super::SomeType {
                Box::into_raw(Box::new(super::SomeType::new())) as *mut super::SomeType
            }
        };

        assert_to_extern_c_function_tokens(start, &expected);
    }

    /// Verify that we generate an associated function for a Swift class method.
    #[test]
    fn swift_class_method_no_args() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Swift" {
                    type Foo;

                    #[swift_bridge(associated_to = Foo)]
                    fn new () -> Foo;
                }
            }
        };
        let expected = quote! {
            #[repr(C)]
            pub struct Foo(*mut std::ffi::c_void);

            impl Foo {
                pub fn new () -> Foo {
                    Foo(unsafe{ __swift_bridge__Foo_new() })
                }
            }

            impl Drop for Foo {
                fn drop (&mut self) {
                    unsafe { __swift_bridge__Foo__free(self.0) }
                }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$Foo$new"]
                fn __swift_bridge__Foo_new() -> *mut std::ffi::c_void;

                #[link_name = "__swift_bridge__$Foo$_free"]
                fn __swift_bridge__Foo__free (this: *mut std::ffi::c_void);
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate tokens for a static method that have arguments.
    #[test]
    fn associated_rust_static_method_one_arg() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    #[swift_bridge(associated_to = SomeType)]
                    fn new (foo: u8) -> u8;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$new"]
            pub extern "C" fn __swift_bridge__SomeType_new (foo: u8) -> u8 {
                super::SomeType::new(foo)
            }
        };

        assert_to_extern_c_function_tokens(start, &expected);
    }

    /// Verify that we generate tokens for a static method that does not have a return type.
    #[test]
    fn associated_static_method_no_return_type() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;

                    #[swift_bridge(associated_to = SomeType)]
                    fn new ();
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$new"]
            pub extern "C" fn __swift_bridge__SomeType_new ()  {
                 super::SomeType::new()
            }
        };

        assert_to_extern_c_function_tokens(start, &expected);
    }

    /// Verify that we generate the tokens for exposing an associated method.
    #[test]
    fn associated_rust_method_no_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type MyType;
                    fn increment (&mut self);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$MyType$increment"]
            pub extern "C" fn __swift_bridge__MyType_increment (
                this: *mut super::MyType
            ) {
                (unsafe { &mut *this }).increment()
            }
        };

        assert_to_extern_c_function_tokens(start, &expected);
    }

    /// Verify that we generate a method for a Swift class' instance method.
    #[test]
    fn swift_instance_methods() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                extern "Swift" {
                    type Foo;

                    fn notify (&self);
                    fn message (self: &Foo);
                    fn call (&mut self, volume: u8);
                }
            }
        };
        let expected = quote! {
            #[repr(C)]
            pub struct Foo(*mut std::ffi::c_void);

            impl Foo {
                pub fn notify (&self) {
                    unsafe { __swift_bridge__Foo_notify(self.0) }
                }

                pub fn message (&self) {
                    unsafe { __swift_bridge__Foo_message(self.0) }
                }

                pub fn call (&mut self, volume: u8) {
                    unsafe { __swift_bridge__Foo_call(self.0, volume) }
                }
            }

            impl Drop for Foo {
                fn drop (&mut self) {
                    unsafe { __swift_bridge__Foo__free(self.0) }
                }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$Foo$notify"]
                fn __swift_bridge__Foo_notify(this: *mut std::ffi::c_void);

                #[link_name = "__swift_bridge__$Foo$message"]
                fn __swift_bridge__Foo_message(this: *mut std::ffi::c_void);

                #[link_name = "__swift_bridge__$Foo$call"]
                fn __swift_bridge__Foo_call(this: *mut std::ffi::c_void, volume: u8);

                #[link_name = "__swift_bridge__$Foo$_free"]
                fn __swift_bridge__Foo__free (this: *mut std::ffi::c_void);
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we generate the tokens for exposing an associated method.
    #[test]
    fn rust_associated_method_one_args() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    fn message (&self, val: u8);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$message"]
            pub extern "C" fn __swift_bridge__SomeType_message (
                this: *mut super::SomeType,
                val: u8
            ) {
                (unsafe { &*this }).message(val)
            }
        };

        assert_to_extern_c_function_tokens(start, &expected);
    }

    /// Verify that extern "Rust" functions we can accept and return void pointers.
    #[test]
    fn extern_rust_void_pointers() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    fn void_pointers (arg1: *const c_void, arg2: *mut c_void) -> *const c_void;
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$void_pointers"]
            pub extern "C" fn __swift_bridge__void_pointers (
                arg1: *const super::c_void,
                arg2: *mut super::c_void
            ) -> *const super::c_void {
                super::void_pointers(arg1, arg2)
            }
        };

        assert_to_extern_c_function_tokens(start, &expected);
    }

    /// Verify that extern "Rust" functions we can accept and return pointers to built in types.
    #[test]
    fn extern_swift_built_in_pointers() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    fn built_in_pointers (arg1: *const u8, arg2: *mut i16) -> *const u32;
                }
            }
        };
        let expected = quote! {
            pub fn built_in_pointers (arg1: *const u8, arg2: *mut i16) -> *const u32 {
                unsafe { __swift_bridge__built_in_pointers(arg1, arg2) }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$built_in_pointers"]
                fn __swift_bridge__built_in_pointers (
                    arg1: *const u8,
                    arg2: *mut i16
                ) -> *const u32;
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that extern "Swift" functions we can accept and return void pointers.
    #[test]
    fn extern_swift_void_pointers() {
        let start = quote! {
            mod foo {
                extern "Swift" {
                    fn void_pointers (arg1: *const c_void, arg2: *mut c_void) -> *const c_void;
                }
            }
        };
        let expected = quote! {
            pub fn void_pointers (arg1: *const super::c_void, arg2: *mut super::c_void) -> *const super::c_void {
                unsafe { __swift_bridge__void_pointers(arg1, arg2) }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$void_pointers"]
                fn __swift_bridge__void_pointers (
                    arg1: *const super::c_void,
                    arg2: *mut super::c_void
                ) -> *const super::c_void;
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected);
    }

    /// Verify that we do not wrap the type in manually drop if the method uses owned self.
    #[test]
    fn associated_method_drops_owned_self() {
        let start = quote! {
            mod foo {
                extern "Rust" {
                    type SomeType;
                    fn consume (self);
                }
            }
        };
        let expected = quote! {
            #[no_mangle]
            #[export_name = "__swift_bridge__$SomeType$consume"]
            pub extern "C" fn __swift_bridge__SomeType_consume (
                this: *mut super::SomeType
            ) {
                (* unsafe { Box::from_raw(this) }).consume()
            }
        };

        assert_to_extern_c_function_tokens(start, &expected);
    }

    /// Verify that we generate tokens for a freestanding function that has a shared struct as
    /// as argument type.
    #[test]
    fn freestanding_rust_function_shared_struct_arg() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                #[swift_bridge(swift_repr = "struct")]
                struct Foo;

                extern "Rust" {
                    fn some_function (arg: Foo);
                }
            }
        };
        let expected_func = quote! {
            #[repr(C)]
            pub struct Foo;

            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function (arg: Foo) {
                super::some_function(arg)
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
    }

    /// Verify that we generate tokens for a freestanding function that returns a shared struct.
    #[test]
    fn freestanding_rust_function_returns_shared_struct() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                #[swift_bridge(swift_repr = "struct")]
                struct Foo;

                extern "Rust" {
                    fn some_function () -> Foo;
                }
            }
        };
        let expected_func = quote! {
            #[repr(C)]
            pub struct Foo;

            #[no_mangle]
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function () -> Foo {
                super::some_function()
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
    }

    /// Verify that we create a shared struct that has named fields.
    #[test]
    fn creates_shared_struct_with_named_fields() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                #[swift_bridge(swift_repr = "struct")]
                struct Foo {
                    a: u8,
                    b: u32
                }
            }
        };
        let expected_func = quote! {
            #[repr(C)]
            pub struct Foo {
                pub a: u8,
                pub b: u32
            }
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
    }

    /// Verify that we create a shared tuple struct.
    #[test]
    fn creates_shared_tuple_struct() {
        let start = quote! {
            #[swift_bridge::bridge]
            mod foo {
                #[swift_bridge(swift_repr = "struct")]
                pub struct Foo (u8, u32);
            }
        };
        let expected_func = quote! {
            #[repr(C)]
            pub struct Foo (pub u8, pub u32);
        };

        assert_tokens_contain(&parse_ok(start).to_token_stream(), &expected_func);
    }

    fn parse_ok(tokens: TokenStream) -> SwiftBridgeModule {
        let module_and_errors: SwiftBridgeModuleAndErrors = syn::parse2(tokens).unwrap();
        module_and_errors.module
    }

    fn assert_to_extern_c_function_tokens(module: TokenStream, expected_fn: &TokenStream) {
        let module = parse_ok(module);
        let function = &module.functions[0];

        assert_tokens_eq(
            &function.to_extern_c_function_tokens(&module.swift_bridge_path, &module.types),
            &expected_fn,
        );
    }
}
