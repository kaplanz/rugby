//! Generate static files.

use anyhow::Context;
use constcat::concat;
use log::trace;

use crate::err::Result;

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-gen");

/// [`Gen`](crate::cli::Command::Gen) entrypoint.
#[expect(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Initialize logger
    crate::log::init(None).context("logger initialization failed")?;
    // Log arguments
    trace!("{args:#?}");

    // Execute subcommand
    match args.document {
        cli::Document::Cfg => cfg::exec(),
        cli::Document::Cmp { shell } => cmp::exec(shell),
        cli::Document::Man { cmd } => man::exec(cmd),
    }
}

/// Configuration file.
pub mod cfg {
    use std::io::Write;

    use anyhow::Context;

    use crate::Result;

    /// [`Cfg`](super::cli::Document::Cfg) entrypoint.
    pub fn exec() -> Result<()> {
        // Declare buffer
        let buf = std::io::stdout();
        // Generate output
        r#gen(buf)
    }

    /// Generate configuration file.
    pub fn r#gen(mut buf: impl Write) -> Result<()> {
        buf.write_all(include_str!("../../../rugby.toml").as_bytes())
            .context("could not generate config file")
            .map_err(Into::into)
    }
}

/// Shell completions.
pub mod cmp {
    use std::io::Write;

    use clap::{Command, CommandFactory};
    use clap_complete::Shell;

    use crate::{NAME, Result};

    /// [`Cmp`](super::cli::Document::Cmp) entrypoint.
    pub fn exec(shell: Shell) -> Result<()> {
        // Build command
        let mut cmd = crate::Cli::command().flatten_help(true);
        cmd.build();
        // Declare buffer
        let buf = std::io::stdout();
        // Generate output
        r#gen(shell, cmd, buf)
    }

    /// Generate shell completions.
    pub fn r#gen(shell: Shell, mut cmd: Command, mut buf: impl Write) -> Result<()> {
        clap_complete::generate(shell, &mut cmd, NAME, &mut buf);
        Ok(()) // unconditionally succeed
    }
}

/// Manual pages.
pub mod man {
    use std::io::Write;

    use anyhow::Context;
    use clap::{Command, CommandFactory};
    use clap_mangen::Man;

    use super::cli::Command as Subcommand;
    use crate::{NAME, Result};

    /// Manual section.
    ///
    /// The sections of the manual are:
    /// 1.  General Commands Manual
    /// 2.  System Calls Manual
    /// 3.  Library Functions Manual
    /// 4.  Kernel Interfaces Manual
    /// 5.  File Formats Manual
    /// 6.  Games Manual
    /// 7.  Miscellaneous Information Manual
    /// 8.  System Manager's Manual
    /// 9.  Kernel Developer's Manual
    pub const MANSECT: &str = "6";

    /// [`Man`](super::cli::Document::Man) entrypoint.
    pub fn exec(cmd: Option<Subcommand>) -> Result<()> {
        // Build command
        let mut cmd = match cmd {
            None => crate::Cli::command(),
            Some(Subcommand::Check) => crate::exe::check::Cli::command(),
            Some(Subcommand::Run) => crate::exe::run::Cli::command(),
            Some(Subcommand::Gen) => crate::exe::r#gen::Cli::command(),
        }
        .flatten_help(true);
        cmd.build();
        // Declare buffer
        let mut buf = std::io::stdout();
        // Generate output
        r#gen(cmd, &mut buf)
    }

    /// Generate manual page.
    pub fn r#gen(cmd: Command, mut buf: impl Write) -> Result<()> {
        Man::new(cmd)
            .title(NAME)
            .section(MANSECT)
            .render(&mut buf)
            .context("could not generate man page")
            .map_err(Into::into)
    }
}
