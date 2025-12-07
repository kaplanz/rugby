//! Command-line emulator.

#![warn(clippy::pedantic)]
// Allowed lints: clippy
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use clap::Parser;
use rugby::NAME;

use crate::cli::{Cli, Command};
use crate::err::{Exit, Result};

pub mod app;
pub mod cfg;
pub mod cli;
pub mod dir;
pub mod err;
pub mod exe;
pub mod log;

/// Application entry.
fn main() -> Exit {
    // Parse args
    let args = Cli::parse();
    // Set verbosity
    log::VLEVEL
        .set(args.log)
        .expect("unable to set verbosity level");

    // Execute subcommand
    let out = match args.cmd {
        Command::Chk(cli) => {
            // rugby check
            exe::chk::main(*cli)
        }
        Command::Run(cli) => {
            // rugby run
            exe::run::main(*cli)
        }
        Command::Gen(cli) => {
            // rugby gen
            exe::r#gen::main(*cli)
        }
        Command::Man(cli) => {
            // rugby help
            exe::man::main(*cli)
        }
    };

    // Return exit status
    match out {
        Ok(()) => Exit::Success,
        Err(e) => Exit::Failure(e),
    }
}
