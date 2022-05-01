#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(declare_generic)]
        type SomeGenericType<A>;

        type SomeGenericType<u32>;

        fn new_some_generic_type_u32() -> SomeGenericType<u32>;
        fn reflect_generic_u32(arg: SomeGenericType<u32>) -> SomeGenericType<u32>;
    }

    extern "Rust" {
        #[swift_bridge(Copy(4))]
        type SomeGenericCopyType<u32>;

        fn new_some_generic_copy_type_u32() -> SomeGenericCopyType<u32>;
        fn reflect_generic_copy_u32(arg: SomeGenericCopyType<u32>) -> SomeGenericCopyType<u32>;
    }

    extern "Rust" {
        #[swift_bridge(declare_generic)]
        type GenericWithOpaqueRustInnerTy<A>;
        type GenericWithOpaqueRustInnerTy<InnerTy>;
        type InnerTy;

        fn new_generic_with_inner_opaque_type() -> GenericWithOpaqueRustInnerTy<InnerTy>;
        fn reflect_generic_with_inner_opaque_type(
            arg: GenericWithOpaqueRustInnerTy<InnerTy>,
        ) -> GenericWithOpaqueRustInnerTy<InnerTy>;
    }
}

pub struct SomeGenericType<T> {
    #[allow(unused)]
    field: T,
}

#[derive(Copy, Clone)]
pub struct SomeGenericCopyType<T> {
    #[allow(unused)]
    field: T,
}

pub struct GenericWithOpaqueRustInnerTy<T> {
    #[allow(unused)]
    field: T,
}
pub struct InnerTy;

fn new_some_generic_type_u32() -> SomeGenericType<u32> {
    SomeGenericType { field: 123 }
}

fn reflect_generic_u32(arg: SomeGenericType<u32>) -> SomeGenericType<u32> {
    arg
}

fn new_some_generic_copy_type_u32() -> SomeGenericCopyType<u32> {
    SomeGenericCopyType { field: 123 }
}
fn reflect_generic_copy_u32(arg: SomeGenericCopyType<u32>) -> SomeGenericCopyType<u32> {
    arg
}

fn new_generic_with_inner_opaque_type() -> GenericWithOpaqueRustInnerTy<InnerTy> {
    GenericWithOpaqueRustInnerTy { field: InnerTy }
}
fn reflect_generic_with_inner_opaque_type(
    arg: GenericWithOpaqueRustInnerTy<InnerTy>,
) -> GenericWithOpaqueRustInnerTy<InnerTy> {
    arg
}
