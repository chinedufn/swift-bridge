#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn call_swift_add();
    }

    extern "Swift" {
        fn add(lhs: i32, rhs: i32) -> i32;
    }
}

fn call_swift_add() {
    assert!(ffi::add(1, 1) == 2);
}
