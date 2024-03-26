#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct ArgsIntoSomeStruct {
        field: u64,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct AnotherStruct {
        foo: u8,
    }

    extern "Rust" {
        // The `test_args_into` function declaration here accepts `ArgsIntoSomeStruct`
        // and `AnotherStruct`,
        // but the real definition outside this module accepts different types that each of these
        // types impl Into for.
        // So.. if this compiles then we know that our `args_into` attribute is working.
        #[swift_bridge(args_into = (some_arg, another_arg))]
        fn test_args_into(some_arg: ArgsIntoSomeStruct, another_arg: AnotherStruct);
    }
}

fn test_args_into(_some_arg: TypeA, _another_arg: TypeB) {}

struct TypeA;

enum TypeB {
    #[allow(unused)]
    Foo(u8),
}

impl Into<TypeA> for ffi::ArgsIntoSomeStruct {
    fn into(self) -> TypeA {
        TypeA
    }
}

impl Into<TypeB> for ffi::AnotherStruct {
    fn into(self) -> TypeB {
        TypeB::Foo(self.foo)
    }
}
