//! Generate FFI glue between Swift and Rust code.

#![deny(missing_docs)]

pub use swift_bridge_macro::bridge;

mod std_bridge;

pub use self::std_bridge::{option, string};

#[doc(hidden)]
#[repr(C)]
pub struct FfiSlice<T> {
    pub start: *const T,
    pub len: usize,
}

impl<T> FfiSlice<T> {
    /// Create an FfiSlice from a slice.
    pub fn from_slice(slice: &[T]) -> Self {
        FfiSlice {
            start: slice.as_ptr(),
            len: slice.len(),
        }
    }

    /// Get a reference to the slice that this FfiSlice points to.
    pub fn as_slice(&self) -> &'static [T] {
        unsafe { std::slice::from_raw_parts(self.start, self.len) }
    }
}
