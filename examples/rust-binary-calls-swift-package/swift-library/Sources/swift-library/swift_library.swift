func swift_multiply_by_4(num: Int64) -> Int64 {
    print("Starting Swift multiply by 4 function...")

    print("Calling the Rust double function twice in order to 4x our number...")
    let double = rust_double_number(num);
    let four_times = rust_double_number(double);

    print("Leaving Swift multiply by 4 function...")
    return four_times
}