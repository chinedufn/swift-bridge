class SwiftString {
    var string: String

    init() {
        string = ""
    }

    init(str: RustStr) {
        string = String(bytes: str.toBufferPointer(), encoding: .utf8)!
    }

    func as_ptr() -> UnsafePointer<UInt8> {
        let buf: [UInt8] = Array(string.utf8)
        return UnsafePointer(buf)
    }

    func len () -> UInt {
        UInt(string.count)
    }
}

extension RustStr {
    func toBufferPointer() -> UnsafeBufferPointer<UInt8> {
        UnsafeBufferPointer(start: self.start, count: Int(self.len))
    }

    func toString() -> String {
        String(bytes: self.toBufferPointer(), encoding: .utf8)!
    }
}

extension String {
    func toRustStr() -> RustStr {
        let buf: [UInt8] = Array(self.utf8)
        let start = UnsafeMutablePointer(mutating: buf)
        let len = UInt(self.count)
        return RustStr(start: start, len: len)
    }
}// TODO:
//  Implement iterator https://developer.apple.com/documentation/swift/iteratorprotocol

class RustVec<T: Vectorizable & FfiOption> {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    init() {
        ptr = T.vecOfSelfNew()
        isOwned = true
    }

    func push (value: T) {
        T.vecOfSelfPush(vecPtr: ptr, value: value)
    }

    func pop () -> Optional<T> {
        let val = T.vecOfSelfPop(vecPtr: ptr)
        if _get_option_return() {
            return val;
        } else {
            return nil
        }
    }

    func get(index: UInt) -> Optional<T> {
        let val = T.vecOfSelfGet(vecPtr: ptr, index: index)
        if _get_option_return() {
            return val;
        } else {
            return nil
        }
    }

    /// Rust returns a UInt, but we cast to an Int because many Swift APIs such as
    /// `ForEach(0..rustVec.len())` expect Int.
    func len() -> Int {
        Int(T.vecOfSelfLen(vecPtr: ptr))
    }

    func asPtr() -> UnsafePointer<T> {
        T.vecOfSelfAsPtr(vecPtr: ptr)
    }

    func toUnsafeBufferPointer() -> UnsafeBufferPointer<T> {
        UnsafeBufferPointer(start: asPtr(), count: len())
    }

    deinit {
         T.vecOfSelfFree(vecPtr: ptr)
    }
}

extension RustVec: Sequence {
    func makeIterator() -> RustVecIterator<T> {
        return RustVecIterator(self)
    }
}

struct RustVecIterator<T: Vectorizable & FfiOption>: IteratorProtocol {
    var rustVec: RustVec<T>
    var index: UInt = 0

    init (_ rustVec: RustVec<T>) {
        self.rustVec = rustVec
    }

    mutating func next() -> T? {
        let val = rustVec.get(index: index)
        index += 1
        return val
    }
}

extension RustVec: Collection {
    typealias Index = Int

    func index(after i: Int) -> Int {
        i + 1
    }

    subscript(position: Int) -> T {
        self.get(index: UInt(position))!
    }

    var startIndex: Int {
        0
    }

    var endIndex: Int {
        self.len()
    }
}

extension RustVec: RandomAccessCollection {
}

extension UnsafeBufferPointer {
    func toFfiSlice () -> __private__FfiSlice {
        __private__FfiSlice(start: UnsafeMutablePointer(mutating: self.baseAddress), len: UInt(self.count))
    }
}

extension Array {
    /// Get an UnsafeBufferPointer to the array's content's first byte with the array's length.
    ///
    /// ```
    /// // BAD! Swift will immediately free the arrays memory and so your pointer is invalid.
    /// let pointer = useMyPointer([1, 2, 3].toUnsafeBufferPointer())
    ///
    /// // GOOD! The array will outlive the buffer pointer.
    /// let array = [1, 2, 3]
    /// useMyPointer(array.toUnsafeBufferPointer())
    /// ```
    func toUnsafeBufferPointer() -> UnsafeBufferPointer<Element> {
        UnsafeBufferPointer(start: UnsafePointer(self), count: self.count)
    }
}

protocol Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer;

    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer)

    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self)

    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self>

    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self>

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt

    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self>
}
protocol FfiOption {
    /// Used to create a value of this type that won't actually be used by the other side of the
    /// FFI boundary since we've set a flag to instruct Rust to ignore what we return and use
    /// `None` instead.
    static func unusedValue() -> Self
}

