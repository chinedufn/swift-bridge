//
//  CallbackTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 9/11/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class CallbackTests: XCTestCase {
    
    /// Run our tests where Rust passes a callback to Swift.
    func testRustCallsSwift() throws {
        test_callbacks_rust_calls_swift()
    }
}
