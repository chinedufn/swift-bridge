#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Copy(6), Equatable)]
        type RustCopyEquatableType;

        #[swift_bridge(init)]
        fn new() -> RustCopyEquatableType;

        // PartialEq trait method, explicitly exposed for testing on the Swift side.
        fn eq(&self, rhs: &RustCopyEquatableType) -> bool;

        // Allow constructing with a value, to verify that truly different values don't compare equal.
        #[swift_bridge(init)]
        fn mutate_first(
            #[swift_bridge(label = "withFirstValue")] first_value: u16,
        ) -> RustCopyEquatableType;

        // Used to verify that even after we pass by value the type still works on the
        // Swift side since it implements Copy.
        fn consume(self);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct RustCopyEquatableType([u16; 3]);
impl RustCopyEquatableType {
    fn new() -> Self {
        Self([11, 22, 33])
    }
    fn mutate_first(first_value: u16) -> Self {
        let mut this = Self::new();
        this.0[0] = first_value;
        this
    }
    fn consume(self) {}
}
