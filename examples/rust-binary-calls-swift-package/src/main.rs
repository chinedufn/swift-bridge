fn main() {
    let start_num = 100;

    println!("The Rust starting number is {start_num}.");

    let num = ffi::swift_multiply_by_4(start_num);

    println!("Printing the number from Rust...");
    println!("The number is now {num}.")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_double_number() {
        assert_eq!(rust_double_number(2), 4);
    }

    #[test]
    fn test_swift_multiply_by_4() {
        assert_eq!(ffi::swift_multiply_by_4(2), 8);
    }
}
