use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Test code generation for an extern "Rust" type.
mod extern_rust_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type SomeType;
                }
            }
        }
    }

    /// Verify that we generate a function that frees the memory behind an opaque pointer to a Rust
    /// type.
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$SomeType$_free"]
            pub extern "C" fn __swift_bridge__SomeType__free (
                this: *mut super::SomeType
            ) {
                let this = unsafe { Box::from_raw(this) };
                drop(this);
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public class SomeType: SomeTypeRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$SomeType$_free(ptr)
        }
    }
}
public class SomeTypeRefMut: SomeTypeRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SomeTypeRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
typedef struct SomeType SomeType;
void __swift_bridge__$SomeType$_free(void* self);
    "#,
    );

    #[test]
    fn extern_rust_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for an extern "Rust" type that implements Copy.
mod extern_rust_copy_type {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(Copy(32))]
                    type SomeType;
                }
            }
        }
    }

    // We use a somewhat hacky approach to asserting that the Copy size is correct at compile time.
    // In the future we'd prefer something like
    //  `assert_eq!(std::mem::size_of::<super::SomeType>(), 32usize);`
    // If compile time assertions are ever supported by Rust.
    // https://github.com/rust-lang/rfcs/issues/2790
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsManyAndDoesNotContainMany {
            contains: vec![
                quote! {
                    const _: () = {
                        let _: [u8; std::mem::size_of::<super::SomeType>()] = [0; 32usize];
                        fn _assert_copy() {
                            swift_bridge::assert_copy::<super::SomeType>();
                        }
                    };
                },
                quote! {
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct __swift_bridge__SomeType([u8; 32usize]);
                    impl __swift_bridge__SomeType {
                        #[inline(always)]
                        fn into_rust_repr(self) -> super::SomeType {
                            unsafe { std::mem::transmute(self) }
                        }

                        #[inline(always)]
                        fn from_rust_repr(repr: super::SomeType) -> Self {
                            unsafe { std::mem::transmute(repr) }
                        }
                    }
                },
            ],
            // Copy types don't need a function for freeing memory.
            does_not_contain: vec![quote! {
                __swift_bridge__SomeType__free
            }],
        }
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
public struct SomeType {
    fileprivate var bytes: __swift_bridge__$SomeType

    func intoFfiRepr() -> __swift_bridge__$SomeType {
        bytes
    }
}
"#,
            r#"
extension __swift_bridge__$SomeType {
    func intoSwiftRepr() -> SomeType {
        SomeType(bytes: self)
    }
}
"#,
        ])
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
typedef struct __swift_bridge__$SomeType { uint8_t bytes[32]; } __swift_bridge__$SomeType;
    "#,
            "#include <stdint.h>",
        ])
    }

    #[test]
    fn extern_rust_copy_type() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we properly generate a method for a Copy opaque Rust type.
mod extern_rust_copy_type_method {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(Copy(32))]
                    type SomeType;

                    fn some_method(self);
                    fn some_method_ref(&self);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::ContainsMany(vec![
            quote! {
                pub extern "C" fn __swift_bridge__SomeType_some_method (
                    this: __swift_bridge__SomeType
                ) {
                    this.into_rust_repr().some_method()
                }
            },
            quote! {
                pub extern "C" fn __swift_bridge__SomeType_some_method_ref (
                    this: __swift_bridge__SomeType
                ) {
                    this.into_rust_repr().some_method_ref()
                }
            },
        ])
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsManyAfterTrim(vec![
            r#"
extension SomeType {
    public func some_method() {
        __swift_bridge__$SomeType$some_method(self.bytes)
    }
}
"#,
            r#"
extension SomeType {
    public func some_method_ref() {
        __swift_bridge__$SomeType$some_method_ref(self.bytes)
    }
}
"#,
        ])
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsManyAfterTrim(vec![
            r#"
void __swift_bridge__$SomeType$some_method(struct __swift_bridge__$SomeType this);
    "#,
            r#"
void __swift_bridge__$SomeType$some_method_ref(struct __swift_bridge__$SomeType this);
    "#,
        ])
    }

    #[test]
    fn extern_rust_copy_type_method() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Test code generation for freestanding Swift function that takes an opaque Rust type argument.
mod extern_swift_freestanding_fn_with_owned_opaque_rust_type_arg {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    type MyType;
                }

                extern "Swift" {
                    fn some_function (arg: MyType);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub fn some_function (arg: super::MyType) {
                unsafe { __swift_bridge__some_function( Box::into_raw(Box::new(arg)) as *mut super::MyType ) }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$some_function"]
                fn __swift_bridge__some_function (arg: *mut super::MyType);
            }
        })
    }

    const EXPECTED_SWIFT: ExpectedSwiftCode = ExpectedSwiftCode::ContainsAfterTrim(
        r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function (_ arg: UnsafeMutableRawPointer) {
    some_function(arg: MyType(ptr: arg))
}
"#,
    );

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ContainsAfterTrim(
        r#"
typedef struct MyType MyType;
"#,
    );

    #[test]
    fn extern_swift_freestanding_fn_with_owned_opaque_rust_type_arg() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}
