use std::{path::PathBuf, process::Command};

fn main() {
    // 1. Use `swift-bridge-build` to generate Swift/C FFI glue.
    //    You can also use the `swift-bridge` CLI.
    let bridge_files = vec!["src/main.rs"];
    swift_bridge_build::parse_bridges(bridge_files)
        .write_all_concatenated(swift_bridge_out_dir(), "rust-calls-swift");

    // 2. Compile Swift library
    compile_swift();

    // 3. Link to Swift library
    println!("cargo:rustc-link-lib=static=swift-library");
    println!(
        "cargo:rustc-link-search={}",
        swift_library_static_lib_dir().to_str().unwrap()
    );

    // This fix is only for macOS
    #[cfg(target_os = "macos")]
    {
        // Without this we will get warnings about not being able to find dynamic libraries, and then
        // we won't be able to compile since the Swift static libraries depend on them:
        // For example:
        // ld: warning: Could not find or use auto-linked library 'swiftCompatibility51'
        // ld: warning: Could not find or use auto-linked library 'swiftCompatibility50'
        // ld: warning: Could not find or use auto-linked library 'swiftCompatibilityDynamicReplacements'
        // ld: warning: Could not find or use auto-linked library 'swiftCompatibilityConcurrency'
        let xcode_path = if let Ok(output) = std::process::Command::new("xcode-select")
            .arg("--print-path")
            .output()
        {
            String::from_utf8(output.stdout.as_slice().into())
                .unwrap()
                .trim()
                .to_string()
        } else {
            "/Applications/Xcode.app/Contents/Developer".to_string()
        };
        println!(
            "cargo:rustc-link-search={}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx/",
            &xcode_path
        );
        println!("cargo:rustc-link-search={}", "/usr/lib/swift");
    }

    // This fix is for Linux only
    #[cfg(target_os = "linux")]
    {
        let swift_lib_path = get_swift_lib_path().unwrap();
        println!("cargo:rustc-link-search={}", swift_lib_path);
        println!("cargo:rustc-link-lib=swiftCore");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=swiftSwiftOnoneSupport");
    }
}

fn get_swift_lib_path() -> Result<String, String> {
    let output = Command::new("which")
        .arg("swift")
        .output()
        .map_err(|e| format!("Failed to execute `which swift`: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "`which swift` failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let swift_path = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse `which swift` output: {}", e))?
        .trim()
        .to_string();

    let swift_lib_path = PathBuf::from(swift_path)
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("lib/swift_static/linux"))
        .ok_or("Failed to determine Swift library path")?;

    swift_lib_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or("Failed to convert Swift library path to string".to_string())
}

fn compile_swift() {
    let swift_package_dir = manifest_dir().join("swift-library");

    let mut cmd = Command::new("swift");

    cmd.current_dir(swift_package_dir).arg("build").args(&[
        "-Xswiftc",
        "-import-objc-header",
        "-Xswiftc",
        swift_source_dir()
            .join("bridging-header.h")
            .to_str()
            .unwrap(),
    ]);

    if is_release_build() {
        cmd.args(&["-c", "release"]);
    }

    let exit_status = cmd.spawn().unwrap().wait_with_output().unwrap();

    if !exit_status.status.success() {
        panic!(
            r#"
Stderr: {}
Stdout: {}
"#,
            String::from_utf8(exit_status.stderr).unwrap(),
            String::from_utf8(exit_status.stdout).unwrap(),
        )
    }
}

fn swift_bridge_out_dir() -> PathBuf {
    generated_code_dir()
}

fn manifest_dir() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    PathBuf::from(manifest_dir)
}

fn is_release_build() -> bool {
    std::env::var("PROFILE").unwrap() == "release"
}

fn swift_source_dir() -> PathBuf {
    manifest_dir().join("swift-library/Sources/swift-library")
}

fn generated_code_dir() -> PathBuf {
    swift_source_dir().join("generated")
}

fn swift_library_static_lib_dir() -> PathBuf {
    let debug_or_release = if is_release_build() {
        "release"
    } else {
        "debug"
    };

    manifest_dir().join(format!("swift-library/.build/{}", debug_or_release))
}
