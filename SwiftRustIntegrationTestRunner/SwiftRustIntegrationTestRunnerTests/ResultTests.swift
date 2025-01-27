//
//  ResultTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 9/20/22.

import XCTest
@testable import SwiftRustIntegrationTestRunner

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
}
