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
