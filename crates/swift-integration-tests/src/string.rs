use std::ffi::c_void;
use std::marker::PhantomData;
use std::str::Utf8Error;

extern "C" {
    #[link_name = "swift_bridge$unstable$swift_string$new"]
    fn string_new(ptr: *const u8, len: usize) -> *mut c_void;

    #[link_name = "swift_bridge$unstable$swift_string$ptr"]
    fn string_ptr(this: *const c_void) -> *const u8;

    #[link_name = "swift_bridge$unstable$swift_string$free"]
    fn string_free(this: *mut c_void);

    #[link_name = "swift_bridge$unstable$swift_string$length"]
    fn string_length(this: *const c_void) -> usize;
}

#[no_mangle]
pub extern "C" fn run_string_tests() {
    let string = SwiftString::new("hello");

    assert_eq!(string.len(), 5);

    assert_eq!(string.to_str().unwrap(), "hello");

    drop(string);
}

struct SwiftString {
    ptr: *mut c_void,
}

impl SwiftString {
    fn new(initial: &str) -> Self {
        let ptr = unsafe { string_new(initial.as_ptr(), initial.len()) };
        Self { ptr }
    }

    fn as_ptr(&self) -> *const u8 {
        unsafe { string_ptr(self.ptr) }
    }

    fn as_bytes(&self) -> &[u8] {
        let ptr = self.as_ptr();
        let len = self.len();

        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    fn to_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    fn len(&self) -> usize {
        unsafe { string_length(self.ptr) }
    }
}

impl Drop for SwiftString {
    fn drop(&mut self) {
        unsafe { string_free(self.ptr) }
    }
}
