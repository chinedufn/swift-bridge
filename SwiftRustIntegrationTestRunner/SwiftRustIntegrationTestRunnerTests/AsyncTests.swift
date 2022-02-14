//
//  SharedStructAttributeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class AsyncTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Verify that we can call a function that uses a type that was already declared in a different bridge module.
    /// See crates/swift-integration-tests/src/struct_attributes/already_declared.rs
    func testAsyncExperiment() async throws {
        let num = await callRustAsyncFn()
        XCTAssertEqual(num, 5)
    }
    
    func testSwiftCallsRustAsyncFn() async throws {
        await rust_async_return_null()
    }
    
    func testSwiftCallsRustAsyncFnRetU8() async throws {
        let num = await rust_async_return_u8()
        XCTAssertEqual(num, 123)
    }
}

func callRustAsyncFn() async -> Int32 {
    class CbWrapper {
        var cb: (Result<Int32, Never>) -> ()
        
        init(cb: @escaping (Result<Int32, Never>) -> ()) {
            self.cb = cb
        }
    }
       
    func onComplete(wrapperPtr: UnsafeMutableRawPointer?, value: Int32) {
        let wrapper = Unmanaged<CbWrapper>.fromOpaque(wrapperPtr!).takeRetainedValue()
        wrapper.cb(.success(Int32(5)))
    }
 
    return await withCheckedContinuation({ (continuation: CheckedContinuation<Int32, Never>) in
        let callback = { num in
            continuation.resume(with: num)
        }
        
        let wrapper = CbWrapper(cb: callback)
        let wrapperPtr = Unmanaged.passRetained(wrapper).toOpaque()
        
        async_rust_fn(wrapperPtr, onComplete)
    })
}

