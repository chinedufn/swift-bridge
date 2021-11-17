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

@_silgen_name("fooBar")
func fooBar () -> UInt8 {
    return 5
}

// FIXME: Rename to ARustStack when we generate this code.. have the user facing types Swift types use
//  real type names, and C header underlying types use
//public class ARustStack {
//    private var ownedPtr: OwnedPtrToRust
//    private var refPtr: RefPtrToRust
//
//    init() {
//        ownedPtr = swift_bridge$unstable$ARustStruct$new()
//        refPtr = RefPtrToRust(ptr: ownedPtr.ptr)
//    }
//
//    deinit{
//        swift_bridge$unstable$ARustStruct$free(ownedPtr)
//    }
//
//    func push (_ val: UInt8) {
//        swift_bridge$unstable$ARustStruct$push(refPtr, val)
//    }
//
//    func pop () {
//        swift_bridge$unstable$ARustStruct$pop(refPtr)
//    }
//
//    func len () -> UInt {
//        swift_bridge$unstable$ARustStruct$len(refPtr)
//    }
//
//    func asUnsafeBufferPointer () -> UnsafeBufferPointer<UInt8> {
//        let start = swift_bridge$unstable$ARustStruct$as_ptr(refPtr)
//        let len = self.len()
//
//        let buf = UnsafeBufferPointer(start: start, count: Int(len))
//
//        for b in buf {
//            print(b)
//        }
//
//        return buf
//    }
//}


public class ASwiftStack {
    private var stack: [UInt8] = []
    
    func push (_ val: UInt8) {
        stack.append(val)
    }
    
    func pop () {
        let _ = stack.popLast();
    }
    
    func asPtr() -> UnsafeMutablePointer<UInt8> {
        UnsafeMutablePointer(mutating: self.stack)
    }
    
    func len () -> UInt {
        UInt(stack.count)
    }
}

@_cdecl("swift_bridge$unstable$freestanding$new")
func new () -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(ASwiftStack()).toOpaque()
}

@_cdecl("swift_bridge$unstable$ASwiftStack$push")
func push (this: UnsafeMutableRawPointer, val: UInt8) {
    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
    stack.push(val)
}

@_cdecl("swift_bridge$unstable$ASwiftStack$pop")
func pop (this: UnsafeMutableRawPointer) {
    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
    stack.pop()
}

@_cdecl("swift_bridge$unstable$ASwiftStack$as_ptr")
func as_ptr(this: UnsafeMutableRawPointer) -> UnsafeMutablePointer<UInt8> {
    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
    return stack.asPtr()
}

@_cdecl("swift_bridge$unstable$ASwiftStack$len")
func len(this: UnsafeMutableRawPointer) -> UInt {
    let stack: ASwiftStack = Unmanaged.fromOpaque(this).takeUnretainedValue()
    return stack.len()
}

// Scratchpad
public class Foo {
    private var ptr: OwnedPtrToRust
    
    init() {
        fatalError("No swift_bridge(constructor) attribute provided.")
    }
}
