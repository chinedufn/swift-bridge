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
    let resources = parse_resource_pairs(matches);

    let mut config = CreatePackageConfig {
        bridge_dir: PathBuf::from(bridges_dir),
        paths: HashMap::new(),
        out_dir: out_dir.to_path_buf(),
        package_name: name.to_string(),
        resources,
    };

    for platform in ApplePlatform::ALL {
        if let Some(path) = matches.value_of(platform.dir_name()) {
            config.paths.insert(*platform, PathBuf::from(path));
        }
    }

    create_package(config);
}

fn parse_resource_pairs(matches: &ArgMatches) -> Vec<(PathBuf, PathBuf)> {
    let pairs: Vec<String> = matches
        .get_many("resource")
        .map(|v| v.cloned().collect())
        .unwrap_or_default();

    pairs
        .into_iter()
        .map(|pair| {
            let mut split = pair.split(':');
            let from = PathBuf::from(split.next().unwrap());
            let to = PathBuf::from(
                split
                    .next()
                    .expect(&format!("Invalid resource pair: {pair}")),
            );
            if !from.exists() {
                panic!("Resource file does not exist: {from:?}");
            }
            if to.is_absolute() {
                panic!("Resource destination must be relative: {to:?}");
            }
            (from, to)
        })
        .collect()
}
