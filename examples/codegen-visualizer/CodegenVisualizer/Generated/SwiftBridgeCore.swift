import Foundation

class SwiftString {
    var string: String

    init() {
        string = ""
    }

    init(str: RustStr) {
        string = String(bytes: str.toBufferPointer(), encoding: .utf8)!
    }

    func as_ptr() -> UnsafePointer<UInt8> {
        // TODO: Does the utf8String have the same lifetime as our String?
        //  If not this can lead to undefined behavior..
        let ptr = UnsafeRawPointer((self.string as NSString).utf8String)!
        let start = ptr.assumingMemoryBound(to: UInt8.self)
        return start
    }

    func len () -> UInt {
        UInt(string.count)
    }
}

extension RustString {
    func toString() -> String {
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

    func toString() -> String {
        let bytes = self.toBufferPointer()
        return String(bytes: bytes, encoding: .utf8)!
    }
}// TODO:
//  Implement iterator https://developer.apple.com/documentation/swift/iteratorprotocol

class RustVec<T: Vectorizable> {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
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

    func get(index: UInt) -> Optional<T.SelfRef> {
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

    deinit {
         T.vecOfSelfFree(vecPtr: ptr)
    }
}

extension RustVec: Sequence {
    func makeIterator() -> RustVecIterator<T> {
        return RustVecIterator(self)
    }
}

struct RustVecIterator<T: Vectorizable>: IteratorProtocol {
    var rustVec: RustVec<T>
    var index: UInt = 0

    init (_ rustVec: RustVec<T>) {
        self.rustVec = rustVec
    }

    mutating func next() -> T.SelfRef? {
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

    subscript(position: Int) -> T.SelfRef {
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
    associatedtype SelfRef
    associatedtype SelfRefMut

    static func vecOfSelfNew() -> UnsafeMutableRawPointer;

    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer)

    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self)

    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self>

    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SelfRef>

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SelfRefMut>

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u8$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u8$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u16$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u16$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u32$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u32$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_u64$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_u64$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_usize$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_usize$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i8$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i8$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i16$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i16$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i32$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i32$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_i64$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_i64$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_isize$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_isize$len(vecPtr)
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

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {
        __swift_bridge__$Vec_bool$get_mut(vecPtr, index)
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_bool$len(vecPtr)
    }
}
    
@_cdecl("__swift_bridge__$SwiftString$new")
func __swift_bridge__SwiftString_new () -> __private__PointerToSwiftType {
    __private__PointerToSwiftType(ptr: Unmanaged.passRetained(SwiftString()).toOpaque())
}

@_cdecl("__swift_bridge__$SwiftString$new_with_str")
func __swift_bridge__SwiftString_new_with_str (_ str: RustStr) -> __private__PointerToSwiftType {
    __private__PointerToSwiftType(ptr: Unmanaged.passRetained(SwiftString(str: str)).toOpaque())
}

@_cdecl("__swift_bridge__$SwiftString$as_ptr")
func __swift_bridge__SwiftString_as_ptr (_ this: UnsafeMutableRawPointer) -> UnsafePointer<UInt8> {
    Unmanaged<SwiftString>.fromOpaque(this).takeUnretainedValue().as_ptr()
}

@_cdecl("__swift_bridge__$SwiftString$len")
func __swift_bridge__SwiftString_len (_ this: UnsafeMutableRawPointer) -> UInt {
    Unmanaged<SwiftString>.fromOpaque(this).takeUnretainedValue().len()
}


public class RustString: RustStringRefMut {
    var isOwned: Bool = true

    init() {
        super.init(ptr: __swift_bridge__$RustString$new())
    }

    init(_ str: String) {
        super.init(ptr: str.utf8CString.withUnsafeBufferPointer({strPtr in
            __swift_bridge__$RustString$new_with_str(RustStr(start: UnsafeMutableRawPointer(mutating: strPtr.baseAddress!).assumingMemoryBound(to: UInt8.self), len: UInt(str.count)))
        }))
    }

    override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RustString$_free(ptr)
        }
    }
}
public class RustStringRefMut: RustStringRef {
    override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class RustStringRef {
    var ptr: UnsafeMutableRawPointer

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    func len() -> UInt {
        __swift_bridge__$RustString$len(ptr)
    }

    func as_str() -> RustStr {
        __swift_bridge__$RustString$as_str(ptr)
    }

    func trim() -> RustStr {
        __swift_bridge__$RustString$trim(ptr)
    }
}
extension RustString: Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RustString$new()
    }

    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RustString$drop(vecPtr)
    }

    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RustString) {
        __swift_bridge__$Vec_RustString$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RustString$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RustString(ptr: pointer!) as! Self)
        }
    }

    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RustStringRef> {
        let pointer = __swift_bridge__$Vec_RustString$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RustStringRef(ptr: pointer!)
        }
    }

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RustStringRefMut> {
        let pointer = __swift_bridge__$Vec_RustString$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RustStringRefMut(ptr: pointer!)
        }
    }

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RustString$len(vecPtr)
    }
}


@_cdecl("__swift_bridge__$SwiftString$_free")
func __swift_bridge__SwiftString__free (ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<SwiftString>.fromOpaque(ptr).takeRetainedValue()
}



