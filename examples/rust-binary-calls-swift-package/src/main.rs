fn main() {
    let start_num = 100;

    println!("The Rust starting number is {}.", start_num);

    let num = ffi::swift_multiply_by_4(start_num);

    println!("Printing the number from Rust...");
    println!("The number is now {}.", num)
}

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_double_number(num: i64) -> i64;
    }

    extern "Swift" {
        fn swift_multiply_by_4(num: i64) -> i64;
    }
}

fn rust_double_number(num: i64) -> i64 {
    println!("Rust double function called...");

    num * 2
}
