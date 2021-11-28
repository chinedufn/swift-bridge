use std::cell::RefCell;
use std::rc::Rc;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type SomeRustType;

        #[swift_bridge(associated_to = SomeRustType)]
        fn new(start_count: u32) -> SomeRustType;

        fn increment_counter(&mut self, amount: u32);

        fn test_call_swift_fn_with_owned_opaque_rust_arg();
    }

    extern "Swift" {
        fn increment_some_owned_opaque_rust_type(arg: SomeRustType, amount: u32);
    }
}

fn test_call_swift_fn_with_owned_opaque_rust_arg() {
    let some_rust_type = SomeRustType::new(5);
    let counter = some_rust_type.counter.clone();

    // Unwrap fails since there is a strong reference to the Rc held in `some_rust_type`
    let counter = Rc::try_unwrap(counter).err().unwrap();

    ffi::increment_some_owned_opaque_rust_type(some_rust_type, 10);

    // Unwrap succeeds since Swift freed the owned `some_rust_type` along with it's Rc at the end of
    // the `increment_some_rust_type` function.
    let counter = Rc::try_unwrap(counter).unwrap();

    assert_eq!(counter.take(), 15);
}

pub struct SomeRustType {
    counter: Rc<RefCell<u32>>,
}

impl SomeRustType {
    fn new(start_count: u32) -> Self {
        SomeRustType {
            counter: Rc::new(RefCell::new(start_count)),
        }
    }

    fn increment_counter(&mut self, amount: u32) {
        *self.counter.borrow_mut() += amount;
    }
}
