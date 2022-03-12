use clap::ArgMatches;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use swift_bridge_build::{create_package, ApplePlatform, CreatePackageConfig};

/// Executes the correct function depending on the cli input
pub fn handle_matches(matches: ArgMatches) {
    match matches.subcommand_name() {
        Some("create-package") => {
            handle_create_package(matches.subcommand_matches("create-package").unwrap())
        }
        _ => unreachable!("No subcommand or unknown subcommand given"), // Shouldn't happen
    }
}

/// Executes the `create-package` command
fn handle_create_package(matches: &ArgMatches) {
    let bridges_dir = matches.value_of("bridges-dir").unwrap(); // required
    let out_dir = matches.value_of("out-dir").map(|p| Path::new(p)).unwrap(); // required
    let name = matches.value_of("name").unwrap(); // required

    let mut config = CreatePackageConfig {
        bridge_dir: PathBuf::from(bridges_dir),
        paths: HashMap::new(),
        out_dir: out_dir.to_path_buf(),
        package_name: name.to_string(),
    };

    for platform in ApplePlatform::ALL {
        if let Some(path) = matches.value_of(platform.dir_name()) {
            config.paths.insert(*platform, PathBuf::from(path));
        }
    }

    create_package(config);
}
