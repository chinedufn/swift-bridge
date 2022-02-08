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
            fatalError()
        }
    }
}
