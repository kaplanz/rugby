//! Command-line interface.

use std::path::PathBuf;

use clap::{Args, Parser, ValueHint};
use clap_verbosity_flag::Verbosity;

use crate::cfg::Config;
use crate::{NAME, exe};

/// Emulate the Nintendo Game Boy.
///
/// Cycle-accurate emulation with support for custom palettes, configurable
/// speed, interactive debugging, and more!
#[derive(Debug, Parser)]
#[command(name = NAME, author, version, about, long_about)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Execution mode.
    #[command(subcommand)]
    pub cmd: Command,

    // Logging verbosity.
    #[command(flatten)]
    #[command(next_help_heading = "Logging")]
    pub log: Verbosity,
}

/// Execution mode.
#[derive(Debug, Parser)]
#[command(name = NAME)]
#[command(disable_help_subcommand = true)]
#[non_exhaustive]
pub enum Command {
    /// Analyze provided ROM.
    #[command(visible_alias = "c")]
    Check(Box<exe::check::Cli>),
    /// Emulate provided ROM.
    #[command(visible_alias = "r")]
    Run(Box<exe::run::Cli>),
    /// Generate static files.
    Gen(Box<exe::r#gen::Cli>),
    /// Show help information.
    #[command(alias = "man")]
    Help(Box<exe::help::Cli>),
}

/// Configuration options.
#[derive(Args, Debug)]
pub struct Settings {
    /// Configuration file.
    ///
    /// When options are specified in multiple locations, they will be applied
    /// with the following precedence: cli > env > file.
    #[arg(long = "conf", env = rugby::extra::cfg::env::CFG)]
    #[arg(value_name = "PATH")]
    #[arg(value_hint = ValueHint::FilePath)]
    #[arg(default_value_os_t = crate::cfg::path())]
    #[arg(help_heading = None)]
    #[arg(hide_default_value = std::env::var(rugby::extra::cfg::env::CFG).is_ok())]
    #[arg(hide_env_values    = std::env::var(rugby::extra::cfg::env::CFG).is_err())]
    pub path: PathBuf,

    /// Configuration data.
    #[command(flatten)]
    #[command(next_help_heading = "Settings")]
    pub data: Config,
}
