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
    
    func testSwiftCallRustWithOptionVecType() throws {
        let vec = RustVec<UInt16>()
        vec.push(value: 123)
        vec.push(value: 321)
        let refrelct = rust_reflect_option_vector_rust_type(vec)
        XCTAssertEqual(vec.len(), 2)
        XCTAssertEqual(vec.get(index: 0), 123)
        XCTAssertEqual(vec.get(index: 1), 321)
        
        XCTAssertNil(rust_reflect_option_vector_rust_type(nil))
    }
    
    func testSwiftCallRustWithOptionOpaqueRustType() throws {
        let val = OptTestOpaqueRustType(123)
        let reflect = rust_reflect_option_opaque_rust_type(val)
        XCTAssertEqual(reflect!.field(), 123)
        
        XCTAssertNil(rust_reflect_option_opaque_rust_type(nil))
    }
    
    func testSwiftCallRustWithOptionOpaqueRustCopyType() throws {
        let val = new_opaque_rust_copy_type(123)
        let reflect: OptTestOpaqueRustCopyType? = rust_reflect_option_opaque_rust_copy_type(val)
        
        // TODO: Support methods on generic types
        // XCTAssertEqual(reflect!.field(), 123)
        XCTAssertNil(rust_reflect_option_opaque_rust_copy_type(nil))
    }
    
    func testSwiftCallRustWithOptionGenericOpaqueRustType() throws {
        let val = new_generic_opaque_rust_type(123)
        let reflect = rust_reflect_option_generic_opaque_rust_type(val)
        
        // TODO: Support methods on generic types
        // XCTAssertEqual(reflect!.field(), 123)
        XCTAssertNil(rust_reflect_option_opaque_rust_type(nil))
    }
    
     func testSwiftCallRustWithOptionGenericOpaqueRustCopyType() throws {
        let val = new_generic_opaque_rust_copy_type(123)
        let reflect: OptTestGenericOpaqueRustCopyType? = rust_reflect_option_generic_opaque_rust_copy_type(val)
         
        // TODO: Support methods on generic types
        // XCTAssertEqual(reflect!.field(), 123)
        XCTAssertNil(rust_reflect_option_generic_opaque_rust_copy_type(nil))
    }

    
    func testStructWithOptionFieldsSome() throws {
        let val = StructWithOptionFields(
            u8: 123, i8: 123, u16: 123, i16: 123,
            u32: 123, i32: 123, u64: 123, i64: 123,
            usize: 123, isize: 123, f32: 123.4, f64: 123.4,
            boolean: true
        )
        let reflected = rust_reflect_struct_with_option_fields(val)
        XCTAssertEqual(reflected.u8, 123)
        XCTAssertEqual(reflected.i8, 123)
        XCTAssertEqual(reflected.u16, 123)
        XCTAssertEqual(reflected.i16, 123)
        XCTAssertEqual(reflected.u32, 123)
        XCTAssertEqual(reflected.i32, 123)
        XCTAssertEqual(reflected.u64, 123)
        XCTAssertEqual(reflected.i64, 123)
        XCTAssertEqual(reflected.usize, 123)
        XCTAssertEqual(reflected.isize, 123)
        XCTAssertEqual(reflected.f32, 123.4)
        XCTAssertEqual(reflected.f64, 123.4)
        XCTAssertEqual(reflected.boolean, true)
    }
    
    func testStructWithOptionFieldsNone() {
        let val = StructWithOptionFields(
            u8: nil, i8: nil, u16: nil, i16: nil,
            u32: nil, i32: nil, u64: nil, i64: nil,
            usize: nil, isize: nil, f32: nil, f64: nil,
            boolean: nil
        )
        let reflected = rust_reflect_struct_with_option_fields(val)
        XCTAssertEqual(reflected.i8, nil)
        XCTAssertEqual(reflected.u16, nil)
        XCTAssertEqual(reflected.i16, nil)
        XCTAssertEqual(reflected.u32, nil)
        XCTAssertEqual(reflected.i32, nil)
        XCTAssertEqual(reflected.u64, nil)
        XCTAssertEqual(reflected.i64, nil)
        XCTAssertEqual(reflected.usize, nil)
        XCTAssertEqual(reflected.isize, nil)
        XCTAssertEqual(reflected.f32, nil)
        XCTAssertEqual(reflected.f64, nil)
        XCTAssertEqual(reflected.boolean, nil)
    }
    
    func testEnumWhereVariantsHaveNoData() {
        let val = OptionEnumWithNoData.Variant2
        let reflectedSome = rust_reflect_option_enum_with_no_data(val)
        let reflectedNone = rust_reflect_option_enum_with_no_data(nil)
        
        switch reflectedSome! {
        case .Variant2:
            break;
        default:
            fatalError()
        }
        
        XCTAssertNil(reflectedNone)
    }
    
    func testOptionStruct() {
        let val = OptionStruct(field: 123)
        let reflectedSome = rust_reflect_option_struct_with_no_data(val)
        let reflectedNone = rust_reflect_option_struct_with_no_data(nil)
        
        XCTAssertEqual(reflectedSome!.field, 123)
        XCTAssertNil(reflectedNone)
    }
    
    func testRustCallSwiftReturnOption() {
        run_option_tests()
    }
}

