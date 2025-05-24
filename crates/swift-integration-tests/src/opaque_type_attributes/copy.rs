#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Copy(6))]
        type RustCopyType;

        #[swift_bridge(init)]
        fn new() -> RustCopyType;

        // PartialEq trait method, explicitly exposed for testing on the Swift side.
        fn eq(&self, rhs: &RustCopyType) -> bool;

        // Used to verify that even after we pass by value the type still works on the
        // Swift side since it implements Copy.
        fn consume(self);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct RustCopyType([u16; 3]);
impl RustCopyType {
    fn new() -> Self {
        Self([11, 22, 33])
    }
    fn consume(self) {}
}
