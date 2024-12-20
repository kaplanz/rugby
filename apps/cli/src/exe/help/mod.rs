//! Generate static files.

use std::process;

use anyhow::Context;
use clap::CommandFactory;
use constcat::concat;
use log::trace;

use crate::err::Result;
use crate::exe::gen::man;

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-help");

/// [`Help`](crate::cli::Command::Help) entrypoint.
#[allow(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Build command
    let mut cmd = match args.cmd {
        None => crate::Cli::command(),
        Some(cli::Command::Run) => crate::exe::run::Cli::command(),
        Some(cli::Command::Info) => crate::exe::info::Cli::command(),
        Some(cli::Command::Gen) => crate::exe::gen::Cli::command(),
    }
    .flatten_help(true);
    cmd.build();

    // Generate manpage to tempfile
    let mut tmp = tempfile::NamedTempFile::new().context("could not open temporary file")?;
    trace!("using temporary file: {}", tmp.path().display());
    man::gen(cmd, tmp.as_file_mut())?;

    // Spawn manual program
    let exit = process::Command::new("man")
        .args([tmp.path()])
        .status()
        .context("could not open manual page")?;

    // Forward exit status
    if let Some(code) = exit.code() {
        process::exit(code);
    } else {
        Ok(())
    }
}
