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

    #[swift_bridge(swift_repr = "struct")]
    #[derive(serde::Debug)]
    struct StructDeriveDebug {
        field: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    #[derive(serde::Serialize)]
    struct StructDeriveSerialize {
        field: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    #[derive(serde::Deserialize)]
    struct StructDeriveDeserialize {
        field: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    #[derive(serde::Serialize, serde::Deserialize)]
    struct StructDeriveSerDe {
        field: String,
    }

    #[swift_bridge(swift_repr = "struct")]
    #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
    struct StructDeriveAll {
        field: u8,
    }
}
