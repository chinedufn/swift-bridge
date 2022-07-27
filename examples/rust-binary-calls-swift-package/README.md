# Rust Binary calls Swift Package

In this example we create a Rust binary that statically links to a Swift Package.

This means that we:

1. Use `swift-bridge-build` to generate our Swift FFI layer.

2. Compile the Swift Package into a static library. We include our generated `swift-bridge` FFI glue from step 1.

3. Compile our Rust executable. Along the way we link to our Swift static library.

---

## To Run

```
git clone https://github.com/chinedufn/swift-bridge
cd swift-bridge

cargo run -p rust-binary-calls-swift-package
```

You should see the following output:

```sh
The Rust starting number is 100.
Starting Swift multiply by 4 function...
Calling the Rust double function twice in order to 4x our number...
Rust double function called...
Rust double function called...
Leaving Swift multiply by 4 function...
Printing the number from Rust...
The number is now 400.
```
