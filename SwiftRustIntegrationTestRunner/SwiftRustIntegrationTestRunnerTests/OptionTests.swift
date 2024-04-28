//
//  OptionTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/21/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class OptionTests: XCTestCase {

    /// Verify that Swift can call Rust functions that accept and return Option<T>
    /// where T is a primitive.
    func testSwiftCallRustOptionPrimitive() throws {
        XCTAssertEqual(rust_reflect_option_u8(70), 70)
        XCTAssertEqual(rust_reflect_option_u8(nil), nil)

        XCTAssertEqual(rust_reflect_option_i8(70), 70)
        XCTAssertEqual(rust_reflect_option_i8(nil), nil)

        XCTAssertEqual(rust_reflect_option_u16(70), 70)
        XCTAssertEqual(rust_reflect_option_u16(nil), nil)

        XCTAssertEqual(rust_reflect_option_i16(70), 70)
        XCTAssertEqual(rust_reflect_option_i16(nil), nil)

        XCTAssertEqual(rust_reflect_option_u32(70), 70)
        XCTAssertEqual(rust_reflect_option_u32(nil), nil)

        XCTAssertEqual(rust_reflect_option_i32(70), 70)
        XCTAssertEqual(rust_reflect_option_i32(nil), nil)

        XCTAssertEqual(rust_reflect_option_u64(70), 70)
        XCTAssertEqual(rust_reflect_option_u64(nil), nil)

        XCTAssertEqual(rust_reflect_option_i64(70), 70)
        XCTAssertEqual(rust_reflect_option_i64(nil), nil)

        XCTAssertEqual(rust_reflect_option_f32(70.0), 70.0)
        XCTAssertEqual(rust_reflect_option_f32(nil), nil)

        XCTAssertEqual(rust_reflect_option_f64(70.0), 70.0)
        XCTAssertEqual(rust_reflect_option_f64(nil), nil)

        XCTAssertEqual(rust_reflect_option_bool(true), true)
        XCTAssertEqual(rust_reflect_option_bool(false), false)
        XCTAssertEqual(rust_reflect_option_bool(nil), nil)
    }

    /// Verify that Rust can call Swift functions that accept and return Option<T>.
    func testRustCallSwiftOptionPrimitive() throws {
        test_rust_calls_swift_option_primitive()
    }

    /// Verify that Swift can call a Rust function that accepts and returns an Option<T>
    /// where T is a String.
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

    func testSwiftCallRustWithOptionVecOfPrimitiveType() throws {
        let vec = RustVec<UInt16>()
        vec.push(value: 123)
        vec.push(value: 321)

        let reflected = rust_reflect_option_vector_rust_type(vec)!
        XCTAssertEqual(reflected.len(), 2)

        XCTAssertEqual(reflected.get(index: 0), 123)
        XCTAssertEqual(reflected.get(index: 1), 321)

        XCTAssertNil(rust_reflect_option_vector_rust_type(nil))
    }

    func testSwiftCallRustWithOptionOpaqueRustType() throws {
        let val = OptTestOpaqueRustType(123)
        let reflect = rust_reflect_option_opaque_rust_type(val)
        XCTAssertEqual(reflect!.field(), 123)

        XCTAssertNil(rust_reflect_option_opaque_rust_type(nil))
    }

    /// Verify that we can bridge options of opaque Swift types.
    func testSwiftCallRustWithOptionOpaqueSwiftType() throws {
        let val = OptTestOpaqueSwiftType(val: 727)
        let reflect = rust_reflect_option_opaque_swift_type(val)
        XCTAssertEqual(reflect!.val, 727)

        XCTAssertNil(rust_reflect_option_opaque_swift_type(nil))
    }

    /// Verify that we can pass and receive an `Option<&RustType>`.
    ///
    /// We deinitialize the first reference and create a second to confirm that
    /// deinitializing the reference does not deinitialize the Rust type.
    func testSwiftCallRustWithOptionRefOpaqueRustType() throws {
        let val = OptTestOpaqueRefRustType.new(123)
        let opt_ref = val.field_ref()

        var reflect = rust_reflect_option_ref_opaque_rust_type(opt_ref)
        XCTAssertEqual(reflect!.field(), 123)
        XCTAssertNil(rust_reflect_option_ref_opaque_rust_type(nil))
        reflect = nil

        reflect = rust_reflect_option_ref_opaque_rust_type(opt_ref)
        XCTAssertEqual(reflect!.field(), 123)
    }

    func testSwiftCallRustWithOptionOpaqueRustCopyType() throws {
        let val = new_opaque_rust_copy_type(123)
        let _: OptTestOpaqueRustCopyType? = rust_reflect_option_opaque_rust_copy_type(val)

        // TODO: Support methods on generic types
        // XCTAssertEqual(reflect!.field(), 123)
        XCTAssertNil(rust_reflect_option_opaque_rust_copy_type(nil))
    }

    func testSwiftCallRustWithOptionGenericOpaqueRustType() throws {
        let val = new_generic_opaque_rust_type(123)
        let _: OptTestGenericOpaqueRustType<UInt8>? = rust_reflect_option_generic_opaque_rust_type(val)

        // TODO: Support methods on generic types
        // XCTAssertEqual(reflect!.field(), 123)
        XCTAssertNil(rust_reflect_option_opaque_rust_type(nil))
    }

     func testSwiftCallRustWithOptionGenericOpaqueRustCopyType() throws {
        let val = new_generic_opaque_rust_copy_type(123)
        let _: OptTestGenericOpaqueRustCopyType? = rust_reflect_option_generic_opaque_rust_copy_type(val)

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
            XCTFail()
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
}
