#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(declare_generic)]
        type SomeGenericType<A>;

        type SomeGenericType<u32>;

        fn new_some_generic_type_u32() -> SomeGenericType<u32>;

        fn reflect_generic_u32(arg: SomeGenericType<u32>) -> SomeGenericType<u32>;
    }
}

pub struct SomeGenericType<T> {
    #[allow(unused)]
    field: T,
}

fn new_some_generic_type_u32() -> SomeGenericType<u32> {
    SomeGenericType { field: 123 }
}

fn reflect_generic_u32(arg: SomeGenericType<u32>) -> SomeGenericType<u32> {
    arg
}
