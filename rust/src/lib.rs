#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn print_hello_rust();
        fn is_from_rust() -> bool;
        fn get_hello_rust() -> String;
    } 
}

fn print_hello_rust() {
    println!("Hello from Rust!");
}

fn is_from_rust() -> bool {
    true
}

fn get_hello_rust() -> String {
    String::from("Hello Rust!")
}