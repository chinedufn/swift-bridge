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

class FooRef {
    var ptr: UInt8
    
    func aRefSelfFunc() {
    }
    
    init(ptr: UInt8) {
        self.ptr = ptr
    }
}

class FooRefMut: FooRef {
    func aRefMutableSelfFunc() {
        self.ptr += 1
    }
}

class Foo: FooRefMut {
    func anOwnedSelfFunc() {
    }
    
    deinit {
        ptr += 1
    }
}

func callWithFoo(foo: FooRef) {
}

func scratchPad () {
    let foo = Foo(ptr: 5)
    let fooRef = FooRef(ptr: 10)
    let fooRefMut = FooRefMut(ptr: 15)
    
    callWithFoo(foo: foo)
    callWithFoo(foo: fooRef)
    callWithFoo(foo: fooRefMut)
    
    foo.anOwnedSelfFunc()
    foo.aRefSelfFunc()
    foo.aRefMutableSelfFunc()
    
    fooRef.aRefSelfFunc()
    
    fooRefMut.aRefMutableSelfFunc()
    fooRefMut.aRefSelfFunc()
}
