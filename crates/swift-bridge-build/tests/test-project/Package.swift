// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "test_project",
    dependencies: [
        .package(path: "../test-swift-packages/MySwiftPackage")
    ],
    targets: [
        .executableTarget(
            name: "test_project",
            dependencies: [
                .product(name: "MySwiftPackage", package: "MySwiftPackage")
            ]),
    ]
)
