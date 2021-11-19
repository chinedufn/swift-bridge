//
//  RustTests.swift
//  XcodeSwift
//
//  Created by Frankie Nwafili on 11/14/21.
//

import Foundation

public func runStringTests () {
    run_string_tests()
}

public func runOpaqueSwiftClassTests () {
    run_opaque_swift_class_tests()
}

public class ASwiftStack {
    private var stack: [UInt8] = []
    
    func push (val: UInt8) {
        stack.append(val)
    }
    
    func pop () {
        let _ = stack.popLast();
    }
    
    func as_ptr() -> UnsafeMutablePointer<UInt8> {
        UnsafeMutablePointer(mutating: self.stack)
    }
    
    func len () -> UInt {
        UInt(stack.count)
    }
}

//@_cdecl("swift_bridge$unstable$freestanding$new")
//func new () -> UnsafeMutableRawPointer {
//    Unmanaged.passRetained(ASwiftStack()).toOpaque()
//}

//@_cdecl("swift_bridge$unstable$ASwiftStack$push")
//func push (this: UnsafeMutableRawPointer, val: UInt8) {
//    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
//    stack.push(val)
//}

//@_cdecl("swift_bridge$unstable$ASwiftStack$pop")
//func pop (this: UnsafeMutableRawPointer) {
//    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
//    stack.pop()
//}
//
//@_cdecl("swift_bridge$unstable$ASwiftStack$as_ptr")
//func as_ptr(this: UnsafeMutableRawPointer) -> UnsafeMutablePointer<UInt8> {
//    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
//    return stack.asPtr()
//}
//
//@_cdecl("swift_bridge$unstable$ASwiftStack$len")
//func len(this: UnsafeMutableRawPointer) -> UInt {
//    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
//    return stack.len()
//}

