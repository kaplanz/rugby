//! Command-line interface.

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Args, Parser, ValueHint};
use rugby_cfg::Config;

use crate::NAME;

/// Emulate the Nintendo Game Boy.
///
/// Cycle-accurate emulation with support for custom palettes, configurable
/// speed, interactive debugging, and more!
#[derive(Debug, Parser)]
#[clap(name = NAME, author, version, about, long_about)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    /// Configuration file.
    ///
    /// When options are specified in multiple locations, they will be applied
    /// with the following precedence: cli > env > file.
    #[clap(long, env = rugby_cfg::env::CFG)]
    #[clap(value_name = "PATH")]
    #[clap(default_value_os_t = crate::cfg::path())]
    #[clap(hide_default_value = std::env::var(rugby_cfg::env::CFG).is_ok())]
    #[clap(hide_env_values    = std::env::var(rugby_cfg::env::CFG).is_err())]
    #[clap(value_hint = ValueHint::FilePath)]
    pub conf: PathBuf,

    /// Runtime options.
    #[clap(flatten)]
    #[clap(next_help_heading = "Runtime")]
    pub run: Runtime,

    /// Configuration data.
    #[clap(flatten)]
    #[clap(next_help_heading = None)]
    pub cfg: Config,

    /// Serial connection.
    #[clap(flatten)]
    #[clap(next_help_heading = "Serial")]
    pub link: Option<Link>,

    /// Debugging options.
    #[clap(flatten)]
    #[clap(next_help_heading = "Debug")]
    pub dbg: Debug,
}

/// Runtime options.
#[derive(Args, Debug)]
pub struct Runtime {
    /// Exit without running.
    ///
    /// Instead of entering the main emulation loop, return immediately after
    /// loading the cartridge ROM. This option could be used along with
    /// `--check` to validate a ROM, or, if logging is enabled, to print the
    /// cartridge header without emulating.
    #[clap(short = 'x', long)]
    pub exit: bool,

    /// Run in headless mode.
    ///
    /// Starts without initializing or opening the UI.
    #[clap(short = 'H', long)]
    pub headless: bool,
}

/// Serial connection.
#[derive(Args, Debug)]
#[group(requires_all = ["host", "peer"])]
pub struct Link {
    /// Link cable local address.
    ///
    /// Binds a local UDP socket to the specified address for serial
    /// communications.
    #[clap(long)]
    #[clap(value_name = "ADDR")]
    #[clap(required = false)]
    pub host: SocketAddr,

    /// Link cable peer address.
    ///
    /// Opens a UDP socket at the specified address for serial communications.
    #[clap(long)]
    #[clap(value_name = "ADDR")]
    #[clap(required = false)]
    pub peer: SocketAddr,
}

/// Debugging options.
#[derive(Args, Debug)]
pub struct Debug {
    /// Enable interactive debugging.
    ///
    /// Starts with Game Boy Debugger (GBD) enabled, presenting the prompt after
    /// loading.
    #[cfg(feature = "gbd")]
    #[clap(short = 'i', long)]
    pub gbd: bool,

    /// Introspective tracing.
    #[cfg(feature = "trace")]
    #[clap(flatten)]
    pub trace: Option<trace::Trace>,

    /// Enable VRAM debug windows.
    ///
    /// Starts with debug windows opened, visually rendering VRAM contents.
    #[cfg(feature = "win")]
    #[clap(long)]
    pub win: bool,
}

/// Introspective tracing.
#[cfg(feature = "trace")]
pub mod trace {
    use std::path::PathBuf;

    use clap::{Args, ValueEnum};

    /// Introspective tracing.
    #[derive(Args, Debug)]
    #[group(requires = "trace")]
    pub struct Trace {
        /// Enable introspective tracing.
        ///
        /// Produces tracing logs of the emulator's state in the requested
        /// format.
        #[clap(name = "trace")]
        #[clap(long)]
        #[clap(required = false)]
        #[clap(value_name = "FORMAT")]
        pub fmt: Format,

        /// Output tracing logfile.
        ///
        /// Defines the path where tracing output will be logged.
        #[clap(name = "tracelog")]
        #[clap(long)]
        #[clap(requires = "trace")]
        #[clap(value_name = "PATH")]
        pub log: Option<PathBuf>,
    }

    /// Tracing output format.
    #[derive(Clone, Copy, Debug, ValueEnum)]
    #[non_exhaustive]
    pub enum Format {
        /// Tracing format used by Ben Smith's binjgb emulator.
        ///
        /// [binjgb]: https://github.com/binji/binjgb
        Binjgb,
        /// Tracing format specified by Robert Heaton's Gameboy Doctor.
        ///
        /// [gbdoc]: https://robertheaton.com/gameboy-doctor
        Doctor,
    }
}
