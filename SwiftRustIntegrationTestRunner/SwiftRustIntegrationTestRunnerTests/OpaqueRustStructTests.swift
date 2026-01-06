//
//  OpaqueRustStructTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/14/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

@MainActor
class OpaqueRustStructTests: XCTestCase {
    
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    /// Verify that we can create and work with an opaque Rust struct via a generated class
    func testOpaqueRustStruct() throws {
        let stack = ARustStack()
        
        XCTAssertEqual(stack.len(), 0)
        stack.push(5)
        stack.push(10)
        XCTAssertEqual(stack.len(), 2)
        
        XCTAssertEqual(stack.as_slice()[0], 5)
        XCTAssertEqual(stack.as_slice()[1], 10)
        
        stack.pop()
        XCTAssertEqual(stack.len(), 1)
    }
    
    /// Verify that when we de-alocate a class instance that is wrapping a type that was returned to us from
    /// Rust by reference we do not free the Rust type's memory (like we do with owned values).
    ///
    /// We call a function twice that returns the same &ARustStack each time.
    ///
    /// This means that we create two class instances on the Swift side that each reference the
    /// same &ARustStack.
    ///
    /// By calling it twice we ensures that we did not free memory after dropping these instances
    /// since that would result in a malloc error "pointer being freed was not allocated".
    func testReferenceToOpaqueRustStruct() throws {
        let stack_wrapper = StackWrapper()
        
        let ref1 = stack_wrapper.get_stack_mut()
        let ref2 = stack_wrapper.get_stack_mut()
        
        ref1.push(5)
        ref2.push(10)
        
        XCTAssertEqual(ref1.len(), 2)
        XCTAssertEqual(ref1.len(), ref2.len())
    }
    
    /// Verify that we can pass a Copy opaque Rust type between Rust and Swift.
    func testOpaqueRustTypeImplCopy() throws {
        let val = RustCopyType()
        let val2 = RustCopyType()
        
        // Because `val` is copy we can still use it after calling
        // a method that takes an owned `self` .
        val.consume()
        
        XCTAssert(val.eq(val2))
    }

    func testOpaqueRustTypeImplEquatable() throws {
        XCTContext.runActivity(named: "Should be equal"){
            _ in
            let val1 = RustEquatableType()
            let val2 = RustEquatableType()

            XCTAssertEqual(val1, val2)
        }

        XCTContext.runActivity(named: "Should not be equal"){
            _ in
            let val1 = RustEquatableType()
            let val2 = RustEquatableType()

            val1.set_value(11)
            val2.set_value(22)

            XCTAssertNotEqual(val1, val2)
        }
    }

    func testOpaqueRustCopyTypeImplEquatable() throws {
        XCTContext.runActivity(named: "Should be equal"){
            _ in
            let val1 = RustCopyEquatableType()
            let val2 = RustCopyEquatableType()

            XCTAssert(val1.eq(val2))   // Direct call to exposed method
            XCTAssertEqual(val1, val2) // via Equatable
        }

        XCTContext.runActivity(named: "Should not be equal"){
            _ in
            let val1 = RustCopyEquatableType()
            let val2 = RustCopyEquatableType(withFirstValue: 87)

            XCTAssertFalse(val1.eq(val2)) // Direct call to exposed method
            XCTAssertNotEqual(val1, val2) // via Equatable
        }
    }

    func testOpaqueRustTypeImplHashable() throws {
        XCTContext.runActivity(named: "Same hash value"){
            _ in
            let val1 = RustHashableType(10)
            let val2 = RustHashableType(10)

            var table: [RustHashableType: String] = [:]
            table[val1] = "hello"
            table[val2] = "world"

            //Should be overwritten.
            if let element = table[val1] {
                XCTAssertEqual(element, "world")
            }else {
                XCTFail()
            }
        }

        XCTContext.runActivity(named: "Not same hash value"){
            _ in
            let val1 = RustHashableType(10)
            let val2 = RustHashableType(100)

            var table: [RustHashableType: String] = [:]
            table[val1] = "hello"
            table[val2] = "world"
            
            //Should not be overwritten
            if let element = table[val1] {
                XCTAssertEqual(element, "hello")
            }else {
                XCTFail()
            }
            if let element = table[val2] {
                XCTAssertEqual(element, "world")
            }else {
                XCTFail()
            }
        }
    }
    
    func testOpaqueRustCopyTypeImplHashable() throws {
        XCTContext.runActivity(named: "Same hash value"){
            _ in
            let val1 = RustCopyHashableType(10)
            let val2 = RustCopyHashableType(10)
            
            var table: [RustCopyHashableType: String] = [:]
            table[val1] = "hello"
            table[val2] = "world"
            
            //Should be overwritten.
            if let element = table[val1] {
                XCTAssertEqual(element, "world")
            }else {
                XCTFail()
            }
        }
        
        XCTContext.runActivity(named: "Not same hash value"){
            _ in
            let val1 = RustCopyHashableType(10)
            let val2 = RustCopyHashableType(100)
            
            var table: [RustCopyHashableType: String] = [:]
            table[val1] = "hello"
            table[val2] = "world"
            
            //Should not be overwritten
            if let element = table[val1] {
                XCTAssertEqual(element, "hello")
            }else {
                XCTFail()
            }
            if let element = table[val2] {
                XCTAssertEqual(element, "world")
            }else {
                XCTFail()
            }
        }
    }
}

