//
//  PrimitiveTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 10/3/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for generic types such as `type SomeType<u32>`
class PrimitiveTests: XCTestCase {
    /// Run tests where Rust calls Swift functions that take primitive args.
    func testRustCallsSwiftPrimitives() throws {
        test_rust_calls_swift_primitives()
    }
    
    /// Run tests where Swift calls Rust functions that take primitive args.
    func testSwiftCallsRustPrimitives() throws {
        XCTAssertEqual(rust_double_u8(10), 20);
        XCTAssertEqual(rust_double_i8(10), 20);
        XCTAssertEqual(rust_double_u16(10), 20);
        XCTAssertEqual(rust_double_i16(10), 20);
        XCTAssertEqual(rust_double_u32(10), 20);
        XCTAssertEqual(rust_double_i32(10), 20);
        XCTAssertEqual(rust_double_u64(10), 20);
        XCTAssertEqual(rust_double_i64(10), 20);
        XCTAssertEqual(rust_double_f32(10.0), 20.0);
        XCTAssertEqual(rust_double_f64(10.0), 20.0);
        XCTAssertEqual(rust_negate_bool(true), false);
        XCTAssertEqual(rust_negate_bool(false), true);
    }
}

