use clap::ArgMatches;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use swift_bridge_build::{create_package, parse_bridges, ApplePlatform, CreatePackageConfig};

/// Executes the correct function depending on the cli input
pub fn handle_matches(matches: ArgMatches) {
    match matches.subcommand_name() {
        Some(cmd @ "create-package") => {
            handle_create_package(matches.subcommand_matches(cmd).unwrap())
        }
        Some(cmd @ "parse-bridges") => {
            handle_parse_bridges(matches.subcommand_matches(cmd).unwrap())
        }
        _ => unreachable!("No subcommand or unknown subcommand given"), // Shouldn't happen
    }
}

/// Executes the `create-package` command
fn handle_create_package(matches: &ArgMatches) {
    let bridges_dir = matches.value_of("bridges-dir").unwrap(); // required
    let out_dir = matches.value_of("out-dir").map(Path::new).unwrap(); // required
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

/// Executes the `parse-bridges` command
fn handle_parse_bridges(matches: &ArgMatches) {
    let crate_name = matches.get_one::<String>("crate-name").unwrap(); // required
    let source_files: Vec<&String> = matches.get_many("source-file").unwrap().collect(); // required
    let output = matches.get_one::<String>("output").map(Path::new).unwrap(); // required

    parse_bridges(source_files.iter().map(Path::new)).write_all_concatenated(output, crate_name);
}
