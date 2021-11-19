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
    
    func as_slice () -> UnsafeBufferPointer<UInt8> {
        UnsafeBufferPointer(start: self.as_ptr(), count: Int(self.len()))
    }
}

