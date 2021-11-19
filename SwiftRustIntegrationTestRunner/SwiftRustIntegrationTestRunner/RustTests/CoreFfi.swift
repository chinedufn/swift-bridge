//
//  CoreFfi.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/14/21.
//

import Foundation

class ScratchPad {
    var string: String
    
    init (str: UnsafeBufferPointer<UInt8>) {
        string = String(bytes: str, encoding: .utf8)!
    }
}

//public class SwiftString {
//    fileprivate var string: String
//
//    init() {
//        string = ""
//    }
//
//    func len () -> UInt {
//        UInt(string.count)
//    }
//}
//
//
//@_cdecl("swift_bridge$unstable$swift_string$new")
//public func swift_string_new(ptr: UnsafeMutablePointer<UInt8>, len: UInt) -> UnsafeMutableRawPointer {
//    let bytes = UnsafeBufferPointer(start: ptr, count: Int(len))
//
//    let string = String(bytes: bytes, encoding: .utf8)!
//
//    let swiftString: SwiftString = SwiftString()
//    swiftString.string = string
//
//    return Unmanaged.passRetained(swiftString).toOpaque()
//}
//
//@_cdecl("swift_bridge$unstable$swift_string$ptr")
//public func swift_string_ptr (string: UnsafeRawPointer) -> UnsafeMutablePointer<UInt8> {
//    let swiftString: SwiftString = Unmanaged.fromOpaque(string).takeUnretainedValue()
//
//    let buf: [UInt8] = Array(swiftString.string.utf8)
//
//    return UnsafeMutablePointer(mutating: buf)
//}
//
//@_cdecl("swift_bridge$unstable$swift_string$length")
//public func swift_string_length (string: UnsafeRawPointer) -> UInt {
//    let swiftString: SwiftString = Unmanaged.fromOpaque(string).takeUnretainedValue()
//    return UInt(swiftString.string.count)
//}
//
//@_cdecl("swift_bridge$unstable$swift_string$free")
//public func swift_string_free (string: UnsafeMutableRawPointer) {
//    let _ = Unmanaged<SwiftString>.fromOpaque(string).takeRetainedValue()
//}
