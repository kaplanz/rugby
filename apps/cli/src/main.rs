#![warn(clippy::pedantic)]

use clap::Parser;
use rugby::NAME;

use crate::cli::{Cli, Command};
use crate::err::{Exit, Result};

mod app;
mod cfg;
mod cli;
#[cfg(feature = "debug")]
mod dbg;
mod dir;
mod drop;
mod emu;
mod err;
mod exe;
mod gui;
mod init;
mod log;
mod talk;
mod util;

/// Application entry.
fn main() -> Exit {
    // Parse args
    let args = Cli::parse();
    // Set verbosity
    log::VERBOSE
        .set(args.log)
        .expect("unable to set logger verbosity");

    // Execute subcommand
    let out = match args.cmd {
        Command::Check(cli) => {
            // rugby check
            exe::check::main(*cli)
        }
        Command::Run(cli) => {
            // rugby run
            exe::run::main(*cli)
        }
        Command::Gen(cli) => {
            // rugby gen
            exe::gen::main(*cli)
        }
        Command::Help(cli) => {
            // rugby help
            exe::help::main(*cli)
        }
    };

    // Return exit status
    match out {
        Ok(()) => Exit::Success,
        Err(e) => Exit::Failure(e),
    }
}
