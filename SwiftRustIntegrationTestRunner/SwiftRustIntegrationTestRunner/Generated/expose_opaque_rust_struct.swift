
public class ARustStack {
    private var ptr: UnsafeMutableRawPointer

    init() {
        ptr = __swift_bridge__$ARustStack$new()
    }

    deinit {
        __swift_bridge__$ARustStack$_free(ptr)
    }

    func push(_ val: UInt8) {
        __swift_bridge__$ARustStack$push(ptr, val)
    }

    func pop() {
        __swift_bridge__$ARustStack$pop(ptr)
    }

    func as_ptr() -> UnsafeMutablePointer<UInt8> {
        __swift_bridge__$ARustStack$as_ptr(ptr)
    }

    func len() -> UInt {
        __swift_bridge__$ARustStack$len(ptr)
    }

    func as_slice() -> UnsafeBufferPointer<UInt8> {
            let slice = __swift_bridge__$ARustStack$as_slice(ptr)
        return UnsafeBufferPointer(start: slice.start, count: Int(slice.len))
    }
} 


