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
        let enumWithUnnamedData1 = EnumWithUnnamedData.Variant1(create_string("hello"), 0)
        switch enumWithUnnamedData1 {
        case .Variant1(let rustString, let valueUInt32):
            XCTAssertEqual(rustString.toString(), "hello")
            XCTAssertEqual(valueUInt32, 0)
        default:
            XCTFail()
        }
        
        let enumWithUnnamedData2 = EnumWithUnnamedData.Variant2(1000, 10)
        switch enumWithUnnamedData2 {
        case .Variant2(let valueInt32, let valueUInt8):
            XCTAssertEqual(valueInt32, 1000)
            XCTAssertEqual(valueUInt8, 10)
        default:
            XCTFail()
        }

        let enumWithUnnamedData3 = EnumWithUnnamedData.Variant3
        switch enumWithUnnamedData3 {
        case .Variant3:
            break
        default:
            XCTFail()
        }

    }
}
