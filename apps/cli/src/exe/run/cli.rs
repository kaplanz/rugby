//! Command-line interface.

use std::net::SocketAddr;

use clap::{Args, Parser};

use super::NAME;
use crate::cli::Settings;

/// Emulate provided ROM.
#[derive(Debug, Parser)]
#[command(name = NAME)]
#[command(arg_required_else_help = true)]
#[group(id = "Run")]
pub struct Cli {
    /// Runtime features.
    #[command(flatten)]
    #[command(next_help_heading = "Features")]
    pub feat: Features,

    /// Configuration options.
    #[command(flatten)]
    pub cfg: Settings,

    /// Debugging options.
    #[cfg(feature = "debug")]
    #[command(flatten)]
    #[command(next_help_heading = "Debug")]
    pub dbg: Debugger,
}

/// Runtime features.
#[derive(Args, Debug)]
pub struct Features {
    /// Exit after instantiation.
    ///
    /// Instead of entering the main emulation loop, exit immediately after
    /// emulator instantiation is complete.
    #[arg(short = 'x', long)]
    pub exit: bool,

    /// Run in headless mode (command-line only).
    ///
    /// Starts without initializing or opening the UI. This is often useful when
    /// debugging to prevent the GUI from taking focus in your OS.
    #[arg(short = 'H', long)]
    pub headless: bool,

    /// Run without audio.
    ///
    /// Starts with the audio subsystem disabled.
    #[arg(short = 'M', long)]
    pub mute: bool,

    /// Serial connection.
    #[command(flatten)]
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
    #[arg(long)]
    #[arg(value_name = "ADDR")]
    #[arg(required = false)]
    pub host: SocketAddr,

    /// Link cable peer address.
    ///
    /// Opens a UDP socket at the specified address for serial communications.
    #[arg(long)]
    #[arg(value_name = "ADDR")]
    #[arg(required = false)]
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
    #[arg(short = 'i', long)]
    pub gbd: bool,

    /// Graphics debug windows.
    ///
    /// Enables debug windows for visually rendering contents of VRAM.
    #[cfg(feature = "gfx")]
    #[arg(long)]
    pub gfx: bool,

    /// Introspective tracing.
    #[cfg(feature = "trace")]
    #[command(flatten)]
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
        /// Enable tracing with format.
        ///
        /// Enables tracing of emulated cycles using the specified format. For
        /// more details on these formats please see their corresponding
        /// projects.
        #[arg(name = "trace")]
        #[arg(long)]
        #[arg(required = false)]
        #[arg(value_name = "FORMAT")]
        pub fmt: Format,

        /// Path to output generated logfile.
        ///
        /// An optional file for logging tracing output. If unspecified or "-",
        /// the standard output stream is used.
        #[arg(name = "logfile")]
        #[arg(long)]
        #[arg(conflicts_with = "compare")]
        #[arg(value_name = "PATH")]
        pub log: Option<PathBuf>,

        /// Compare against existing logfile.
        ///
        /// Instead of emitting trace logs, perform line-by-line comparison
        /// using the supplied tracing logfile. This will continue until the
        /// emulator either diverges from or reaches the end of the provided
        /// logfile.
        #[arg(name = "compare")]
        #[arg(long)]
        #[arg(conflicts_with = "logfile")]
        #[arg(value_name = "PATH")]
        pub cmp: Option<PathBuf>,
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
