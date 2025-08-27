//
//  SendableAttributeTests.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 2/4/25.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for Swift's `Sendable` protocol and the `#[swift_bridge(Sendable)]` attribute.
///
/// For the corresponding Rust code, see `crates/swift-integration-tests/src/sendable_attribute.rs`.
/// For the corresponding codegen tests, see `crates/swift-bridge-ir/src/codegen/codegen_tests/sendable_attribute.rs`.
class SendableTests: XCTestCase {
    /// Verify that we can a Rust type that has the `#[swift_bridge(Sendable)]` attribute gets a `Sendable` protocol implementation.
    func testSendableExternRustType() throws {
        let sendableRustType = SendableRustType()
        
        // Move the type to another thread.
        Thread.detachNewThread {
            let _ = sendableRustType;
        }
    }

    /// Verify that we can send a `RustString` to another thread.
    func testSendableRustString() {
        let rustString = RustString("hello world")

        Thread.detachNewThread {
            XCTAssertEqual(rustString.len(), 11)
        }
    }
}


protocol AssertIsSendable: Sendable {}
extension SendableRustType: AssertIsSendable{}

