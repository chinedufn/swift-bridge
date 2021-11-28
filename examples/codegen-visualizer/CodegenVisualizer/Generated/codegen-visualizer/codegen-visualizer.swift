@_cdecl("__swift_bridge__$GeneratedCodeHolder$set_generated_rust")
func __swift_bridge__GeneratedCodeHolder_set_generated_rust (_ this: UnsafeMutableRawPointer, _ rust: RustStr) {
    Unmanaged<GeneratedCodeHolder>.fromOpaque(this).takeUnretainedValue().setGeneratedRust(rust: rust)
}

@_cdecl("__swift_bridge__$GeneratedCodeHolder$set_generated_swift")
func __swift_bridge__GeneratedCodeHolder_set_generated_swift (_ this: UnsafeMutableRawPointer, _ swift: RustStr) {
    Unmanaged<GeneratedCodeHolder>.fromOpaque(this).takeUnretainedValue().setGeneratedSwift(swift: swift)
}

@_cdecl("__swift_bridge__$GeneratedCodeHolder$set_generated_c_header")
func __swift_bridge__GeneratedCodeHolder_set_generated_c_header (_ this: UnsafeMutableRawPointer, _ c: RustStr) {
    Unmanaged<GeneratedCodeHolder>.fromOpaque(this).takeUnretainedValue().setGeneratedCHeader(c: c)
}

@_cdecl("__swift_bridge__$GeneratedCodeHolder$set_error_message")
func __swift_bridge__GeneratedCodeHolder_set_error_message (_ this: UnsafeMutableRawPointer, _ error: RustStr) {
    Unmanaged<GeneratedCodeHolder>.fromOpaque(this).takeUnretainedValue().setErrorMessage(error: error)
}


public class RustApp {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init(_ generated_code_holder: GeneratedCodeHolder) {
        ptr = __swift_bridge__$RustApp$new(Unmanaged.passRetained(generated_code_holder).toOpaque())
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$RustApp$_free(ptr)
        }
    }

    func start_generated_rust_code_formatter_thread() {
        __swift_bridge__$RustApp$start_generated_rust_code_formatter_thread(ptr)
    }

    func generate_swift_bridge_code(_ bridge_module_source: RustStr) {
        __swift_bridge__$RustApp$generate_swift_bridge_code(ptr, bridge_module_source)
    }
}

@_cdecl("__swift_bridge__$GeneratedCodeHolder$_free")
func __swift_bridge__GeneratedCodeHolder__free (ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<GeneratedCodeHolder>.fromOpaque(ptr).takeRetainedValue()
}



