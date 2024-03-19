// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
	name: "TestReleaseSucceeds",
	products: [
		.library(
			name: "TestReleaseSucceeds",
			targets: ["TestReleaseSucceeds"]),
        .executable(
            name: "TestReleaseSucceedsRunner",
            targets: ["TestReleaseSucceedsRunner"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "RustXcframework",
			path: "RustXcframework.xcframework"
		),
		.target(
			name: "TestReleaseSucceeds",
			dependencies: ["RustXcframework"]),
        .executableTarget(
            name: "TestReleaseSucceedsRunner",
            dependencies: ["TestReleaseSucceeds"])
	]
)	
