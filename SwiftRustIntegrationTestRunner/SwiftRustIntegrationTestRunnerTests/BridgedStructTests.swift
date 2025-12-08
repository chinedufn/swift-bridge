//
//  BridgedStructTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Tests for the #[swift_bridge::bridged] attribute macro.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class BridgedStructTests: XCTestCase {

    // Test creating a bridged struct from Rust
    func testRustCreatesBridgedResponse() throws {
        let response = rust_create_bridged_response(true, 42)
        XCTAssertTrue(response.success)
        XCTAssertEqual(response.count, 42)
    }

    // Test passing a bridged struct to Rust
    func testRustReceivesBridgedResponse() throws {
        let response = BridgedResponse(success: true, count: 10)
        let result = rust_receive_bridged_response(response)
        XCTAssertTrue(result)

        let failResponse = BridgedResponse(success: false, count: 0)
        let failResult = rust_receive_bridged_response(failResponse)
        XCTAssertFalse(failResult)
    }

    // Test bridged struct with String field
    func testRustCreatesBridgedMessage() throws {
        let message = rust_create_bridged_message("Hello".intoRustString(), -42)
        XCTAssertEqual(message.code, -42)
    }

    // Test Result<BridgedStruct, Error> return - success case
    func testRustFallibleBridgedResponseSuccess() throws {
        let result = try rust_fallible_bridged_response(true)
        XCTAssertTrue(result.success)
        XCTAssertEqual(result.count, 42)
    }

    // Test Result<BridgedStruct, Error> return - error case
    func testRustFallibleBridgedResponseError() throws {
        do {
            _ = try rust_fallible_bridged_response(false)
            XCTFail("Expected error to be thrown")
        } catch let error as BridgedError {
            switch error {
            case .InvalidInput(let msg):
                XCTAssertEqual(msg.toString(), "test error")
            default:
                XCTFail("Wrong error variant")
            }
        }
    }

    // Test Rust calling Swift with bridged struct
    func testRustCallsSwiftBridgedResponse() throws {
        test_rust_calls_swift_bridged_response()
    }

    // Test Rust calling Swift fallible function
    func testRustCallsSwiftFallibleBridged() throws {
        test_rust_calls_swift_fallible_bridged()
    }

    // MARK: - Optional Field Tests

    // Test optional primitive field with Some value
    func testOptionalPrimitiveSome() throws {
        let opt = rust_create_optional_primitive_some(42)
        XCTAssertNotNil(opt.value)
        XCTAssertEqual(opt.value, 42)
    }

    // Test optional primitive field with None value
    func testOptionalPrimitiveNone() throws {
        let opt = rust_create_optional_primitive_none()
        XCTAssertNil(opt.value)
    }

    // Test passing optional primitive to Rust
    func testRustReceivesOptionalPrimitive() throws {
        let some = BridgedOptionalPrimitive(value: 100)
        XCTAssertEqual(rust_receive_optional_primitive(some), 100)

        let none = BridgedOptionalPrimitive(value: nil)
        XCTAssertEqual(rust_receive_optional_primitive(none), -1)
    }
}
