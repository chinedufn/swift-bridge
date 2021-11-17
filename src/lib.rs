//! Generate FFI glue between Swift and Rust code.

#![deny(missing_docs)]

use std::os::raw::c_void;
pub use swift_bridge_macro::bridge;

// The underlying T gets dropped when this is dropped.
#[doc(hidden)]
#[repr(C)]
pub struct OwnedPtrToRust<T> {
    pub ptr: *mut T,
}

// The underlying T gets dropped when this is dropped.
#[doc(hidden)]
#[repr(C)]
pub struct OwnedPtrToSwift {
    pub ptr: *mut c_void,
}

// The underlying T does not get dropped when this is dropped.
#[doc(hidden)]
#[repr(C)]
pub struct RefPtrToRust<T> {
    pub ptr: *mut T,
}

#[doc(hidden)]
#[repr(C)]
pub struct RustSlice<T> {
    pub start: *const T,
    pub len: usize,
}

#[doc(hidden)]
impl<T> RustSlice<T> {
    pub fn from_slice(slice: &[T]) -> Self {
        RustSlice {
            start: slice.as_ptr(),
            len: slice.len(),
        }
    }
}

impl<T> OwnedPtrToRust<T> {
    pub fn new(ptr: *mut T) -> Self {
        OwnedPtrToRust { ptr }
    }
}

impl<T> Drop for OwnedPtrToRust<T> {
    fn drop(&mut self) {
        let pointee = unsafe { Box::from_raw(self.ptr) };
        drop(pointee)
    }
}
