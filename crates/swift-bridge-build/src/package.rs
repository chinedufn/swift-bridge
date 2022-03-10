//! Generate a Swift Package from Rust code

use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

/// Config for generating Swift packages
pub struct CreatePackageConfig<'a> {
    /// The directory containing the generated bridges
    pub bridge_dir: &'a dyn AsRef<Path>,
    /// Path per platform. e.g. `(ApplePlatform::iOS, "target/aarch64-apple-ios/debug/libmy_rust_lib.a")`
    pub paths: HashMap<ApplePlatform, &'a dyn AsRef<Path>>,
    /// The directory where the package will be saved
    pub out_dir: &'a dyn AsRef<Path>,
    /// The name for the Swift package
    pub package_name: &'a str,
}

impl<'a> CreatePackageConfig<'a> {
    /// Creates a new `GeneratePackageConfig` for generating Swift Packages from Rust code.
    pub fn new(
        bridge_dir: &'a dyn AsRef<Path>,
        paths: HashMap<ApplePlatform, &'a dyn AsRef<Path>>,
        out_dir: &'a dyn AsRef<Path>,
        package_name: &'a str,
    ) -> Self {
        Self {
            bridge_dir,
            paths,
            out_dir,
            package_name,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
/// Currently supported platforms for genereting Swift Packages.
pub enum ApplePlatform {
    /// `aarch64-apple-ios`
    IOS,
    /// `x86_64-apple-ios`
    /// `aarch64-apple-ios-sim`
    ///
    /// iOS simulator for debugging in XCode's simulator.
    Simulator,
    /// `x86_64-apple-darwin`
    MacOS,
    /// no official Rust target for this platform
    MacCatalyst,
    /// `aarch64-apple-tvos`
    /// `x86_64-apple-tvos`
    TvOS,
    /// no official Rust target for this platform
    WatchOS,
    /// no official Rust target for this platform
    WatchOSSimulator,
    /// no official Rust target for this platform
    CarPlayOS,
    /// no official Rust target for this platform
    CarPlayOSSimulator,
}

impl ApplePlatform {
    /// The directory name inside of the xcframework for the specified platform.
    fn dir_name(&self) -> &str {
        match self {
            ApplePlatform::IOS => "ios",
            ApplePlatform::Simulator => "simulator",
            ApplePlatform::MacOS => "macos",
            ApplePlatform::MacCatalyst => "macos-mac-catalyst",
            ApplePlatform::TvOS => "tvos",
            ApplePlatform::WatchOS => "watchos",
            ApplePlatform::WatchOSSimulator => "watchos-simulator",
            ApplePlatform::CarPlayOS => "carplay",
            ApplePlatform::CarPlayOSSimulator => "carplay-simulator",
        }
    }
}

/// Generates an xcframework embedded in a Swift Package from the Rust project.
///
/// - Also see the [relevant book chapter](https://chinedufn.github.io/swift-bridge/building/swift-packages/index.html)
pub fn create_package(config: CreatePackageConfig) {
    // Create output directory //
    let output_dir: &Path = config.out_dir.as_ref();
    if !&output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("Couldn't create output directory");
    }

    // Generate XCFramework //
    gen_xcframework(&output_dir, &config);

    // Generate Swift Package //
    gen_package(&output_dir, &config);
}

/// Generates the XCFramework
fn gen_xcframework(output_dir: &Path, config: &CreatePackageConfig) {
    // Create directories
    let temp_dir = tempdir().expect("Couldn't create temporary directory");
    let tmp_framework_path = &temp_dir.path().join("swiftbridge._tmp_framework");
    fs::create_dir(&tmp_framework_path).expect("Couldn't create framework directory");
    
    let include_dir = tmp_framework_path.join("include");
    if !include_dir.exists() {
        fs::create_dir(&include_dir).expect("Couldn't create inlcude directory for xcframework");
    }

    // Create modulemap
    let modulemap_path = include_dir.join("module.modulemap");
    fs::write(
        &modulemap_path,
        "module Framework {\n    header \"SwiftBridgeCore.h\"\n",
    )
    .expect("Couldn't write modulemap file");
    let mut modulemap_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&modulemap_path)
        .expect("Couldn't open modulemap file for writing");

    // copy headers
    let bridge_dir: &Path = config.bridge_dir.as_ref();
    fs::copy(
        bridge_dir.join("SwiftBridgeCore.h"),
        &include_dir.join("SwiftBridgeCore.h"),
    )
    .expect("Couldn't copy SwiftBirdgeCore header file");
    let bridge_project_dir = fs::read_dir(&bridge_dir)
        .expect("Couldn't read generated directory")
        .find_map(|file| {
            let file = file.unwrap().path();
            if file.is_dir() {
                Some(file)
            } else {
                None
            }
        })
        .expect("Couldn't find project directory inside of generated directory");
    let bridge_project_header_dir = fs::read_dir(&bridge_project_dir)
        .expect("Couldn't read generated directory")
        .find_map(|file| {
            let file = file.unwrap().path();
            if file.extension().unwrap() == "h" {
                Some(file)
            } else {
                None
            }
        })
        .expect("Couldn't find project's header file");
    fs::copy(
        &bridge_project_header_dir,
        &include_dir.join(&bridge_project_header_dir.file_name().unwrap()),
    )
    .expect("Couldn't copy project's header file");
    writeln!(
        modulemap_file,
        "    header \"{}\"",
        bridge_project_header_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    )
    .expect("Couldn't write to modulemap");
    writeln!(modulemap_file, "    export *\n}}").expect("Couldn't write to modulemap");

    // Copy libraries
    for platform in &config.paths {
        let platform_path = &tmp_framework_path.join(platform.0.dir_name());
        if !platform_path.exists() {
            fs::create_dir(&platform_path).expect(&format!(
                "Couldn't create directory for target {:?}",
                platform.0
            ));
        }

        let lib_path: &Path = platform.1.as_ref();
        fs::copy(lib_path, platform_path.join(lib_path.file_name().unwrap())).expect(&format!(
            "Couldn't copy library for platform {:?}",
            platform.0
        ));
    }

    // build xcframework
    let xcframework_dir = output_dir.join("framework.xcframework");
    if xcframework_dir.exists() {
        fs::remove_dir_all(&xcframework_dir).expect("Couldn't delete previous xcframework file");
    }
    fs::create_dir(&xcframework_dir).expect("Couldn't create directory for xcframework");

    let mut args: Vec<String> = Vec::new();
    args.push("-create-xcframework".to_string());
    for platform in &config.paths {
        let file_path = Path::new(platform.0.dir_name())
            .join((platform.1.as_ref() as &Path).file_name().unwrap());

        args.push("-library".to_string());
        args.push(file_path.to_str().unwrap().trim().to_string());
        args.push("-headers".to_string());
        args.push("include".to_string());
    }
    args.push("-output".to_string());
    args.push(
        fs::canonicalize(xcframework_dir)
            .expect("Couldn't convert output directory to absolute path")
            .as_path()
            .to_str()
            .unwrap()
            .to_string(),
    );

    let output = Command::new("xcodebuild")
        .current_dir(&tmp_framework_path)
        .args(args)
        .output()
        .expect("Failed to execute xcodebuild");
    let stderr = std::str::from_utf8(&*output.stderr).unwrap();
    if stderr.chars().count() > 0 {
        panic!("{}", stderr);
    }

    // Remove temporary directory
    let temp_dir_string = temp_dir.path().to_str().unwrap().to_string();
    if let Err(err) = temp_dir.close() {
        eprintln!(
            "Couldn't close temporary directory {} - {}",
            temp_dir_string, err
        );
    }
}

/// Generates the Swift Package
fn gen_package(output_dir: &Path, config: &CreatePackageConfig) {
    let sources_dir = output_dir.join("Sources").join(config.package_name);
    if !sources_dir.exists() {
        fs::create_dir_all(&sources_dir).expect("Couldn't create directory for source files");
    }

    // Copy bridge `.swift` files and append import statements
    let bridge_dir: &Path = config.bridge_dir.as_ref();
    fs::write(
        sources_dir.join("SwiftBridgeCore.swift"),
        format!(
            "import Framework\n{}",
            fs::read_to_string(&bridge_dir.join("SwiftBridgeCore.swift"))
                .expect("Couldn't read core bridging swift file")
        ),
    )
    .expect("Couldn't write core bridging swift file");

    let bridge_project_dir = fs::read_dir(&bridge_dir)
        .expect("Couldn't read generated directory")
        .find_map(|file| {
            let file = file.unwrap().path();
            if file.is_dir() {
                Some(file)
            } else {
                None
            }
        })
        .expect("Couldn't find project directory inside of generated directory");
    let bridge_project_swift_dir = fs::read_dir(&bridge_project_dir)
        .expect("Couldn't read generated directory")
        .find_map(|file| {
            let file = file.unwrap().path();
            if file.extension().unwrap() == "swift" {
                Some(file)
            } else {
                None
            }
        })
        .expect("Couldn't find project's bridging swift file");
    fs::write(
        sources_dir.join(&bridge_project_swift_dir.file_name().unwrap()),
        format!(
            "import Framework\n{}",
            fs::read_to_string(&bridge_project_swift_dir)
                .expect("Couldn't read project's bridging swift file")
        ),
    )
    .expect("Couldn't copy project's bridging swift file to the package");

    // Generate Package.swift
    let package_name = config.package_name;
    let package_swift = format!(
        r#"// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
	name: "{package_name}",
	products: [
		.library(
			name: "{package_name}",
			targets: ["{package_name}"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "Framework",
			path: "framework.xcframework"
		),
		.target(
			name: "{package_name}",
			dependencies: ["Framework"])
	]
)
	"#
    );

    fs::write(output_dir.join("Package.swift"), package_swift)
        .expect("Couldn't write Package.swift file");
}
