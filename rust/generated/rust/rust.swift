func print_hello_rust() {
    __swift_bridge__$print_hello_rust()
}
func is_from_rust() -> Bool {
    __swift_bridge__$is_from_rust()
}
func get_hello_rust() -> RustString {
    RustString(ptr: __swift_bridge__$get_hello_rust())
}


