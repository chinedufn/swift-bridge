
public class ASwiftStack {
    private var ptr: UnsafeMutableRawPointer

    init() {
        fatalError("No #[swift_bridge(constructor)] was defined in the extern Rust module.")
    }

    deinit {
        __swift_bridge__$ASwiftStack$_free(ptr)
    }

    class func new() -> UnsafeMutableRawPointer {
        __swift_bridge__$ASwiftStack$new()
    }

    func push(_ val: UInt8) {
        __swift_bridge__$ASwiftStack$push(ptr, val)
    }
} 