func markReturnTypeSome<T: FfiOption>(_ val: T) -> T {
    _set_option_return(true)
    if true { return val; } else { return T.unusedValue(); }
}

func markReturnTypeNone<T: FfiOption>() -> T {
    _set_option_return(false)
    return T.unusedValue()
}

extension UInt8: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_u8$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_u8$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_u8$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_u8$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u8$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u8$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_u8$as_ptr(vecPtr))
    }

}
    
extension UInt8: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension UInt16: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_u16$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_u16$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_u16$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_u16$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u16$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u16$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_u16$as_ptr(vecPtr))
    }

}
    
extension UInt16: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension UInt32: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_u32$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_u32$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_u32$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_u32$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u32$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u32$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_u32$as_ptr(vecPtr))
    }

}
    
extension UInt32: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension UInt64: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_u64$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_u64$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_u64$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_u64$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u64$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u64$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_u64$as_ptr(vecPtr))
    }

}
    
extension UInt64: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension UInt: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_usize$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_usize$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_usize$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_usize$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_usize$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_usize$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_usize$as_ptr(vecPtr))
    }

}
    
extension UInt: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension Int8: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_i8$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_i8$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_i8$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_i8$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i8$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i8$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_i8$as_ptr(vecPtr))
    }

}
    
extension Int8: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension Int16: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_i16$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_i16$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_i16$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_i16$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i16$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i16$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_i16$as_ptr(vecPtr))
    }

}
    
extension Int16: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension Int32: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_i32$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_i32$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_i32$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_i32$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i32$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i32$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_i32$as_ptr(vecPtr))
    }

}
    
extension Int32: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension Int64: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_i64$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_i64$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_i64$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_i64$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i64$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i64$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_i64$as_ptr(vecPtr))
    }

}
    
extension Int64: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension Int: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_isize$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_isize$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_isize$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_isize$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_isize$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_isize$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_isize$as_ptr(vecPtr))
    }

}
    
extension Int: FfiOption {
    static func unusedValue() -> Self {
        123
    }
}
    
extension Bool: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_bool$new()
    }
    
    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_bool$_free(vecPtr)
    }
    
    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {
        __swift_bridge__$Vec_bool$push(vecPtr, value)
    }
    
    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        __swift_bridge__$Vec_bool$pop(vecPtr)
    }
    
    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_bool$get(vecPtr, index)
    }
    
    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_bool$len(vecPtr)
    }
    
    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self> {
        UnsafePointer(__swift_bridge__$Vec_bool$as_ptr(vecPtr))
    }

}
    
extension Bool: FfiOption {
    static func unusedValue() -> Self {
        false
    }
}
    
@_cdecl("__swift_bridge__$SwiftString$new")
func __swift_bridge__SwiftString_new () -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(SwiftString()).toOpaque()
}

@_cdecl("__swift_bridge__$SwiftString$new_with_str")
func __swift_bridge__SwiftString_new_with_str (_ str: RustStr) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(SwiftString(str: str)).toOpaque()
}

@_cdecl("__swift_bridge__$SwiftString$as_ptr")
func __swift_bridge__SwiftString_as_ptr (_ this: UnsafeMutableRawPointer) -> UnsafePointer<UInt8> {
    Unmanaged<SwiftString>.fromOpaque(this).takeUnretainedValue().as_ptr()
}

@_cdecl("__swift_bridge__$SwiftString$len")
func __swift_bridge__SwiftString_len (_ this: UnsafeMutableRawPointer) -> UInt {
    Unmanaged<SwiftString>.fromOpaque(this).takeUnretainedValue().len()
}


public class RustString {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init() {
        ptr = __swift_bridge__$RustString$new()
    }

    init(_ str: RustStr) {
        ptr = __swift_bridge__$RustString$new_with_str(str)
    }

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    deinit {
        if isOwned {
            __swift_bridge__$RustString$_free(ptr)
        }
    }

    func len() -> UInt {
        __swift_bridge__$RustString$len(ptr)
    }

    func trim() -> RustStr {
        __swift_bridge__$RustString$trim(ptr)
    }
}

@_cdecl("__swift_bridge__$SwiftString$_free")
func __swift_bridge__SwiftString__free (ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<SwiftString>.fromOpaque(ptr).takeRetainedValue()
}



