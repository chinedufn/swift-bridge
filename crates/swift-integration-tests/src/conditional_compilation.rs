#[swift_bridge::bridge]
#[cfg(feature = "this_is_enabled")]
mod enabled {
    extern "Rust" {
        // This function will be exposed since this "this_is_enabled" feature is on by default.
        fn conditionally_exposed_fn() -> u8;
    }
}

#[swift_bridge::bridge]
#[cfg(feature = "this_is_not_enabled")]
mod not_enabled {
    extern "Rust" {
        // This function isn't actually defined at `super::undefined_fn`, but it doesn't matter
        // since this entire bridge module won't be compiled.
        fn undefined_fn();
    }
}

#[cfg(feature = "this_is_enabled")]
fn conditionally_exposed_fn() -> u8 {
    123
}
