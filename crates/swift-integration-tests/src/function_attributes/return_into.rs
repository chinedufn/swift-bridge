use ffi2::AlreadyDeclaredStruct;

#[swift_bridge::bridge]
mod ffi {
    struct SomeStruct;

    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct AlreadyDeclaredStruct;

    extern "Rust" {
        type SomeType;

        // The `get_another_type` function returns "AnotherType".
        // Yet here we are trying to return "SomeType".
        // So, if this compiles it means that our `return_into` macro is working.
        #[swift_bridge(return_into)]
        fn get_another_type() -> SomeType;

        // Verify that our code compiles when we use `return_into` on a shared struct.
        #[swift_bridge(return_into)]
        fn get_struct() -> SomeStruct;

        // Verify that our code compiles when we use `return_into` on an already declared
        // shared struct.
        #[swift_bridge(return_into)]
        fn get_already_declared_struct() -> AlreadyDeclaredStruct;
    }
}
#[swift_bridge::bridge]
mod ffi2 {
    struct AlreadyDeclaredStruct;
}

pub struct SomeType;

struct AnotherType;

fn get_another_type() -> AnotherType {
    AnotherType
}

fn get_struct() -> SomeType {
    SomeType
}

fn get_already_declared_struct() -> SomeType {
    SomeType
}

impl Into<SomeType> for AnotherType {
    fn into(self) -> SomeType {
        SomeType
    }
}

impl Into<ffi::SomeStruct> for SomeType {
    fn into(self) -> ffi::SomeStruct {
        ffi::SomeStruct
    }
}
impl Into<ffi2::AlreadyDeclaredStruct> for SomeType {
    fn into(self) -> ffi2::AlreadyDeclaredStruct {
        ffi2::AlreadyDeclaredStruct
    }
}
