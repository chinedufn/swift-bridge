#![deny(missing_docs)]
//! # The swift-bridge CLI.
//!


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
