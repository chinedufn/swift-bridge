//
//  ResultTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 9/20/22.

import XCTest
@testable import SwiftRustIntegrationTestRunner

@MainActor
class ResultTests: XCTestCase {
    /// Verify that we can pass a Result<String, String> from Swift -> Rust
    func testSwiftCallRustResultString() throws {
        rust_func_takes_result_string(.Ok("Success Message"))
        rust_func_takes_result_string(.Err("Error Message"))
    }

    /// Verify that we can return a Result<String, String> from Rust -> Swift.
    ///
    /// The Err case evidences Swift’s `Error` protocol is implemented correctly
    /// for `RustStringRef`, i.e. `extension RustStringRef: Error {}`
    func testSwiftCallRustReturnsResultString() throws {
        let resultOk = try! rust_func_returns_result_string(true)
        XCTAssertEqual(resultOk.toString(), "Success Message")

        do {
            let _ = try rust_func_returns_result_string(false)
            XCTFail("The function should have returned an error.")
        } catch let error as RustString {
            XCTAssertEqual(error.toString(), "Error Message")
        }
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

    /// Verify that we can receive a Result<OpaqueRust, TransparentEnum> from Rust
    func testResultOpaqueRustTransparentEnum() throws {
        XCTContext.runActivity(named: "Should return a ResultTestOpaqueRustType") {
            _ in
            do {
                let _ :ResultTestOpaqueRustType = try rust_func_return_result_opaque_rust_transparent_enum(true)
            } catch {
                XCTFail()
            }
        }

        XCTContext.runActivity(named: "Should throw an error") {
            _ in
            do {
                let _: ResultTestOpaqueRustType = try rust_func_return_result_opaque_rust_transparent_enum(false)
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

    /// Verify that we can receive a Result<TransparentEnum, OpaqueRust> from Rust
    func testResultTransparentEnumOpaqueRust() throws {
        XCTContext.runActivity(named: "Should return a ResultTestOpaqueRustType") {
            _ in
            do {
                let resultTransparentEnum : ResultTransparentEnum = try rust_func_return_result_transparent_enum_opaque_rust(true)
                switch resultTransparentEnum {
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

        XCTContext.runActivity(named: "Should throw an error") {
            _ in
            do {
                let _: ResultTransparentEnum = try rust_func_return_result_transparent_enum_opaque_rust(false)
                XCTFail("The function should have returned an error.")
            } catch _ as ResultTestOpaqueRustType {

            } catch {
                XCTFail()
            }
        }
    }

    /// Verify that we can receive a Result<(), TransparentEnum> from Rust
    func testResultUnitTypeTransparentEnum() throws {
        XCTContext.runActivity(named: "Should return a Unit type") {
            _ in
            do {
                let _ :() = try rust_func_return_result_unit_type_enum_opaque_rust(true)
            } catch {
                XCTFail()
            }
        }

        XCTContext.runActivity(named: "Should throw an error") {
            _ in
            do {
                let _ :() = try rust_func_return_result_unit_type_enum_opaque_rust(false)
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

    /// Verify that we can receive a Result<(primitive type, OpaqueRustType, String), TransparentEnum> from Rust
    func testResultTupleTransparentEnum() throws {
        XCTContext.runActivity(named: "Should return a tuple type") {
            _ in
            do {
                let tuple: (Int32, ResultTestOpaqueRustType, RustString) = try rust_func_return_result_tuple_transparent_enum(true)
                XCTAssertEqual(tuple.0, 123)
                XCTAssertEqual(tuple.1.val(), ResultTestOpaqueRustType(123).val())
                XCTAssertEqual(tuple.2.toString(), "hello")
            } catch {
                XCTFail()
            }
        }

        XCTContext.runActivity(named: "Should throw an error") {
            _ in
            do {
                let _: (Int32, ResultTestOpaqueRustType, RustString) = try rust_func_return_result_tuple_transparent_enum(false)
                XCTFail("The function should have returned an error.")
            } catch let error as ResultTransparentEnum {
                switch error {
                case .NamedField(let data):
                    XCTAssertEqual(data, -123)
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

    /// Verify that we can receive a Result<(), TransparentStruct> from Rust
    func testResultNullTransparentStruct() throws {
        try! rust_func_return_result_null_transparent_struct(true)

        do {
            try rust_func_return_result_null_transparent_struct(false)
            XCTFail("The function should have returned an error.")
        } catch let error as ResultTransparentStruct {
            XCTAssertEqual(error.inner.toString(), "failed")
        }

        XCTContext.runActivity(named: "Should return a Unit type") {
            _ in
            do {
                let _ :() = try rust_func_return_result_unit_type_enum_opaque_rust(true)
            } catch {
                XCTFail()
            }
        }

        XCTContext.runActivity(named: "Should throw an error") {
            _ in
            do {
                let _ :() = try rust_func_return_result_unit_type_enum_opaque_rust(false)
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

    /// Verify that we can receive a Result<Vec<>, OpaqueRust> from Rust
    func testSwiftCallRustResultVecUInt32Rust() throws {
        let vec = try! rust_func_return_result_of_vec_u32()
        XCTAssertEqual(vec.len(), 3)
        for (i, value) in vec.enumerated() {
            XCTAssertEqual(UInt32(i), value)
        }
    }

    func testSwiftCallRustResultVecOpaqueRust() throws {
        let vec = try! rust_func_return_result_of_vec_opaque()
        XCTAssertEqual(vec.len(), 3)
        for (i, value) in vec.enumerated() {
            XCTAssertEqual(UInt32(i), value.val())
        }
    }

    /// Verify that we can use throwing initializers defined on the Rust side.
    func testThrowingInitializers() throws {
        XCTContext.runActivity(named: "Should fail") {
            _ in
            do {
                let throwingInitializer = try ThrowingInitializer(false)
            } catch let error as ResultTransparentEnum {
                if case .NamedField(data: -123) = error {
                    // This case should pass.
                } else {
                    XCTFail()
                }
            } catch {
                XCTFail()
            }
        }
        XCTContext.runActivity(named: "Should succeed") {
            _ in
            let throwingInitializer = try! ThrowingInitializer(true)
            XCTAssertEqual(throwingInitializer.val(), 123)
        }
    }

    // =========================================================================
    // Tests for sync Swift throwing functions called from Rust
    // =========================================================================

    /// Verify that Rust can call a sync Swift throwing function and receive Ok(u32)
    func testRustCallsSwiftSyncThrowsU32Ok() throws {
        let result = rust_calls_swift_sync_throws_u32_ok()
        XCTAssertEqual(result, 42)
    }

    /// Verify that Rust can call a sync Swift throwing function and receive Err
    func testRustCallsSwiftSyncThrowsU32Err() throws {
        let result = rust_calls_swift_sync_throws_u32_err()
        XCTAssertEqual(result, 123)
    }

    /// Verify that Rust can call a sync Swift throwing function and receive Ok(String)
    func testRustCallsSwiftSyncThrowsStringOk() throws {
        let result = rust_calls_swift_sync_throws_string_ok()
        XCTAssertEqual(result.toString(), "Success from Swift")
    }

    /// Verify that Rust can call a sync Swift throwing function and receive Err(String)
    func testRustCallsSwiftSyncThrowsStringErr() throws {
        let result = rust_calls_swift_sync_throws_string_err()
        XCTAssertEqual(result.toString(), "Error message from Swift")
    }

    /// Verify that Rust can call a sync Swift throwing function with void Ok type
    func testRustCallsSwiftSyncThrowsVoidOk() throws {
        let result = rust_calls_swift_sync_throws_void_ok()
        XCTAssertTrue(result)
    }

    /// Verify that Rust can call a sync Swift throwing function with void Ok and receive Err
    func testRustCallsSwiftSyncThrowsVoidErr() throws {
        let result = rust_calls_swift_sync_throws_void_err()
        XCTAssertEqual(result, 456)
    }

    /// Verify that Rust can call a sync Swift throwing function with opaque error and receive Ok
    func testRustCallsSwiftSyncThrowsOpaqueErrOk() throws {
        let result = rust_calls_swift_sync_throws_opaque_err_ok()
        XCTAssertEqual(result, 789)
    }

    /// Verify that Rust can call a sync Swift throwing function with opaque error and receive Err
    func testRustCallsSwiftSyncThrowsOpaqueErrErr() throws {
        let result = rust_calls_swift_sync_throws_opaque_err_err()
        XCTAssertEqual(result, 999)
    }

    /// Verify that Rust can call a sync Swift throwing function with opaque Ok and Err types and receive Ok
    func testRustCallsSwiftSyncThrowsOpaqueBothOk() throws {
        let result = rust_calls_swift_sync_throws_opaque_both_ok()
        XCTAssertEqual(result, 111)
    }

    /// Verify that Rust can call a sync Swift throwing function with opaque Ok and Err types and receive Err
    func testRustCallsSwiftSyncThrowsOpaqueBothErr() throws {
        let result = rust_calls_swift_sync_throws_opaque_both_err()
        XCTAssertEqual(result, 222)
    }
}
