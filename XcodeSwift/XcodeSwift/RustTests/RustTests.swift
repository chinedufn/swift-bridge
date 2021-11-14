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

// FIXME: Rename to ARustStack when we generate this code.. have the user facing types Swift types use
//  real type names, and C header underlying types use
public class ARustStack {
    private var ownedPtr: OwnedPtrToRust
    private var refPtr: RefPtrToRust
    
    init() {
        ownedPtr = swift_bridge$unstable$ARustStruct$new()
        refPtr = RefPtrToRust(ptr: ownedPtr.ptr)
    }
    
    deinit{
        swift_bridge$unstable$ARustStruct$free(ownedPtr)
    }
    
    func push (_ val: UInt8) {
        swift_bridge$unstable$ARustStruct$push(refPtr, val)
    }
    
    func pop () {
        swift_bridge$unstable$ARustStruct$pop(refPtr)
    }
    
    func len () -> UInt {
        swift_bridge$unstable$ARustStruct$len(refPtr)
    }
    
    func asUnsafeBufferPointer () -> UnsafeBufferPointer<UInt8> {
        let start = swift_bridge$unstable$ARustStruct$as_ptr(refPtr)
        let len = self.len()
        
        let buf = UnsafeBufferPointer(start: start, count: Int(len))
        
        for b in buf {
            print(b)
        }
        
        return buf
    }
}
