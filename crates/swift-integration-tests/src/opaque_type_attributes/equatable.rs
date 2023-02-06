#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Equatable)]
        type RustEquatableType;

        #[swift_bridge(init)]
        fn new() -> RustEquatableType;

        fn set_value(&mut self, value: i32);
    }
}

#[derive(PartialEq)]
pub struct RustEquatableType(i32);

impl RustEquatableType {
    fn new() -> Self {
        RustEquatableType(0)
    }

    fn set_value(&mut self, value: i32) {
        self.0 = value;
    }
}
