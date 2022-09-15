use ffi2::AlreadyDeclaredStruct;

#[swift_bridge::bridge]
mod ffi {
    struct ReturnIntoSomeStruct;

    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct AlreadyDeclaredStruct;

    enum SomeTransparentEnum {
        Variant,
    }

    extern "Rust" {
        type SomeType;

        // The `get_another_type` function returns "AnotherType".
        // Yet here we are trying to return "SomeType".
        // So, if this compiles it means that our `return_into` macro is working.
        #[swift_bridge(return_into)]
        fn get_another_type() -> SomeType;

        // Verify that our code compiles when we use `return_into` on a shared struct.
        #[swift_bridge(return_into)]
        fn get_struct() -> ReturnIntoSomeStruct;

        // Verify that our code compiles when we use `return_into` on an already declared
        // shared struct.
        #[swift_bridge(return_into)]
        fn get_already_declared_struct() -> AlreadyDeclaredStruct;

        // Verify that our code compiles when we use `return_into` on an already declared
        // shared struct.
        #[swift_bridge(return_into)]
        fn get_transparent_enum() -> SomeTransparentEnum;
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

fn get_transparent_enum() -> u32 {
    123
}
impl Into<ffi::SomeTransparentEnum> for u32 {
    fn into(self) -> ffi::SomeTransparentEnum {
        ffi::SomeTransparentEnum::Variant
    }
}

impl Into<SomeType> for AnotherType {
    fn into(self) -> SomeType {
        SomeType
    }
}

impl Into<ffi::ReturnIntoSomeStruct> for SomeType {
    fn into(self) -> ffi::ReturnIntoSomeStruct {
        ffi::ReturnIntoSomeStruct
    }
}
impl Into<ffi2::AlreadyDeclaredStruct> for SomeType {
    fn into(self) -> ffi2::AlreadyDeclaredStruct {
        ffi2::AlreadyDeclaredStruct
    }
}
