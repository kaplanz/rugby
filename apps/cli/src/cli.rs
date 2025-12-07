//! Command-line interface.

use std::path::PathBuf;

use clap::builder::styling;
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
    #[command(next_display_order = 100)]
    pub log: Verbosity,
}

/// Execution mode.
#[derive(Debug, Parser)]
#[command(name = NAME)]
#[command(disable_help_subcommand = true)]
#[command(
    styles = styling::Styles::styled()
        .header(styling::AnsiColor::BrightGreen.on_default().bold())
        .usage(styling::AnsiColor::BrightGreen.on_default().bold())
        .literal(styling::AnsiColor::BrightCyan.on_default().bold())
        .placeholder(styling::AnsiColor::BrightCyan.on_default())
        .error(annotate_snippets::renderer::DEFAULT_ERROR_STYLE)
        .valid(styling::AnsiColor::BrightCyan.on_default().bold())
        .invalid(annotate_snippets::renderer::DEFAULT_WARNING_STYLE)
)]
#[non_exhaustive]
pub enum Command {
    /// Check header for ROM.
    #[command(name = "check")]
    #[command(visible_alias = "c")]
    Chk(Box<exe::chk::Cli>),
    /// Play ROM in emulator.
    #[command(visible_alias = "r")]
    Run(Box<exe::run::Cli>),
    /// Generate app support files.
    Gen(Box<exe::r#gen::Cli>),
    /// Display docs for a command.
    #[command(name = "help")]
    #[command(visible_alias = "h")]
    #[command(alias = "man")]
    Man(Box<exe::man::Cli>),
}

/// Configuration options.
#[derive(Args, Debug)]
#[command(next_help_heading = "Settings")]
pub struct Settings {
    /// Configuration file.
    ///
    /// When options are specified in multiple locations, they will be applied
    /// with the following precedence: cli > env > file.
    #[arg(long = "config", env = rugby::extra::cfg::env::CFG)]
    #[arg(value_name = "PATH")]
    #[arg(value_hint = ValueHint::FilePath)]
    #[arg(default_value_os_t = crate::cfg::path())]
    #[arg(help_heading = None)]
    #[arg(hide_default_value = std::env::var(rugby::extra::cfg::env::CFG).is_ok())]
    #[arg(hide_env_values    = std::env::var(rugby::extra::cfg::env::CFG).is_err())]
    #[arg(help_heading = None)]
    #[arg(display_order = 1)]
    pub path: PathBuf,

    /// Configuration data.
    #[command(flatten)]
    pub data: Config,
}
