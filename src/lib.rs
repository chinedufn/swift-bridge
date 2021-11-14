//! Generate FFI glue between Swift and Rust code.

#![deny(missing_docs)]

pub use swift_bridge_macro::bridge;

// The underlying T gets dropped when this is dropped.
#[doc(hidden)]
#[repr(C)]
pub struct OwnedPtrToRust<T> {
    pub ptr: *mut T,
}

// The underlying T does not get dropped when this is dropped.
#[doc(hidden)]
#[repr(C)]
pub struct RefPtrToRust<T> {
    pub ptr: *mut T,
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
