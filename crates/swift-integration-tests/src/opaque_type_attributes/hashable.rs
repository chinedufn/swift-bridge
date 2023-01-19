#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Hashable, Equatable)]
        type RustHashableType;

        #[swift_bridge(init)]
        fn new(num: isize) -> RustHashableType;
    }
}

#[derive(Hash, PartialEq)]
pub struct RustHashableType(isize);

impl RustHashableType {
    fn new(num: isize) -> Self {
        RustHashableType(num)
    }
}
