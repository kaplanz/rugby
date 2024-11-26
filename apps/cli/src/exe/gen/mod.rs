//! Generate static files.

use crate::err::Result;

pub mod cli;

pub use self::cli::Cli;

/// [`Gen`](crate::cli::Command::Gen) entrypoint.
#[allow(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    match args.document {
        cli::Document::Cfg => cfg::exec(),
        cli::Document::Cmp { shell } => cmp::exec(shell),
        cli::Document::Man => man::exec(),
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
        gen(buf)
    }

    /// Generate configuration file.
    pub fn gen(mut buf: impl Write) -> Result<()> {
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

    use crate::{Result, NAME};

    /// [`Cmp`](super::cli::Document::Cmp) entrypoint.
    pub fn exec(shell: Shell) -> Result<()> {
        // Build command
        let mut cmd = crate::Cli::command().flatten_help(true);
        cmd.build();
        // Declare buffer
        let buf = std::io::stdout();
        // Generate output
        gen(shell, cmd, buf)
    }

    /// Generate shell completions.
    #[allow(clippy::unnecessary_wraps)]
    pub fn gen(shell: Shell, mut cmd: Command, mut buf: impl Write) -> Result<()> {
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

    use crate::{Result, NAME};

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
    pub fn exec() -> Result<()> {
        // Build command
        let mut cmd = crate::Cli::command().flatten_help(true);
        cmd.build();
        // Declare buffer
        let mut buf = std::io::stdout();
        // Generate output
        gen(cmd, &mut buf)
    }

    /// Generate manual page.
    pub fn gen(cmd: Command, mut buf: impl Write) -> Result<()> {
        Man::new(cmd)
            .title(NAME)
            .section(MANSECT)
            .render(&mut buf)
            .context("could not generate man page")
            .map_err(Into::into)
    }
}
