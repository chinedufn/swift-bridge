# Summary

[Introduction](./README.md)

--- 

- [Building](./building/README.md)
  - [Xcode + Cargo](./building/xcode-and-cargo/README.md)
  - [swiftc + Cargo](./building/swiftc-and-cargo/README.md)
  - [Swift Packages](./building/swift-packages/README.md)

- [The Bridge Module](./bridge-module/README.md)
  - [Functions](./bridge-module/functions/README.md)
  - [Opaque Types](./bridge-module/opaque-types/README.md)
  - [Transparent Types](./bridge-module/transparent-types/README.md)
    - [Transparent Structs](./bridge-module/transparent-types/structs/README.md)
    - [Transparent Enums](./bridge-module/transparent-types/enums/README.md)
  - [Generics](./bridge-module/generics/README.md)
  - [Conditional Compilation](./bridge-module/conditional-compilation/README.md)

- [Built In Types](./built-in/README.md)
  - [String <---> String](./built-in/string/README.md)
  - [&str <---> RustStr](./built-in/str/README.md)
  - [Vec<T> <---> RustVec<T>](./built-in/vec/README.md)
  - [Option<T> <---> Optional<T>](./built-in/option/README.md)
  - [Result<T, E> <---> RustResult<T, E>](./built-in/result/README.md)
  - [Box<dyn FnOnce(A, B) -> C>](./built-in/boxed-functions/README.md)

- [Safety](./safety/README.md)

- [Contributing to swift-bridge](./contributing/README.md)
  - [Internal Design](./contributing/internal-design/README.md)
    - [Code Generation](./contributing/internal-design/codegen/README.md)
  - [Adding support for a signature](./contributing/adding-support-for-a-signature/README.md)
  - [Adding compile time errors](./contributing/adding-compile-time-errors/README.md)
