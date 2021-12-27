//
//  RustTests.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/14/21.
//

import Foundation

public class ASwiftStack {
    private var stack: [UInt8] = []
    
    func push (val: UInt8) {
        stack.append(val)
    }
    
    func pop () {
        let _ = stack.popLast();
    }
    
    func as_ptr() -> UnsafePointer<UInt8> {
        UnsafePointer(self.stack)
    }
    
    func len () -> UInt {
        UInt(stack.count)
    }
    
    func as_slice () -> UnsafeBufferPointer<UInt8> {
        UnsafeBufferPointer(start: self.as_ptr(), count: Int(self.len()))
    }
}
