#[swift_bridge::bridge]
mod ffi {
    extern "Swift" {
        type ASwiftStack;

        #[swift_bridge(init)]
        fn new() -> ASwiftStack;

        fn push(&mut self, val: u8);
        fn pop(self: &mut ASwiftStack);

        fn as_ptr(&self) -> *const u8;
        fn len(&self) -> usize;
        // fn as_slice(&self) -> &[u8];
    }
}

#[no_mangle]
pub extern "C" fn run_opaque_swift_class_tests() {
    use ffi::ASwiftStack;

    let mut stack = ASwiftStack::new();

    stack.push(5);
    stack.push(10);

    assert_eq!(stack.len(), 2);

    let ptr = stack.as_ptr();
    let len = stack.len();

    let vals = unsafe { &*slice_from_raw_parts(ptr, len) };

    assert_eq!(vals, &[5, 10]);

    stack.pop();
    assert_eq!(stack.len(), 1);
}

use std::ptr::slice_from_raw_parts;
mod __ffi_generated {
    use std::ffi::c_void;

    // TODO: Add a test that generates this type declaration...
    //  then we can start working on supporting methods..
    //  re-use as much from our extern Rust handling code as possible
    pub struct ASwiftStack(*mut c_void);

    impl ASwiftStack {
        pub fn new() -> ASwiftStack {
            let stack = unsafe { _new() };
            ASwiftStack(stack)
        }

        pub fn push(&mut self, val: u8) {
            unsafe { push(self.0, val) };
        }

        pub fn pop(&mut self) {
            unsafe { pop(self.0) };
        }

        pub fn as_ptr(&mut self) -> *const u8 {
            unsafe { as_ptr(self.0) }
        }

        pub fn len(&mut self) -> usize {
            unsafe { len(self.0) }
        }
    }

    extern "C" {
        #[link_name = "swift_bridge$unstable$freestanding$new"]
        fn _new() -> *mut c_void;

        #[link_name = "swift_bridge$unstable$ASwiftStack$push"]
        fn push(this: *mut c_void, val: u8);

        #[link_name = "swift_bridge$unstable$ASwiftStack$pop"]
        fn pop(this: *mut c_void);

        #[link_name = "swift_bridge$unstable$ASwiftStack$as_ptr"]
        fn as_ptr(this: *mut c_void) -> *const u8;

        #[link_name = "swift_bridge$unstable$ASwiftStack$len"]
        fn len(this: *mut c_void) -> usize;
    }
}
