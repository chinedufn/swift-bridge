#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(Hashable, Equatable)]
        type RustHashableType;

        #[swift_bridge(init)]
        fn new(num: isize) -> RustHashableType;

        #[swift_bridge(Copy(4), Hashable, Equatable)]
        type RustCopyHashableType;

        #[swift_bridge(init)]
        fn new(num: i32) -> RustCopyHashableType;
    }
}

#[derive(Hash, PartialEq)]
pub struct RustHashableType(isize);

impl RustHashableType {
    fn new(num: isize) -> Self {
        RustHashableType(num)
    }
}

#[derive(Clone, Copy, Hash, PartialEq)]
pub struct RustCopyHashableType(i32);

impl RustCopyHashableType {
    fn new(num: i32) -> Self {
        Self(num)
    }
}
