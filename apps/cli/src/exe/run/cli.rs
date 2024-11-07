//! Command-line interface.

use std::net::SocketAddr;

use clap::Args;

use crate::cli::Settings;

/// [Run](super::exec) options.
#[derive(Args, Debug)]
#[clap(arg_required_else_help = true)]
#[group(id = "Run")]
pub struct Cli {
    /// Runtime features.
    #[clap(flatten)]
    #[clap(next_help_heading = "Features")]
    pub feat: Features,

    /// Configuration options.
    #[allow(private_interfaces)]
    #[clap(flatten)]
    pub cfg: Settings,

    /// Debugging options.
    #[clap(flatten)]
    #[clap(next_help_heading = "Debug")]
    pub dbg: Debug,
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
pub struct Debug {
    /// Enable interactive debugging.
    ///
    /// Starts with Game Boy Debugger (GBD) enabled, presenting the prompt after
    /// loading.
    #[cfg(feature = "gbd")]
    #[clap(short = 'i', long)]
    pub gbd: bool,

    /// Introspective tracing.
    #[cfg(feature = "log")]
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
#[cfg(feature = "log")]
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
