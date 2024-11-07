#![warn(clippy::pedantic)]

use clap::Parser;

use crate::cli::{Cli, Command};
use crate::err::{Exit, Result};

mod app;
mod cfg;
mod cli;
#[cfg(feature = "debug")]
mod dbg;
mod dir;
mod err;
mod exe;
mod init;
mod util;

/// Application name.
///
/// This may be used for base subdirectories.
const NAME: &str = env!("CARGO_CRATE_NAME");

/// Application entry.
fn main() -> Exit {
    // Parse args
    let args = Cli::parse();

    // Execute subcommand
    let out = args.cmd.map_or_else(
        // default subcommand
        || exe::run::main(args.run),
        // perform subcommand
        |cmd| match cmd {
            Command::Run(cli) => {
                // rugby run
                exe::run::main(*cli)
            }
            Command::Info(cli) => {
                // rugby info
                exe::info::main(*cli)
            }
        },
    );

    // Return exit status
    match out {
        Ok(()) => Exit::Success,
        Err(e) => Exit::Failure(e),
    }
}
