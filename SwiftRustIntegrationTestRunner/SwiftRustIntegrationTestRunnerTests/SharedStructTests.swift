//
//  ExternalCTypes.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/24/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class SharedStructTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    /// Run all of the tests that are defined on the Rust side in
    /// crates/swift-integration-tests/src/shared_types/shared_struct.rs
    func testRust() {
        test_rust_calls_swift()
    }
    
    func testStructWithNoFields() {
        let _: StructWithNoFields = swift_calls_rust_struct_with_no_fields(StructWithNoFields())
    }
    
    func testStructReprStructWithOnePrimitiveField() {
        let val = swift_calls_rust_struct_repr_struct_one_primitive_field(
            StructReprStructWithOnePrimitiveField(named_field: 56)
        );
        XCTAssertEqual(val.named_field, 56)
    }
    
    /// Verify that we can create a tuple struct.
    func testTupleStruct() {
        let val = StructReprStructTupleStruct(_0: 11, _1: 22)
        let reflected = swift_calls_rust_tuple_struct(val)
        
        XCTAssertEqual(val._0, reflected._0)
        XCTAssertEqual(val._1, reflected._1)
    }
}
