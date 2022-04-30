# Generics

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        // Declare the generic type once.
        #[swift_bridge(declare_generic)]
        type MyType<A, B>;
    }

    extern "Rust" {
        // Bridge as many monomorphized types as you like.
        type MyType<u32, String>;
        fn some_function(arg: MyType<u32, String>) -> &str;

        type MyType<i8, Vec<u8>>;
    }
}

pub struct MyType<T, U> {
    my_field1: T,
    mu_field2: U
}
fn some_function(arg: MyType<u32, String>) -> &str {
    unimplemented!()
}
```
