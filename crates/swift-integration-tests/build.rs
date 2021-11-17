fn main() {
    swift_bridge_build::parse_bridges(vec!["./src/expose_opaque_rust_struct.rs"], "./foo");
}

// Ok so first we want to call build() with the names of the files
// src/import_opaque_swift_class.rs and src/expose_opaque_rust_struct.rs
// We then want to comment out our hand written externs in expose_opaque_rust_struct
// We then want to spit out a Swift file and C header file
// We then want to add the Swift and C header files to our Xcode project
// After that we can massage things until our OpaqueRustStructTests.swift passes
//
// Ok so we need this to work even if we have dependencies that use swift-bridge
// So we should output to a directory
// Swift needs to be able to add files from this directory
// So... SWIFT_BRIDGE_OUT_DIR env var..
// And we set that environment var in Xcode
//
// # Implementation
//
// - (DONE) Make the `parse_bridges` function write each C and Swift file to the passed in out dir
//    of XCodeSwift/Headers
//
// - (DONE) Uncomment the bridge module in expose opaque rust struct file
//
// - Add generated Swift file
//
// - Add c header file to bridging header in Xcode
//
// - Get Xcode building
//
// - Get OpaqueRustStructTests passing
//   - unbox and drop pointer if method taken by self
//
// - Delete out hard coded externs in expose opaque rust struct file
//
// # More planning
