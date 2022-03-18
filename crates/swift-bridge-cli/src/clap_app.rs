use clap::{Arg, Command};

/// The CLI application
pub fn cli() -> Command<'static> {
    Command::new("swift-bridge")
        .about("facilitates Rust and Swift interop.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(create_package_command())
}

/// The command for creating a Swift Package
fn create_package_command() -> Command<'static> {
    Command::new("create-package")
        .about("Create a Swift Package from Rust code.")
        .arg(
            Arg::new("bridges-dir")
                .long("bridges-dir")
                .takes_value(true)
                .value_name("PATH")
                .required(true)
                .help("The path to the generated bridge files"),
        )
        .arg(
            Arg::new("ios")
                .long("ios")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for iOS"),
        )
        .arg(
            Arg::new("simulator")
                .long("simulator")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for the iOS Simulator"),
        )
        .arg(
            Arg::new("macos")
                .long("macos")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for MacOS"),
        )
        .arg(
            Arg::new("mac-catalyst")
                .long("mac-catalyst")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for MacCatalyst"),
        )
        .arg(
            Arg::new("tvos")
                .long("tvos")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for tvOS"),
        )
        .arg(
            Arg::new("watchos")
                .long("watchos")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for WatchOS"),
        )
        .arg(
            Arg::new("watchos-simulator")
                .long("watchos-simulator")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for WatchOSSimulator"),
        )
        .arg(
            Arg::new("carplay")
                .long("carplay")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for AppleCarplay"),
        )
        .arg(
            Arg::new("carplay-simulator")
                .long("carplay-simulator")
                .takes_value(true)
                .value_name("PATH")
                .help("The path to the compiled Rust library for AppleCarplaySimulator"),
        )
        .arg(
            Arg::new("out-dir")
                .long("out-dir")
                .takes_value(true)
                .value_name("PATH")
                .required(true)
                .help("The path of the Swift Package"),
        )
        .arg(
            Arg::new("name")
                .long("name")
                .takes_value(true)
                .value_name("PATH")
                .required(true)
                .help("The name for the Swift Package"),
        )
}
