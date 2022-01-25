//
//  OptionTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/21/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class OptionTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    func testSwiftCallRustOptionU8() throws {
        XCTAssertEqual(rust_reflect_option_u8(70), 70)
        XCTAssertEqual(rust_reflect_option_u8(nil), nil)
    }
     
    func testSwiftCallRustOptionI8() throws {
        XCTAssertEqual(rust_reflect_option_i8(70), 70)
        XCTAssertEqual(rust_reflect_option_i8(nil), nil)
    }
     
    func testSwiftCallRustOptionU16() throws {
        XCTAssertEqual(rust_reflect_option_u16(70), 70)
        XCTAssertEqual(rust_reflect_option_u16(nil), nil)
    }
     
    func testSwiftCallRustOptionI16() throws {
        XCTAssertEqual(rust_reflect_option_i16(70), 70)
        XCTAssertEqual(rust_reflect_option_i16(nil), nil)
    }

    func testSwiftCallRustOptionU32() throws {
        XCTAssertEqual(rust_reflect_option_u32(70), 70)
        XCTAssertEqual(rust_reflect_option_u32(nil), nil)
    }
     
    func testSwiftCallRustOptionI32() throws {
        XCTAssertEqual(rust_reflect_option_i32(70), 70)
        XCTAssertEqual(rust_reflect_option_i32(nil), nil)
    }

    func testSwiftCallRustOptionU64() throws {
        XCTAssertEqual(rust_reflect_option_u64(70), 70)
        XCTAssertEqual(rust_reflect_option_u64(nil), nil)
    }
     
    func testSwiftCallRustOptionI64() throws {
        XCTAssertEqual(rust_reflect_option_i64(70), 70)
        XCTAssertEqual(rust_reflect_option_i64(nil), nil)
    }

    func testSwiftCallRustOptionF32() throws {
        XCTAssertEqual(rust_reflect_option_f32(70.0), 70.0)
        XCTAssertEqual(rust_reflect_option_f32(nil), nil)
    }
     
    func testSwiftCallRustOptionF64() throws {
        XCTAssertEqual(rust_reflect_option_f64(70.0), 70.0)
        XCTAssertEqual(rust_reflect_option_f64(nil), nil)
    }

    func testSwiftCallRustOptionBool() throws {
        XCTAssertEqual(rust_reflect_option_bool(true), true)
        XCTAssertEqual(rust_reflect_option_bool(false), false)
        XCTAssertEqual(rust_reflect_option_bool(nil), nil)
    }
    
    func testSwiftCallRustReturnOptionString() throws {
        let string = rust_reflect_option_string("hello world")
        XCTAssertEqual(string!.toString(), "hello world")
        
        let none: String? = nil
        XCTAssertNil(rust_reflect_option_string(none))
    }
    
    /// We use an `Option<&'static str>` that we create on the Rust side so that
    ///  we don't run into any lifetime issues.
    func testSwiftCallRustReturnOptionStr() throws {
        let str = rust_create_option_static_str()
        XCTAssertEqual(str!.toString(), "hello")
        
        let reflected = rust_reflect_option_str(str)
        XCTAssertEqual(reflected!.toString(), "hello")
        
        let none: RustStr? = nil
        XCTAssertNil(rust_reflect_option_str(none))
    }
    
    func testSwiftCallRustWithOptionOpaqueRustType() throws {
        let val = OptTestOpaqueRustType(123)
        let reflect = rust_reflect_option_opaque_rust_type(val)
        XCTAssertEqual(reflect!.field(), 123)
        
        XCTAssertNil(rust_reflect_option_opaque_rust_type(nil))
    }
    
    func testRustCallSwiftReturnOption() {
        run_option_tests()
    }
}

