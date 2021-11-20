class SwiftString {
    var string: String

    init() {
        string = ""
    }

    init(str: RustStr) {
        string = String(bytes: str.toBufferPointer(), encoding: .utf8)!
    }

    func as_ptr() -> UnsafePointer<UInt8> {
        let buf: [UInt8] = Array(string.utf8)
        return UnsafePointer(buf)
    }

    func len () -> UInt {
        UInt(string.count)
    }
}

extension RustStr {
    func toBufferPointer() -> UnsafeBufferPointer<UInt8> {
        UnsafeBufferPointer(start: self.start, count: Int(self.len))
    }
}