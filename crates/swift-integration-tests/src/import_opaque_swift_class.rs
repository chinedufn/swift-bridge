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
        fn as_slice(&self) -> &[u8];
    }
}

#[no_mangle]
pub extern "C" fn run_opaque_swift_class_tests() {
    use ffi::ASwiftStack;
    use std::ptr::slice_from_raw_parts;

    let mut stack = ASwiftStack::new();

    stack.push(5);
    stack.push(10);

    assert_eq!(stack.len(), 2);

    let ptr = stack.as_ptr();
    let len = stack.len();

    let vals: &[u8] = unsafe { &*slice_from_raw_parts(ptr, len) };

    assert_eq!(vals, &[5, 10]);
    assert_eq!(vals, stack.as_slice());

    stack.pop();
    assert_eq!(stack.len(), 1);
}
