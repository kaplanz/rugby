//! Command-line interface.

use std::path::Path;

use clap::ValueEnum;
use clap_complete::Shell;

use super::NAME;

/// Generate application support files.
#[derive(Debug)]
#[derive(clap::Parser)]
#[command(name = NAME)]
#[command(arg_required_else_help = true)]
#[command(flatten_help = true)]
#[group(id = "gen::Cli")]
pub struct Cli {
    /// Document type.
    #[command(subcommand)]
    pub doc: Document,
}

/// Generated document.
#[derive(Debug)]
#[derive(clap::Subcommand)]
#[command(disable_help_subcommand = true)]
#[non_exhaustive]
pub enum Document {
    /// Configuration file.
    #[command(disable_help_flag = true)]
    Cfg,
    /// Shell completions.
    #[command(disable_help_flag = true)]
    Cmp {
        #[arg(env = "SHELL")]
        #[arg(value_parser = |s: &str| {
            Shell::from_str(s, false).or_else(|_| {
                Path::new(s)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .and_then(|name| Shell::from_str(name, false).ok())
                    .ok_or_else(|| format!("unknown shell: {s}"))
            })
        })]
        shell: Shell,
    },
    /// Manual pages.
    #[command(disable_help_flag = true)]
    Man {
        #[arg(value_name = "COMMAND")]
        cmd: Option<Command>,
    },
}

/// Execution mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(clap::ValueEnum)]
#[non_exhaustive]
pub enum Command {
    /// Check header for ROM.
    #[value(name = "check")]
    #[value(alias = "c")]
    Chk,
    /// Play ROM in emulator.
    #[value(alias = "r")]
    Run,
    /// Generate application support files.
    Gen,
    /// Display docs for a command.
    #[value(name = "help")]
    #[value(aliases = ["h", "man"])]
    Man,
}
