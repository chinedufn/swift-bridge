public class RustString: RustStringRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RustString$_free(ptr)
        }
    }
}

/// Tested in:
///   SwiftRustIntegrationTestRunner/SwiftRustIntegrationTestRunnerTests/ResultTests.swift:
///  `func testSwiftCallRustReturnsResultString()`
extension RustString: Error {}

// THREAD SAFETY: `RustString`, `RustStringRef` and `RustStringRefMut` are safe to send across threads as long as the
// ownership and aliasing rules are followed.
// This is because the underlying Rust `std::string::String`, `&str` and `&mut str` are all `Send+Sync`.
// See the `Safety` chapter in the book for more information about memory and thread safety rules.
//
// For now we have implemented `Sendable` for `RustString`. If users need `RustStringRef` or `RustStringRefMut` to
// implement `Sendable` then we can implement those as well.
//
// Tested in:
//  `SwiftRustIntegrationTestRunner/SwiftRustIntegrationTestRunnerTests/SendableTests.swift`
//  `func testSendableRustString()`
extension RustString: @unchecked Sendable {}

extension RustString {
    public convenience init() {
        self.init(ptr: __swift_bridge__$RustString$new())
    }

    public convenience init<GenericToRustStr: ToRustStr>(_ str: GenericToRustStr) {
        self.init(ptr: str.toRustStr({ strAsRustStr in
            __swift_bridge__$RustString$new_with_str(strAsRustStr)
        }))
    }
}
public class RustStringRefMut: RustStringRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RustStringRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RustStringRef {
    public func len() -> UInt {
        __swift_bridge__$RustString$len(ptr)
    }

    public func as_str() -> RustStr {
        __swift_bridge__$RustString$as_str(ptr)
    }

    public func trim() -> RustStr {
        __swift_bridge__$RustString$trim(ptr)
    }
}
extension RustString: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RustString$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RustString$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RustString) {
        __swift_bridge__$Vec_RustString$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RustString$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RustString(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RustStringRef> {
        let pointer = __swift_bridge__$Vec_RustString$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RustStringRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RustStringRefMut> {
        let pointer = __swift_bridge__$Vec_RustString$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RustStringRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RustStringRef> {
        UnsafePointer<RustStringRef>(OpaquePointer(__swift_bridge__$Vec_RustString$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RustString$len(vecPtr)
    }
}
