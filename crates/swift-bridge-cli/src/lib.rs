//! # The swift-bridge CLI.

#![deny(missing_docs)]

mod clap_app;
mod clap_exec;

/// Contains everything related parsing command input and executing
pub mod app {
    pub use crate::clap_app::*;
    pub use crate::clap_exec::*;

    /// Execute the CLI
    pub fn run() {
        handle_matches(cli().get_matches());
    }
}
