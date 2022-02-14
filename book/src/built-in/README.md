# Built In Types

In addition to allowing you to share your own custom types between Rust and Swift,
`swift_bridge` comes with support for a number of Rust and Swift standard library types.

<!-- NOTE: Whenever we modify this list we need to copy it over to the repository's README -->

| name in Rust                                                    | name in Swift                                                    | notes               |
| ---                                                             | ---                                                              | ---                 |
| u8, i8, u16, i16... etc                                         | UInt8, Int8, UInt16, Int16 ... etc                               |                     |
| bool                                                            | Bool                                                             |                     |
| String, &String, &mut String                                    | RustString, RustStringRef, RustStringRefMut                      |                     |
| &str                                                            | RustStr                                                          |                     |
| Vec\<T>                                                         | RustVec\<T>                                                      |                     |
| SwiftArray\<T>                                                  | Array\<T>                                                        | Not yet implemented |
| &[T]                                                            | UnsafeBufferPointer\<T>                                          |                     |
| &mut [T]                                                        | UnsafeMutableBufferPointer\<T>                                   | Not yet implemented |
| SwiftString                                                     | String                                                           |                     |
| Box<T>                                                          |                                                                  | Not yet implemented |
| [T; N]                                                          |                                                                  | Not yet implemented |
| *const T                                                        | UnsafePointer\<T>                                                |                     |
| *mut T                                                          | UnsafeMutablePointer\<T>                                         |                     |
| Option\<T>                                                      | Optional\<T>                                                     |                     |
| Result\<T>                                                      |                                                                  | Not yet implemented |
| Have a Rust standard library type in mind?<br /> Open an issue! |                                                                  |                     |
|                                                                 | Have a Swift standard library type in mind?<br /> Open an issue! |                     |
