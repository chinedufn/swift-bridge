@_cdecl("__swift_bridge__$ASwiftStack$new")
func __swift_bridge__ASwiftStack_new () -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(ASwiftStack()).toOpaque()
}
@_cdecl("__swift_bridge__$ASwiftStack$push")
func __swift_bridge__ASwiftStack_push (_ this: UnsafeMutableRawPointer, _ val: UInt8) {
    Unmanaged<ASwiftStack>.fromOpaque(this).takeUnretainedValue().push(val: val)
}
@_cdecl("__swift_bridge__$ASwiftStack$pop")
func __swift_bridge__ASwiftStack_pop (_ this: UnsafeMutableRawPointer) {
    Unmanaged<ASwiftStack>.fromOpaque(this).takeUnretainedValue().pop()
}
@_cdecl("__swift_bridge__$ASwiftStack$as_ptr")
func __swift_bridge__ASwiftStack_as_ptr (_ this: UnsafeMutableRawPointer) -> UnsafeMutablePointer<UInt8> {
    Unmanaged<ASwiftStack>.fromOpaque(this).takeUnretainedValue().as_ptr()
}
@_cdecl("__swift_bridge__$ASwiftStack$len")
func __swift_bridge__ASwiftStack_len (_ this: UnsafeMutableRawPointer) -> UInt {
    Unmanaged<ASwiftStack>.fromOpaque(this).takeUnretainedValue().len()
}
@_cdecl("__swift_bridge__$ASwiftStack$as_slice")
func __swift_bridge__ASwiftStack_as_slice (_ this: UnsafeMutableRawPointer) -> RustSlice_uint8_t {
    let buffer_pointer = Unmanaged<ASwiftStack>.fromOpaque(this).takeUnretainedValue().as_slice()
    return RustSlice_uint8_t(start: UnsafeMutablePointer(mutating: buffer_pointer.baseAddress), len: UInt(buffer_pointer.count))
}
@_cdecl("__swift_bridge__$ASwiftStack$_free")
func __swift_bridge__ASwiftStack__free (ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<ASwiftStack>.fromOpaque(ptr).takeRetainedValue()
}


