//
//  OpaqueRustStructTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/14/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

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
}

