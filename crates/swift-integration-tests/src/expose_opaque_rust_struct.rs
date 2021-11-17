// macro generates all of the externs...
// parser parses the externs and generates Swift code..
// parser does this by compiling the file, with telling rustc to run the swift_bridge macro..
// then looking at the tokens in the file.
//
// Parser might get called from a build script or from a CLI
// So.. we basically need to call it on a crate and then traverse it looking for modules to parse.
// We can put this crates/swift-bridge-build
//
// 1. Define externs for an opaque Rust type in here
//
// 2. Define the corresponding class in Swift (wrapper around a raw pointer)
//
// 3. Add a test on the Swift side that we can create and call all of the methods on the Rust type
//
// 4. Make our `swift-bridge-build` with a `pub fn bridge(file: &Path)`. Add tests that we generate
//    the proper externs
//
// 5. Add tests that we generate the proper Swift code
//
// 6. Add test that we generate the proper header file
//
// 7. Call build from swift-integration-tests and have the build function write the files to disk
//    And have the code that it generates read those files
//
// 8. Start implementing more core types (ints .. Vec<T> .. etc)

pub struct ARustStack {
    stack: Vec<u8>,
}

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type ARustStack;

        fn new_stack() -> ARustStack;

        // fn push(&mut self, val: u8);
        // fn pop(self: &mut ARustStack);
        //
        // fn as_ptr(&self) -> *const u8;
        // fn len(&self) -> usize;
    }
}

pub fn new_stack() -> ARustStack {
    ARustStack::new()
}

mod __ffi_generated {
    use super::*;
    use swift_bridge::{OwnedPtrToRust, RefPtrToRust};

    #[no_mangle]
    #[export_name = "swift_bridge$unstable$ARustStruct$new"]
    pub extern "C" fn new() -> OwnedPtrToRust<ARustStack> {
        let stack = ARustStack::new();
        let stack = Box::into_raw(Box::new(stack));

        OwnedPtrToRust::new(stack)
    }

    #[no_mangle]
    #[export_name = "swift_bridge$unstable$ARustStruct$push"]
    pub extern "C" fn push(this: RefPtrToRust<ARustStack>, val: u8) {
        let stack = unsafe { &mut *this.ptr };
        stack.push(val);
    }

    #[no_mangle]
    #[export_name = "swift_bridge$unstable$ARustStruct$pop"]
    pub extern "C" fn pop(this: RefPtrToRust<ARustStack>) {
        let stack = unsafe { &mut *this.ptr };
        stack.pop();
    }

    #[no_mangle]
    #[export_name = "swift_bridge$unstable$ARustStruct$free"]
    pub extern "C" fn free(this: OwnedPtrToRust<ARustStack>) {
        drop(this)
    }

    #[no_mangle]
    #[export_name = "swift_bridge$unstable$ARustStruct$as_ptr"]
    pub extern "C" fn as_ptr(this: RefPtrToRust<ARustStack>) -> *const u8 {
        let stack = unsafe { &mut *this.ptr };
        stack.as_ptr()
    }

    #[no_mangle]
    #[export_name = "swift_bridge$unstable$ARustStruct$len"]
    pub extern "C" fn len(this: RefPtrToRust<ARustStack>) -> usize {
        let stack = unsafe { &mut *this.ptr };
        stack.len()
    }
}

impl ARustStack {
    fn new() -> ARustStack {
        ARustStack { stack: vec![] }
    }

    fn push(&mut self, val: u8) {
        self.stack.push(val);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn as_ptr(&self) -> *const u8 {
        self.stack.as_ptr()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }
}
