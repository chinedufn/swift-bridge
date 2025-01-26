// swift-tools-version: 6.0

import PackageDescription

let linkerSettings: [LinkerSetting] = [
    .linkedLibrary("swift_integration_tests"),
    .unsafeFlags(["-L../target/debug/"])
]

let package = Package(
    name: "IntegrationTests",
    targets: [
        // The compiled static rust library.
        .systemLibrary(
            name: "RustLib"),
        // The generated Swift wrapper code for the Rust library, plus some
        // Swift code used by the Rust library.
        .target(
            name: "SharedLib",
            dependencies: ["RustLib"],
            linkerSettings: linkerSettings),
        .testTarget(
            name: "IntegrationTests",
            dependencies: ["SharedLib", "RustLib"]),
    ]
)
