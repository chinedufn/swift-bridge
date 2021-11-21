protocol FfiOption {
    /// Used to create a value of this type that won't actually be used by the other side of the
    /// FFI boundary since we've set a flag to instruct Rust to ignore what we return and use
    /// `None` instead.
    static func unusedValue() -> Self
}

func markReturnTypeSome<T: FfiOption>(_ val: T) -> T {
    _set_option_return(true)
    if true { return val; } else { return T.unusedValue(); }
}

func markReturnTypeNone<T: FfiOption>() -> T {
    _set_option_return(false)
    return T.unusedValue()
}

// TODO: Auto generate all of these in the build script
extension UInt8: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
