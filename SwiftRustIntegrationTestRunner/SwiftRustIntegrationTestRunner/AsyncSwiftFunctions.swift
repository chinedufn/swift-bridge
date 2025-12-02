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
/// Uses typed throws (Swift 5.9+) to ensure compile-time verification of error type
func swift_async_throws(succeed: Bool) async throws(SwiftAsyncError) -> UInt32 {
    try? await Task.sleep(nanoseconds: 1_000_000) // 1ms
    if succeed {
        return 123
    } else {
        throw SwiftAsyncError.ErrorWithValue(456)
    }
}

/// An async Swift function that throws but returns void (maps to Result<(), E> in Rust)
/// Uses typed throws (Swift 5.9+) to ensure compile-time verification of error type
func swift_async_throws_void(succeed: Bool) async throws(SwiftAsyncVoidError) {
    try? await Task.sleep(nanoseconds: 1_000_000) // 1ms
    if !succeed {
        throw SwiftAsyncVoidError.ErrorWithValue(789)
    }
    // On success, just return (void)
}
