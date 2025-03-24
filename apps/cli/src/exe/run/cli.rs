//! Command-line interface.

use std::net::SocketAddr;

use clap::{Args, Parser};

use super::NAME;
use crate::cli::Settings;

/// Emulate provided ROM.
#[derive(Debug, Parser)]
#[clap(name = NAME)]
#[clap(arg_required_else_help = true)]
#[group(id = "Run")]
pub struct Cli {
    /// Runtime features.
    #[clap(flatten)]
    #[clap(next_help_heading = "Features")]
    pub feat: Features,

    /// Configuration options.
    #[clap(flatten)]
    pub cfg: Settings,

    /// Debugging options.
    #[cfg(feature = "debug")]
    #[clap(flatten)]
    #[clap(next_help_heading = "Debug")]
    pub dbg: Debugger,
}

/// Runtime features.
#[derive(Args, Debug)]
pub struct Features {
    /// Exit after instantiation.
    ///
    /// Instead of entering the main emulation loop, exit immediately after
    /// emulator instantiation is complete.
    #[clap(short = 'x', long)]
    pub exit: bool,

    /// Run in headless mode (command-line only).
    ///
    /// Starts without initializing or opening the UI. This is often useful when
    /// debugging to prevent the GUI from taking focus in your OS.
    #[clap(short = 'H', long)]
    pub headless: bool,

    /// Serial connection.
    #[clap(flatten)]
    pub link: Option<Link>,
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
pub struct Debugger {
    /// Interactive debugging.
    ///
    /// Enables the Game Boy Debugger (GBD), an interactive command-line
    /// debugger which accepts commands at a prompt to control the emulator.
    #[cfg(feature = "gbd")]
    #[clap(short = 'i', long)]
    pub gbd: bool,

    /// Graphics debug windows.
    ///
    /// Enables debug windows for visually rendering contents of VRAM.
    #[cfg(feature = "gfx")]
    #[clap(long)]
    pub gfx: bool,

    /// Introspective tracing.
    #[cfg(feature = "trace")]
    #[clap(flatten)]
    pub trace: Option<trace::Trace>,
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
        /// Tracing enable format.
        ///
        /// Enables tracing of emulated cycles using the specified format. For
        /// more details on these formats please see their corresponding
        /// projects.
        #[clap(name = "trace")]
        #[clap(long)]
        #[clap(required = false)]
        #[clap(value_name = "FORMAT")]
        pub fmt: Format,

        /// Tracing output logfile.
        ///
        /// An optional file for logging tracing output. If unspecified or "-",
        /// the standard output stream is used.
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
