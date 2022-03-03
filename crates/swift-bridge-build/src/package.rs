use std::collections::HashMap;

/// Config for generating Swift packages
pub struct GeneratePackageConfig {
	bridge_dir: String,
    /// Path per platform. e.g. (iOS, "target/aarch64-apple-ios/debug/libmy_rust_lib.a")
	paths: HashMap<PackagePlatform, String>,
	out_dir: String
}

#[allow(non_camel_case_types)]
/// Currently supported platforms for genereting Swift Packages.
pub enum PackagePlatform {
	/// `aarch64-apple-ios`
	iOS,
	/// `x86_64-apple-ios`
	///
	/// iOS simulator for debugging in XCode's simulator.
	Simulator,
	/// `x86_64-apple-darwin`
	macOS,
}

/// Generates an xcframework embedded in a Swift Package from the Rust project.
/// 
/// # Parameters
/// - `config`: The config for generating the swift package, contains the directory
///    containing te bridges, the paths to the libraries per platform and the output directory
pub fn generate_package(config: GeneratePackageConfig) {
	// Generate XCFramework
	
	// Generate Swift Package
}
