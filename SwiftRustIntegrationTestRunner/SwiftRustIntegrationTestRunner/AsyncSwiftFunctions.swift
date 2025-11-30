//
//  AsyncSwiftFunctions.swift
//  SwiftRustIntegrationTestRunner
//
//  Async Swift functions that are called from Rust.
//

import Foundation

/// A simple async Swift function that returns void
func swift_async_void() async {
    // Simulate some async work
    try? await Task.sleep(nanoseconds: 1_000_000) // 1ms
}

/// An async Swift function that returns a u32
func swift_async_return_u32() async -> UInt32 {
    try? await Task.sleep(nanoseconds: 1_000_000) // 1ms
    return 42
}

/// An async Swift function that returns a String
func swift_async_return_string() async -> RustString {
    try? await Task.sleep(nanoseconds: 1_000_000) // 1ms
    return "Hello from Swift async!".intoRustString()
}

/// An async Swift function that can throw an error (maps to Result<T, E> in Rust)
func swift_async_throws(succeed: Bool) async throws -> UInt32 {
    try? await Task.sleep(nanoseconds: 1_000_000) // 1ms
    if succeed {
        return 123
    } else {
        throw SwiftAsyncError.ErrorWithValue(123)
    }
}
