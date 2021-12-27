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
        // We prevent the RustString from getting dropped until we've cloned the memory
        // into a Swift String
        let opaque = Unmanaged.passRetained(self)

        let str = self.as_str()
        let string = str.toString()

        let _ = opaque.takeRetainedValue()

        return string
    }
}

extension RustStr {
    func toBufferPointer() -> UnsafeBufferPointer<UInt8> {
        UnsafeBufferPointer(start: self.start, count: Int(self.len))
    }

    func toString() -> String {
        String(bytes: self.toBufferPointer(), encoding: .utf8)!
    }
}

import Foundation
extension String {
    func toRustStr() -> RustStr {
        // TODO: Does the utf8String have the same lifetime as our String?
        //  If not this can lead to undefined behavior..
        let ptr = UnsafeMutableRawPointer(mutating: (self as NSString).utf8String)!
        let start = ptr.assumingMemoryBound(to: UInt8.self)
        let len = UInt(self.count)
        return RustStr(start: start, len: len)
    }
}