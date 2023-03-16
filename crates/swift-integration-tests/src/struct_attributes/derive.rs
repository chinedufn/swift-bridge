#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    #[derive(Copy, Clone)]
    struct StructDeriveCopy1;

    #[swift_bridge(swift_repr = "struct")]
    #[derive(Copy, Clone)]
    struct StructDeriveCopy2 {
        field: u8,
    }

    #[swift_bridge(swift_repr = "struct")]
    #[derive(Clone)]
    struct StructDeriveClone1;

    #[swift_bridge(swift_repr = "struct")]
    #[derive(Clone)]
    struct StructDeriveClone2 {
        field: u8,
    }

    #[swift_bridge(swift_repr = "struct")]
    #[derive(Clone)]
    struct StructDeriveClone3 {
        field: String,
    }
}