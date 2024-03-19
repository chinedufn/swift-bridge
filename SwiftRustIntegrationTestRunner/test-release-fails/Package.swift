// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
	name: "TestReleaseFails",
	products: [
		.library(
			name: "TestReleaseFails",
			targets: ["TestReleaseFails"]),
        .executable(
            name: "TestReleaseFailsRunner",
            targets: ["TestReleaseFailsRunner"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "RustXcframework",
			path: "RustXcframework.xcframework"
		),
		.target(
			name: "TestReleaseFails",
			dependencies: ["RustXcframework"]),
        .executableTarget(
            name: "TestReleaseFailsRunner",
            dependencies: ["TestReleaseFails"])
	]
)	
