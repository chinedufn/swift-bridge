// Bindings to types from libraries outside of our workspace.
// cbindgen can't generate headers for re-exported types

// Rust's uuid::Uuid, which is a [u8; 16] under the hood
// typedef struct pos_ffi_Uuid {
//     uint8_t uuid[16];
// } pos_ffi_Uuid;

// --------------------------------------------------

// Ok... so we want to be able to interface with types defined outside of swift-bridge.
//  So.. types defined in C headers.
// Say we have Uuid([u8; 16]).
// On the Rust side we can access that as uuid::Uuid;
// On the Swift side we can access it through a bridging header.
// The bridging header defines `struct Uuid { uint8_t uuid[16] }`
// On the Rust side we just care that our type is `#[repr(C)]`. The Rust compiler will enforce that.
// On the Swift side we just care that they type actually exists at all.
// So.. sometimes you'll have a type that is coming from something completely separate from Rust.
// Let's say you had a C++ Uuid library.
// So we want to be able to say `type Uuid = crate::path::to::Uuid`. In this case we wouldn't
// generate a Rust struct for `Uuid` since we're saying that we already have one.
// We also wouldn't pass opaque pointers over the boundary. We're treating this as a C type that can
// be safely passed across the boundary.
// On the Rust side we'd use `Uuid` as the type still.
// So really we just want to be able to say
// "Don't generate a Rust struct, we already have one".
// This isn't Swift code though.. It's a C type. `extern "Swift"` is for classes. So we should use
// `extern "C"`.
// So.. we're saying "Hey there's a type that I already have a C header for that I want to make use
// of.
// Now.. what if we do have that header file.. but we haven't added it to Swift? Ideally we're able
// to say "include this header"
// Not a problem though... since if the type isn't actually accessible to Swift then we'll get a
// compile time error.
// Ok so.. ideally we guarantee that the `Uuid` in Rust and the one in `Swift` both have the same
// layout.
// That would require us to generate the C header from swift-bridge code.. which we can't do because
// thie whole point is to support types that we don't control.
// So for now we'll just assume that they have the same layout and figure this out later..
//
// Ok .. lastly.. putting this type in `extern Swift` doesn't make much sense.
// It's really a C type.
// Ok so we have a C header file somewhere that defines some stuff that both Swift and Rust want to
// use.
// Yeah... and `extern "C"` block makes the most sense.
// Ok.. So `extern "C" { type Uuid = path::to::Uuid } would allow us to use `Uuid` within our
// module.
//
// # Implementation
//
// 1. (DONE) Create `mod existing_c_types`
//
// 2. (DONE) Create `extern "C" { type SomeExternalCType }`
//
// 3. (DONE) Create an `extern "Rust"` that uses it
//
// 4. (DONE) Create an `extern "Swift"` that uses it.
//
// 5. (DONE) Add tests to `ExternalCTypes.swift` that verify that the Uuid tests work.
//
// 6. Add some tests to the parser for parsing shared structs
//
// 7. Add parser test that an extern "Rust" block can use a shared type
//
// 8. Add parser test that an extern "Swift" block can use a shared type
//
// 9. Add test to to_tokens.rs where we have an extern "Rust" freestanding function that uses a
//    shared type
//
// 10. Add test to to_tokens.rs where we have an extern "Swift" freestanding function that uses a
//    shared type
//
// 11. Add test to generate_swift.rs where we have an extern "Rust" freestanding function that uses a
//    shared type
//
// 12. Add test to generate_swift.rs where we have an extern "Rust" freestanding function that uses a
//    shared type
//
// 13. Verify that our `ExternalCTypes.swift` tests pass.

#[swift_bridge::bridge]
mod ffi {
    struct SharedStruct {
        a: u8,
        b: u16,
        c: u32,
    }

    extern "Rust" {
        fn rust_return_existing_c_type() -> SharedStruct;

        fn rust_return_ref_existing_c_type() -> &SharedStruct;

        fn rust_return_mut_ref_existing_c_type() -> &mut SharedStruct;

        fn rust_receive_owned_existing_c_type(arg: SharedStruct);

        fn rust_receive_ref_existing_c_type(arg: &SharedStruct);

        fn rust_receive_ref_mut_existing_c_type(arg: &mut SharedStruct);
    }

    extern "Swift" {
        fn swift_return_existing_c_type() -> SharedStruct;

        fn swift_return_ref_existing_c_type() -> &SharedStruct;

        fn swift_return_mut_ref_existing_c_type() -> &mut SharedStruct;

        fn swift_receive_owned_existing_c_type(arg: SharedStruct);

        fn swift_receive_ref_existing_c_type(arg: &SharedStruct);

        fn swift_receive_ref_mut_existing_c_type(arg: &mut SharedStruct);
    }

    extern "Rust" {
        fn rust_run_shared_type_tests();
    }
}

fn rust_run_shared_type_tests() {
    test_swift_return_existing_c_type();
    test_swift_receive_ref_existing_c_type();
    test_swift_return_mut_ref_existing_c_type();
    test_swift_receive_owned_existing_c_type();
    test_swift_receive_ref_existing_c_type();
    test_swift_receive_ref_mut_existing_c_type();
}

fn test_swift_return_existing_c_type() {
    let _: SomeExistingCType = ffi::swift_return_existing_c_type();
}

fn test_swift_return_ref_existing_c_type() {
    let _: &SomeExistingCType = ffi::swift_return_ref_existing_c_type();
}

fn test_swift_return_mut_ref_existing_c_type() {
    let obj: &mut SomeExistingCType = ffi::swift_return_mut_ref_existing_c_type();
    obj.a = 111;
    obj.b = 222;
    obj.c = 333;
}

fn test_swift_receive_owned_existing_c_type() {
    let arg = SomeExistingCType {
        a: 10,
        b: 20,
        c: 30,
    };

    ffi::swift_receive_owned_existing_c_type(arg);
}

fn test_swift_receive_ref_existing_c_type() {
    let arg = SomeExistingCType {
        a: 40,
        b: 50,
        c: 60,
    };

    ffi::swift_receive_ref_existing_c_type(&arg);
}

fn test_swift_receive_ref_mut_existing_c_type() {
    let arg = SomeExistingCType {
        a: 70,
        b: 80,
        c: 90,
    };

    ffi::swift_receive_ref_mut_existing_c_type(&mut arg);

    assert_eq!(arg.a, 222);
    assert_eq!(arg.b, 333);
    assert_eq!(arg.c, 444);
}
