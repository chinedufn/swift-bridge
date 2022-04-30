use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that we can declare a generic type.
mod declare_generic {
    use super::*;

    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(declare_generic)]
                    type SomeType<A>;
                }
            }
        }
    }

    // We don't declare any extern functions (such as the function to free memory) for the
    // generic declaration. We only declare functions for the concrete types.
    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::DoesNotContain(quote! {
            fn
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
public class SomeType<A>: SomeTypeRefMut<A> {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            (self as! SwiftBridgeGenericFreer).rust_free()
        }
    }
}
public class SomeTypeRefMut<A>: SomeTypeRef<A> {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class SomeTypeRef<A> {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
"#,
        )
    }

    // Nothing to bridge for a declared generic. We only need to bridge its monomorphizations.
    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::DoesNotContainAfterTrim("SomeType")
    }

    #[test]
    fn declare_generic() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we can declare a generic with concrete types.
mod monomorphized_generic_opaque_rust_type {
    use super::*;
    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(declare_generic)]
                    type SomeType<A>;

                    type SomeType<u32>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$SomeType$u32$_free"]
            pub extern "C" fn __swift_bridge__SomeType_u32__free (
                this: *mut super::SomeType<u32>
            ) {
                let this = unsafe { Box::from_raw(this) };
                drop(this);
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
extension SomeType: SwiftBridgeGenericFreer
where A == UInt32 {
    public func rust_free() {
        __swift_bridge__$SomeType$u32$_free(ptr)
    }
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$SomeType$u32$_free(void* self);
    "#,
        )
    }

    #[test]
    fn monomorphized_generic_opaque_rust_type() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we can use a generic opaque Rust type as a function argument.
mod generic_opaque_rust_type_arg {
    use super::*;
    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(declare_generic)]
                    type SomeType<A>;

                    type SomeType<u32>;
                    fn some_function(arg: SomeType<u32>);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function (
                arg: *mut super::SomeType<u32>
            ) {
                super::some_function(unsafe { * Box::from_raw(arg) })
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: SomeType<UInt32>) {
    __swift_bridge__$some_function({arg.isOwned = false; return arg.ptr;}())
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void __swift_bridge__$some_function(void* arg);
    "#,
        )
    }

    #[test]
    fn generic_opaque_rust_type_arg() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}

/// Verify that we can return a generic opaque Rust type from a function.
mod generic_opaque_rust_type_return {
    use super::*;
    fn bridge_module() -> TokenStream {
        quote! {
            #[swift_bridge::bridge]
            mod ffi {
                extern "Rust" {
                    #[swift_bridge(declare_generic)]
                    type SomeType<A>;

                    type SomeType<u32>;
                    fn some_function() -> SomeType<u32>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function () -> *mut super::SomeType<u32> {
                 Box::into_raw(Box::new(super::some_function())) as *mut super::SomeType<u32>
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function() -> SomeType<UInt32> {
    SomeType(ptr: __swift_bridge__$some_function())
}
"#,
        )
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
void* __swift_bridge__$some_function(void);
    "#,
        )
    }

    #[test]
    fn generic_opaque_rust_type_return() {
        CodegenTest {
            bridge_module: bridge_module().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
