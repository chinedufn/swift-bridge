#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn print_hello();
    }

    extern "Swift" {}
}

fn print_hello() {
    println!("HI!")
}
