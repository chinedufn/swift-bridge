/// Config for generating Swift packages
pub struct GeneratePackageConfig {
	bridge_dir: String,
	paths: HashMap<String, String>,
	out_dir: String
}

/// Generates an xcframework embedded in a Swift Package from the Rust project.
/// 
/// Parameters
/// - `config`: The config for generating the swift package, contains the directory
///    containing te bridges, the paths to the libraries per platform and the output directory
pub fn generate_package(config: GeneratePackageConfig) {
	// Generate XCFramework

	// Generate Swift Package
}
