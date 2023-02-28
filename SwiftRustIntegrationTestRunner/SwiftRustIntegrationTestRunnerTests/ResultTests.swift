//
//  ResultTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 9/20/22.

import XCTest
@testable import SwiftRustIntegrationTestRunner

extension ResultTransparentEnum: Error {}
extension ResultTransparentEnum: @unchecked Sendable {}

class ResultTests: XCTestCase {
    /// Verify that we can pass a Result<String, String> from Swift -> Rust
    func testSwiftCallRustResultString() throws {
        rust_func_takes_result_string(.Ok("Success Message"))
        rust_func_takes_result_string(.Err("Error Message"))
    }
    
    /// Verify that we can pass a Result<OpaqueRust, OpaqueRust> from Swift -> Rust
    func testSwiftCallRustResultOpaqueRust() throws {
        let reflectedOk = try! rust_func_reflect_result_opaque_rust(
            .Ok(ResultTestOpaqueRustType(111))
        )
        XCTAssertEqual(reflectedOk.val(), 111)
        
        do {
            let _ = try rust_func_reflect_result_opaque_rust(
                .Err(ResultTestOpaqueRustType(222))
            )
            XCTFail("The function should have returned an error.")
        } catch let error as ResultTestOpaqueRustType {
            XCTAssertEqual(error.val(), 222)
        }
    }
    
    /// Verify that we can pass a Result<OpaqueSwift, OpaqueSwift> from Swift -> Rust
    func testSwiftCallRustResultOpaqueSwift() throws {
        rust_func_takes_result_opaque_swift(
            .Ok(ResultTestOpaqueSwiftType(val: 555))
        )
        rust_func_takes_result_opaque_swift(
            .Err(ResultTestOpaqueSwiftType(val: 666))
        )
    }

    /// Verify that we can receive a Result<(), OpaqueRust> from Rust
    func testSwiftCallRustResultNullOpaqueRust() throws {
        try! rust_func_return_result_null_opaque_rust(true)

        do {
            try rust_func_return_result_null_opaque_rust(false)
            XCTFail("The function should have returned an error.")
        } catch let error as ResultTestOpaqueRustType {
            XCTAssertEqual(error.val(), 222)
        }
    }

    /// Verify that we can receive a Result<UnitStruct, OpaqueRust> from Rust
    func testSwiftCallRustResultUnitStructOpaqueRust() throws {
        try! rust_func_return_result_unit_struct_opaque_rust(true)

        do {
            try rust_func_return_result_unit_struct_opaque_rust(false)
            XCTFail("The function should have returned an error.")
        } catch let error as ResultTestOpaqueRustType {
            XCTAssertEqual(error.val(), 222)
        }
    }
    
    func testResultTransparentEnumOpaqueRust() throws {
        do {
            let _ = try rust_func_return_result_transparent_enum_opaque_rust(false)
            XCTFail("The function should have returned an error.")
        } catch let error as ResultTransparentEnum {
            switch error {
            case .NamedField(let data):
                XCTAssertEqual(data, 123)
            case .UnnamedFields(_, _):
                XCTFail()
            case .NoFields:
                XCTFail()
            }
        } catch {
            XCTFail()
        }
    }
}
