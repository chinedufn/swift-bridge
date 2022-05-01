//
//  GenericTests.rs.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 4/30/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for generic types such as `type SomeType<u32>`
class GenericTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    func testReflectGenericOpaqueRustType() {
        let val = new_some_generic_type_u32()
        let _: SomeGenericType<UInt32> = reflect_generic_u32(val)
    }
    
    func testReflectGenericOpaqueCopyRustType() {
        let val = new_some_generic_copy_type_u32()
        let _: SomeGenericCopyType<UInt32> = reflect_generic_copy_u32(val)
    }
    
    func testReflectGenericWithInnerOpaqueRustType() {
        let val = new_generic_with_inner_opaque_type()
        let _: GenericWithOpaqueRustInnerTy<InnerTy> = reflect_generic_with_inner_opaque_type(val)
    }
}

