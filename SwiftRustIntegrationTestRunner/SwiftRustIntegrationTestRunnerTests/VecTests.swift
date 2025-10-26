//
//  VecTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/21/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class VecTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    func testRustVecU8Len() throws {
        let vec = RustVec<UInt8>()
        XCTAssertEqual(vec.len(), 0)
        vec.push(value: 123)
        XCTAssertEqual(vec.len(), 1)
    }
    func testRustVecU8Pop() throws {
        let vec = RustVec<UInt8>()
        vec.push(value: 123)
        let popped = vec.pop()
        XCTAssertEqual(popped, 123)
        XCTAssertEqual(vec.len(), 0)
    }
    func testRustVecU8Get() throws {
        let vec = RustVec<UInt8>()
        vec.push(value: 111)
        vec.push(value: 222)
        XCTAssertEqual(vec.get(index: 1), 222)
    }
    func testRustVecU8AsPtr() throws {
        let vec = RustVec<UInt8>()
        vec.push(value: 10)
        let ptr = vec.as_ptr()
        XCTAssertEqual(ptr.pointee, 10)
    }
    func testRustVecU8Iterator() throws {
        let vec = RustVec<UInt8>()
        vec.push(value: 111)
        vec.push(value: 222)
        
        var iterations = 0
        for (index, val) in vec.enumerated() {
            XCTAssertEqual(val, vec[index])
            iterations += 1
        }
        XCTAssertEqual(iterations, 2)
    }
    
    func testVecOfOpaqueRustTypeLen() throws {
        let vec = RustVec<ARustTypeInsideVecT>()
        XCTAssertEqual(vec.len(), 0)
        vec.push(value: ARustTypeInsideVecT("hello world"))
        XCTAssertEqual(vec.len(), 1)
    }
    func testVecOfOpaqueRustTypeGet() throws {
        let vec: RustVec<ARustTypeInsideVecT> = RustVec()
        vec.push(value: ARustTypeInsideVecT("hello world"))
        XCTAssertEqual(vec.get(index: 0)!.text().toString(), "hello world")
    }
    func testVecOfOpaqueRustTypePop() throws {
        let vec: RustVec<ARustTypeInsideVecT> = RustVec()
        vec.push(value: ARustTypeInsideVecT("hello world"))
        
        XCTAssertEqual(vec.len(), 1)
        let popped = vec.pop()
        XCTAssertEqual(popped?.text().toString(), "hello world")
        XCTAssertEqual(vec.len(), 0)
    }
    
    /// Verify that a Vec<T> of opaque Rust types can be used as an argument and return
    /// type for extern "Rust" functions.
    func testReflectVecOfOpaqueRustType() throws {
        let vec: RustVec<ARustTypeInsideVecT> = RustVec()
        vec.push(value: ARustTypeInsideVecT("hello world"))
        
        let reflected = rust_reflect_vec_opaque_rust_type(vec)
        XCTAssertEqual(reflected.len(), 1)
        XCTAssertEqual(reflected.get(index: 0)!.text().toString(), "hello world")
    }
    
    func testVecOfOpaqueRustCopyTypeLen() throws {
        let vec = RustVec<ARustCopyTypeInsideVecT>()
        XCTAssertEqual(vec.len(), 0)
        vec.push(value: ARustCopyTypeInsideVecT(42))
        XCTAssertEqual(vec.len(), 1)
    }
    func testVecOfOpaqueRustCopyTypeGet() throws {
        let vec: RustVec<ARustCopyTypeInsideVecT> = RustVec()
        vec.push(value: ARustCopyTypeInsideVecT(42))
        XCTAssertEqual(vec.get(index: 0)!.value(), 42)
    }
    func testVecOfOpaqueRustCopyTypePop() throws {
        let vec: RustVec<ARustCopyTypeInsideVecT> = RustVec()
        vec.push(value: ARustCopyTypeInsideVecT(42))
        
        XCTAssertEqual(vec.len(), 1)
        let popped = vec.pop()
        XCTAssertEqual(popped?.value(), 42)
        XCTAssertEqual(vec.len(), 0)
    }
    
    /// Verify that a Vec<T> of opaque Rust copy types can be used as an argument and return
    /// type for extern "Rust" functions.
    func testReflectVecOfOpaqueRustCopyType() throws {
        let vec: RustVec<ARustCopyTypeInsideVecT> = RustVec()
        vec.push(value: ARustCopyTypeInsideVecT(42))
        
        let reflected = rust_reflect_vec_opaque_rust_copy_type(vec)
        XCTAssertEqual(reflected.len(), 1)
        XCTAssertEqual(reflected.get(index: 0)!.value(), 42)
    }
    
    /// Verify that a Vec<T> of transparent enums can be used as an argument and return
    /// type for extern "Rust" functions.
    func testReflectVecOfTransparentEnum() throws {
        let vec: RustVec<TransparentEnumInsideVecT> = RustVec()
        vec.push(value: TransparentEnumInsideVecT.VariantB)
        
        let reflected = rust_reflect_vec_transparent_enum(vec)
        XCTAssertEqual(reflected.len(), 1)
        XCTAssertEqual(reflected.get(index: 0)!, TransparentEnumInsideVecT.VariantB)
        XCTAssertEqual(reflected.pop()!, TransparentEnumInsideVecT.VariantB)
    }
    
    /// Verify that we can construct a RustVec of every primitive type.
    /// We tested all of the methods on  two different primitives above to be sure that our
    /// functions that generate the pieces of the RustVec support aren't accidentally hard coded to
    /// only work for one type.
    /// Here we call the rest of the types, confident that if we can construct them then the rest of their
    /// methods will work since they worked for the other types above.
    func testConstructPrimitiveRustVecs() throws {
        XCTAssertEqual(RustVec<UInt8>().pop(), nil);
        XCTAssertEqual(RustVec<UInt16>().len(), 0);
        XCTAssertEqual(RustVec<UInt32>().len(), 0);
        XCTAssertEqual(RustVec<UInt64>().len(), 0);
        XCTAssertEqual(RustVec<UInt>().len(), 0);
        
        XCTAssertEqual(RustVec<Int8>().len(), 0);
        XCTAssertEqual(RustVec<Int16>().len(), 0);
        XCTAssertEqual(RustVec<Int32>().len(), 0);
        XCTAssertEqual(RustVec<Int64>().len(), 0);
        XCTAssertEqual(RustVec<Int>().len(), 0);
        
        XCTAssertEqual(RustVec<Bool>().len(), 0);

        XCTAssertEqual(RustVec<Float>().len(), 0);
        XCTAssertEqual(RustVec<Double>().len(), 0);
    }

    /// Verify that Rust can pass `RustVec`s to and receive `RustVec`s from Swift.
    func testRustCallsSwiftRustVecFunctions() {
        run_vec_tests()
    }
}


