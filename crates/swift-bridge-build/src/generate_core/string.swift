import Foundation

extension RustString {
    public func toString() -> String {
        let str = self.as_str()
        let string = str.toString()

        return string
    }
}

extension RustStr {
    func toBufferPointer() -> UnsafeBufferPointer<UInt8> {
        let bytes = UnsafeBufferPointer(start: self.start, count: Int(self.len))
        return bytes
    }

    public func toString() -> String {
        let bytes = self.toBufferPointer()
        return String(bytes: bytes, encoding: .utf8)!
    }
}
extension RustStr: Identifiable {
    public var id: String {
        self.toString()
    }
}
extension RustStr: Equatable {
    public static func == (lhs: RustStr, rhs: RustStr) -> Bool {
        // TODO: Rather than compare Strings, we can avoid allocating by calling a function
        // on the Rust side that compares the underlying byte slices.
        return
            lhs.toString() == rhs.toString()
    }
}

public protocol IntoRustString {
    func intoRustString() -> RustString;
}

public protocol ToRustStr {
    func toRustStr<T> (_ withUnsafeRustStr: (RustStr) -> T) -> T;
}

extension String: IntoRustString {
    public func intoRustString() -> RustString {
        // TODO: When passing an owned Swift std String to Rust we've being wasteful here in that
        //  we're creating a RustString (which involves Boxing a Rust std::string::String)
        //  only to unbox it back into a String once it gets to the Rust side.
        //
        //  A better approach would be to pass a RustStr to the Rust side and then have Rust
        //  call `.to_string()` on the RustStr.
        RustString(self)
    }
}

extension RustString: IntoRustString {
    public func intoRustString() -> RustString {
        self
    }
}

/// If the String is Some:
///   Safely get a scoped pointer to the String and then call the callback with a RustStr
///   that uses that pointer.
///
/// If the String is None:
///   Call the callback with a RustStr that has a null pointer.
///   The Rust side will know to treat this as `None`.
func optionalStringIntoRustString<S: IntoRustString>(_ string: Optional<S>) -> RustString? {
    if let val = string {
        return val.intoRustString()
    } else {
        return nil
    }
}

extension String: ToRustStr {
    /// Safely get a scoped pointer to the String and then call the callback with a RustStr
    /// that uses that pointer.
    public func toRustStr<T> (_ withUnsafeRustStr: (RustStr) -> T) -> T {
        return self.utf8CString.withUnsafeBufferPointer({ bufferPtr in
            let rustStr = RustStr(
                start: UnsafeMutableRawPointer(mutating: bufferPtr.baseAddress!).assumingMemoryBound(to: UInt8.self),
                // Subtract 1 because of the null termination character at the end
                len: UInt(bufferPtr.count - 1)
            )
            return withUnsafeRustStr(rustStr)
        })
    }
}

extension RustStr: ToRustStr {
    public func toRustStr<T> (_ withUnsafeRustStr: (RustStr) -> T) -> T {
        return withUnsafeRustStr(self)
    }
}

func optionalRustStrToRustStr<S: ToRustStr, T>(_ str: Optional<S>, _ withUnsafeRustStr: (RustStr) -> T) -> T {
    if let val = str {
        return val.toRustStr(withUnsafeRustStr)
    } else {
        return withUnsafeRustStr(RustStr(start: nil, len: 0))
    }
}
