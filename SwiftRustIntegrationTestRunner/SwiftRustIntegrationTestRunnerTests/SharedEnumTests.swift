//
//  SharedEnumTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 2/8/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class SharedEnumTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    func testEnumWithNoData() {
        let enumWithNoData1 = EnumWithNoData.Variant1
        let enumWithNoData2 = EnumWithNoData.Variant2
        
        let reflected1 = reflect_enum_with_no_data(enumWithNoData1)
        let reflected2 = reflect_enum_with_no_data(enumWithNoData2)
        
        switch (reflected1, reflected2) {
        case (.Variant1, .Variant2):
            break;
        default:
            XCTFail()
        }
    }

    func testEnumWithUnnamedData() {
        let enumWithUnnamedData1 = EnumWithUnnamedData.TwoFields(create_string("hello"), OpaqueRustForEnumTest())
        switch reflect_enum_with_unnamed_data(enumWithUnnamedData1) {
        case .TwoFields(let rustString, let opaqueRustForEnumTest):
            XCTAssertEqual(rustString.toString(), "hello")
            XCTAssertEqual(opaqueRustForEnumTest, OpaqueRustForEnumTest())
        default:
            XCTFail()
        }
        
        let enumWithUnnamedData2 = EnumWithUnnamedData.OneField(1000)
        switch reflect_enum_with_unnamed_data(enumWithUnnamedData2) {
        case .OneField(let valueInt32):
            XCTAssertEqual(valueInt32, 1000)
        default:
            XCTFail()
        }

        let enumWithUnnamedData3 = EnumWithUnnamedData.NoFields
        switch reflect_enum_with_unnamed_data(enumWithUnnamedData3) {
        case .NoFields:
            break
        default:
            XCTFail()
        }
    }
    
    func testEnumWithNamedData() {
        let enumWithNamedData1 = EnumWithNamedData.TwoFields(hello: create_string("hello"), data_u8: 123)
        switch reflect_enum_with_named_data(enumWithNamedData1) {
        case .TwoFields(let hello, let dataU8):
            XCTAssertEqual(hello.toString(), "hello")
            XCTAssertEqual(dataU8, 123)
        default:
            XCTFail()
        }
        
        let enumWithNamedData2 = EnumWithNamedData.OneField(data_i32: -123)
        switch reflect_enum_with_named_data(enumWithNamedData2) {
        case .OneField(let dataI32):
            XCTAssertEqual(dataI32, -123)
        default:
            XCTFail()
        }

        let enumWithNamedData3 = EnumWithNamedData.NoFields
        switch reflect_enum_with_named_data(enumWithNamedData3) {
        case .NoFields:
            break
        default:
            XCTFail()
        }
    }
    
    func testEnumWithOpaqueRust() {
        let named = EnumWithOpaqueRust.Named(data: OpaqueRustForEnumTest())
        switch reflect_enum_with_opaque_type(named) {
        case .Named(let value):
            XCTAssertEqual(value, OpaqueRustForEnumTest())
        case .Unnamed(_):
            XCTFail()
        }
        
        let unnamed = EnumWithOpaqueRust.Unnamed(OpaqueRustForEnumTest())
        switch reflect_enum_with_opaque_type(unnamed) {
        case .Named(_):
            XCTFail()
        case .Unnamed(let value):
            XCTAssertEqual(value, OpaqueRustForEnumTest())
        }
    }

    func testEnumWithGenericOpaqueRust() {
        let named = EnumWithGenericOpaqueRust.Named(data: new_generic_opaque_rust_for_enum_test())
        switch reflect_enum_with_generic_opaque_type(named) {
        case .Named(_):
            //TODO: call several methods on GenericOpaqueRustForEnumTest<Int32>
            break
        case .Unnamed(_):
            XCTFail()
        }
        
        let unnamed = EnumWithGenericOpaqueRust.Unnamed(new_generic_opaque_rust_for_enum_test())
        switch reflect_enum_with_generic_opaque_type(unnamed) {
        case .Named(_):
            XCTFail()
        case .Unnamed(_):
            //TODO: call several methods on GenericOpaqueRustForEnumTest<Int32>
            break
        }
    }

}
