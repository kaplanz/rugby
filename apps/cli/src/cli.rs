//! Command-line interface.

use std::path::PathBuf;

use clap::{Args, Parser, ValueHint};
use clap_verbosity_flag::Verbosity;

use crate::cfg::Config;
use crate::{exe, NAME};

/// Emulate the Nintendo Game Boy.
///
/// Cycle-accurate emulation with support for custom palettes, configurable
/// speed, interactive debugging, and more!
#[derive(Debug, Parser)]
#[clap(name = NAME, author, version, about, long_about)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    /// Execution mode.
    #[clap(subcommand)]
    pub cmd: Command,

    // Logging verbosity.
    #[clap(flatten)]
    #[clap(next_help_heading = "Logging")]
    pub log: Verbosity,
}

/// Execution mode.
#[derive(Debug, Parser)]
#[clap(name = NAME)]
#[clap(disable_help_subcommand = true)]
#[non_exhaustive]
pub enum Command {
    /// Analyze provided ROM.
    Check(Box<exe::check::Cli>),
    /// Emulate provided ROM.
    Run(Box<exe::run::Cli>),
    /// Generate static files.
    Gen(Box<exe::gen::Cli>),
    /// Show help information.
    Help(Box<exe::help::Cli>),
}

/// Configuration options.
#[derive(Args, Debug)]
pub struct Settings {
    /// Configuration file.
    ///
    /// When options are specified in multiple locations, they will be applied
    /// with the following precedence: cli > env > file.
    #[clap(long = "conf", env = rugby_cfg::env::CFG)]
    #[clap(value_name = "PATH")]
    #[clap(default_value_os_t = crate::cfg::path())]
    #[clap(help_heading = None)]
    #[clap(hide_default_value = std::env::var(rugby_cfg::env::CFG).is_ok())]
    #[clap(hide_env_values    = std::env::var(rugby_cfg::env::CFG).is_err())]
    #[clap(value_hint = ValueHint::FilePath)]
    pub path: PathBuf,

    /// Configuration data.
    #[clap(flatten)]
    #[clap(next_help_heading = "Settings")]
    pub data: Config,
}
