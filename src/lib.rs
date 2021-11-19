//! Generate FFI glue between Swift and Rust code.

#![deny(missing_docs)]

pub use swift_bridge_macro::bridge;

mod std_bridge;

pub use self::std_bridge::string;

#[doc(hidden)]
#[repr(C)]
pub struct RustSlice<T> {
    pub start: *const T,
    pub len: usize,
}

impl<T> RustSlice<T> {
    pub fn from_slice(slice: &[T]) -> Self {
        RustSlice {
            start: slice.as_ptr(),
            len: slice.len(),
        }
    }

    pub fn as_slice(&self) -> &'static [T] {
        unsafe { std::slice::from_raw_parts(self.start, self.len) }
    }
}
