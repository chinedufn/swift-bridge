// swift-tools-version:5.5

import PackageDescription

let package = Package(
    name: "swift-package-test-package",
    products: [
        .library(
            name: "swift-package-test-package",
            targets: ["swift-package-test-package"]),
    ],
    dependencies: [
        .package(path: "../swift-package-rust-library-fixture/MySwiftPackage")
    ],
    targets: [
        .target(
            name: "swift-package-test-package",
            dependencies: []),
        .testTarget(
            name: "swift-package-test-packageTests",
            dependencies: ["swift-package-test-package", "MySwiftPackage"]),
    ]
)
