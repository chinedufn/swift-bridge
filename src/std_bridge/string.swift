import Foundation

class SwiftString {
    var string: String

    init() {
        string = ""
    }

    init(str: RustStr) {
        string = String(bytes: str.toBufferPointer(), encoding: .utf8)!
    }

    func as_ptr() -> UnsafePointer<UInt8> {
        // TODO: Does the utf8String have the same lifetime as our String?
        //  If not this can lead to undefined behavior..
        let ptr = UnsafeRawPointer((self.string as NSString).utf8String)!
        let start = ptr.assumingMemoryBound(to: UInt8.self)
        return start
    }

    func len () -> UInt {
        UInt(string.count)
    }
}

extension RustString {
    func toString() -> String {
        let str = self.as_str()
        let string = str.toString()

        return string
    }
}

extension RustStr {
    func toBufferPointer() -> UnsafeBufferPointer<UInt8> {
        let bytes = UnsafeBufferPointer(start: self.start, count: Int(self.len))
        return bytes
    }

    func toString() -> String {
        let bytes = self.toBufferPointer()
        return String(bytes: bytes, encoding: .utf8)!
    }
}