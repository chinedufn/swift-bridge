#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type ARustStack;

        #[swift_bridge(init)]
        fn new() -> ARustStack;

        fn push(&mut self, val: u8);
        fn pop(self: &mut ARustStack);

        fn as_ptr(&self) -> *const u8;
        fn len(&self) -> usize;

        fn as_slice(&self) -> &[u8];
    }

    extern "Rust" {
        type StackWrapper;

        #[swift_bridge(init)]
        fn new() -> StackWrapper;

        fn get_stack_mut(&mut self) -> &mut ARustStack;
    }

    extern "Rust" {
        #[swift_bridge(Copy(6))]
        type RustCopyType;

        #[swift_bridge(init)]
        fn new() -> RustCopyType;

        fn eq(&self, rhs: &RustCopyType) -> bool;

        // Used to verify that even after we pass by value the type still works on the
        // Swift side since it implements Copy.
        fn consume(self);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct RustCopyType([u16; 3]);
impl RustCopyType {
    fn new() -> Self {
        Self([11, 22, 33])
    }
    fn consume(self) {}
}

pub struct StackWrapper(ARustStack);

impl StackWrapper {
    fn new() -> Self {
        StackWrapper(ARustStack::new())
    }

    fn get_stack_mut(&mut self) -> &mut ARustStack {
        &mut self.0
    }
}

pub struct ARustStack {
    stack: Vec<u8>,
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

    pub fn as_slice(&self) -> &[u8] {
        self.stack.as_slice()
    }
}
