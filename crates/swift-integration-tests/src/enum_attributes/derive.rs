#[swift_bridge::bridge]
mod ffi {
    #[derive(Debug)]
    enum DeriveDebugEnum {
        Variant,
    }
}
