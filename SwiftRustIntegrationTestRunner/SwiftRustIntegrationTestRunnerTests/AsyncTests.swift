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

    func testSwiftCallsRustAsyncFn() async throws {
        await rust_async_return_null()
    }
   
    /// Verify that we can pass and return a u8 to an async Rust function
    func testSwiftCallsRustAsyncFnReflectU8() async throws {
        let num = await rust_async_reflect_u8(123)
        XCTAssertEqual(num, 123)
    }
    
     /// Verify that we can pass and return a String to an async Rust function
    func testSwiftCallsRustAsyncFnReflectString() async throws {
        let string = await rust_async_reflect_string("hello world")
        XCTAssertEqual(string.toString(), "hello world")
    }
    
    /// Verify that we can call async Rust methods
    func testSwiftCallsRustAsyncMethodReflectU16() async throws {
        let test = TestRustAsyncSelf()

        let num = await test.reflect_u16(567)
        XCTAssertEqual(num, 567)
    }

    
    /// Verify that we can pass and return a Result<OpaqueRust, OpaqueRust> to an async Rust function
    func testSwiftCallsRustAsyncFnReflectResultOpaqueRust() async throws {
        
        // Should return an AsyncResultOpaqueRustType1 type.
        do {
            let _ = try await rust_async_func_reflect_result_opaque_rust(.Ok(AsyncResultOpaqueRustType1(10)))
        } catch {
            XCTFail()
        }
        
        // Should throw an AsyncResultOpaqueRustType2 type that conforms to Error protocol.
        do {
            let _ = try await rust_async_func_reflect_result_opaque_rust(.Err(AsyncResultOpaqueRustType2(100)))
            XCTFail()
        } catch let error as AsyncResultOpaqueRustType2 {
            XCTAssertEqual(error.val(), 100)
        } catch {
            XCTFail()
        }
    }
    
    /// Verify that we can return a Result<TransparentEnum, TransparentEnum> from async Rust function
    func testSwiftCallsRustAsyncFnReturnResultTransparentEnum() async throws {
        
        //Should return an AsyncResultOkEnum
        do {
            let value: AsyncResultOkEnum = try await rust_async_func_return_result_transparent_enum_and_transparent_enum(true)
            switch value {
            case .NoFields:
                XCTFail()
            case .UnnamedFields(let valueInt32, let valueString):
                XCTAssertEqual(valueInt32, 123)
                XCTAssertEqual(valueString.toString(), "hello")
            case .NamedFields(_):
                XCTFail()
            }
        } catch {
            XCTFail()
        }
        
        //Should throw an AsyncResultErrEnum
        do {
            let _ = try await rust_async_func_return_result_transparent_enum_and_transparent_enum(false)
            XCTFail()
        } catch let error as AsyncResultErrEnum {
            switch error {
            case .NoFields:
                XCTFail()
            case .UnnamedFields(_, _):
                XCTFail()
            case .NamedFields(let valueUInt32):
                XCTAssertEqual(valueUInt32, 100)
            }
        } catch {
            XCTFail()
        }
    }
    
    /// Verify that we can return a Result<OpaqueRust, TransparentEnum> from async Rust function
    func testSwiftCallsRustAsyncFnReturnResultOpaqueRustTransparentEnum() async throws {
        //Should return an AsyncResultOpaqueRustType1
        do {
            let value: AsyncResultOpaqueRustType1 = try await rust_async_func_return_result_opaque_rust_and_transparent_enum(true)
            XCTAssertEqual(value.val(), 10)
        } catch {
            XCTFail()
        }
        
        //Should throw an AsyncResultErrEnum
        do {
            let _: AsyncResultOpaqueRustType1 = try await rust_async_func_return_result_opaque_rust_and_transparent_enum(false)
        } catch let error as AsyncResultErrEnum {
            switch error {
            case .NoFields:
                XCTFail()
            case .UnnamedFields(_, _):
                XCTFail()
            case .NamedFields(let value):
                XCTAssertEqual(value, 1000)
            }
        } catch {
            XCTFail()
        }
    }
    
    /// Verify that we can return a Result<TransparentEnum, OpaqueRust> from async Rust function
    func testSwiftCallsRustAsyncFnReturnResultTransparentEnumOpaqueRust() async throws {
        //Should return an AsyncResultOkEnum
        do {
            let value: AsyncResultOkEnum = try await rust_async_func_return_result_transparent_enum_and_opaque_rust(true)
            switch value {
            case .NoFields:
                break
            case .UnnamedFields(_, _):
                XCTFail()
            case .NamedFields(let value):
                XCTFail()
            }
        } catch {
            XCTFail()
        }
        
        //Should throw an AsyncResultOpaqueRustType1
        do {
            let _ = try await rust_async_func_return_result_transparent_enum_and_opaque_rust(false)
            XCTFail()
        } catch let error as AsyncResultOpaqueRustType1 {
            XCTAssertEqual(error.val(), 1000)
        } catch {
            XCTFail()
        }
        
    }
    
    func testSwiftCallsRustAsyncFnRetStruct() async throws {
        let _: AsyncRustFnReturnStruct = await rust_async_return_struct()
    }
}

