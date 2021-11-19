class SwiftString {
    var string: String

    init() {
        string = ""
    }

    init(str: UnsafeBufferPointer<UInt8>) {
        string = String(bytes: str, encoding: .utf8)!
    }

    func as_ptr() -> UnsafePointer<UInt8> {
        let buf: [UInt8] = Array(string.utf8)
        return UnsafePointer(buf)
    }

    func len () -> UInt {
        UInt(string.count)
    }
}