#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type IdentifiableFnNamedId;

        #[swift_bridge(init)]
        fn new() -> IdentifiableFnNamedId;
        // Here we make sure that the `Identifiable` attribute works on a function named `id`.
        // Functions named `id` don't get their Identifiable protocol extension filled out since
        // the `id` function already exists.
        // i.e. we generate `extension IdentifiableFnNamedId: Identifiable {}`
        #[swift_bridge(Identifiable)]
        fn id(self: &IdentifiableFnNamedId) -> u16;
    }

    extern "Rust" {
        type IdentifiableFnNotNamedId;

        #[swift_bridge(init)]
        fn new() -> IdentifiableFnNotNamedId;
        // Here we make sure that the `Identifiable` attribute works on a function not named `id`.
        #[swift_bridge(Identifiable)]
        fn some_function(self: &IdentifiableFnNotNamedId) -> u32;
    }

    extern "Rust" {
        #[swift_bridge(Copy(1))]
        type OpaqueCopyTypeIdentifiable;

        #[swift_bridge(init)]
        fn new() -> OpaqueCopyTypeIdentifiable;
        // Here we make sure that the `Identifiable` attribute works on a Copy opaque type.
        #[swift_bridge(Identifiable)]
        fn id(self: &OpaqueCopyTypeIdentifiable) -> u8;
    }

    extern "Rust" {
        type IdentifiableU8;

        #[swift_bridge(init)]
        fn new() -> IdentifiableU8;
        #[swift_bridge(Identifiable)]
        fn id(&self) -> u8;
    }

    extern "Rust" {
        type IdentifiableI8;

        #[swift_bridge(init)]
        fn new() -> IdentifiableI8;
        #[swift_bridge(Identifiable)]
        fn id(&self) -> i8;
    }

    extern "Rust" {
        type IdentifiableStr;

        #[swift_bridge(init)]
        fn new() -> IdentifiableStr;
        #[swift_bridge(Identifiable)]
        fn id(&self) -> &'static str;
    }

    // TODO: Add more Identifiable test types..
}

pub struct IdentifiableFnNotNamedId;

impl IdentifiableFnNotNamedId {
    fn new() -> Self {
        Self
    }

    fn some_function(&self) -> u32 {
        123
    }
}

#[derive(Copy, Clone)]
pub struct OpaqueCopyTypeIdentifiable(u8);

impl OpaqueCopyTypeIdentifiable {
    fn new() -> Self {
        Self(123)
    }

    fn id(&self) -> u8 {
        self.0
    }
}

macro_rules! identifiable_test_type {
    ($name:ident, $id_ty:ty, $id_val:expr) => {
        pub struct $name;
        impl $name {
            fn new() -> Self {
                $name
            }

            fn id(&self) -> $id_ty {
                $id_val
            }
        }
    };
}

identifiable_test_type!(IdentifiableFnNamedId, u16, 123);

identifiable_test_type!(IdentifiableU8, u8, 123);
identifiable_test_type!(IdentifiableI8, i8, 123);
identifiable_test_type!(IdentifiableStr, &'static str, "hello world");
