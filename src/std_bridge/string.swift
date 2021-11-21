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

extension String {
    func toRustStr() -> RustStr {
        let buf: [UInt8] = Array(self.utf8)
        let start = UnsafeMutablePointer(mutating: buf)
        let len = UInt(self.count)
        return RustStr(start: start, len: len)
    }
}
