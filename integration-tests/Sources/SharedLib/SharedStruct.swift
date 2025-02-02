//
//  SharedStruct.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 1/20/22.
//

import Foundation
import RustLib

func rust_calls_swift_struct_with_no_fields(arg: StructWithNoFields) -> StructWithNoFields {
    arg
}

func rust_calls_swift_struct_repr_struct_one_primitive_field(
    arg: StructReprStructWithOnePrimitiveField
) -> StructReprStructWithOnePrimitiveField {
    arg
}

func rust_calls_swift_struct_repr_struct_one_string_field(arg: StructReprStructWithOneStringField)
    -> StructReprStructWithOneStringField
{
    arg
}
