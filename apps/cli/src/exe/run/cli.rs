//! Command-line interface.

use std::path::PathBuf;

use rugby::extra::cfg::types::speed;

use super::NAME;
use crate::cli::Settings;

/// Play ROM in emulator.
#[derive(Debug)]
#[derive(clap::Parser)]
#[command(name = NAME)]
#[command(arg_required_else_help = true)]
#[group(id = "run::Cli")]
pub struct Cli {
    /// Configuration options.
    #[command(flatten)]
    pub cfg: Settings,

    /// Runtime options.
    #[command(flatten)]
    pub cli: Opt,

    /// Debugging options.
    #[cfg(feature = "debug")]
    #[command(flatten)]
    pub dbg: Debugger,
}

/// Runtime options.
#[derive(Debug)]
#[derive(clap::Args)]
pub struct Opt {
    /// Boot ROM options.
    #[command(flatten)]
    pub boot: Boot,

    /// Cartridge options.
    #[command(flatten)]
    pub cart: Cart,

    /// Simulated clock speed.
    ///
    /// Select from a list of possible speeds to simulate the emulator's clock.
    #[arg(short = 's', long = "speed", value_name = "SPEED")]
    #[arg(value_parser = speed::ValueParser)]
    #[arg(help_heading = None)]
    pub spd: Option<speed::Speed>,

    /// Exit after instantiation.
    ///
    /// Instead of entering the main emulation loop, exit immediately after
    /// emulator instantiation is complete.
    #[arg(short = 'x', long, help_heading = None)]
    pub exit: bool,

    /// Run without video (command-line only).
    ///
    /// Starts without initializing or opening the UI. This is often useful when
    /// debugging to prevent the GUI from taking focus in your OS.
    #[arg(short = 'H', long, help_heading = "Video", display_order = 0)]
    pub headless: bool,

    /// Run without audio.
    ///
    /// Starts with the audio subsystem disabled.
    #[arg(short = 'M', long, help_heading = "Audio", display_order = 0)]
    pub mute: bool,
}

/// Boot ROM options.
#[derive(Debug)]
#[derive(clap::Args)]
#[group(id = "run::Boot")]
#[command(next_help_heading = "Boot")]
pub struct Boot {
    /// Skip running boot ROM.
    ///
    /// Negates `-b/--boot`.
    #[arg(long = "no-boot")]
    #[arg(overrides_with = "boot")]
    #[arg(default_value_t = true)]
    #[arg(default_value_if("boot", clap::builder::ArgPredicate::IsPresent, "false"))]
    pub skip: bool,
}

/// Cartridge options.
#[derive(Debug)]
#[derive(clap::Args)]
#[group(id = "run::Cart")]
#[command(next_help_heading = "Cart")]
pub struct Cart {
    /// Cartridge ROM image file.
    ///
    /// A cartridge will be constructed from the data specified in this file.
    /// The cartridge header specifies precisely what hardware will be
    /// instantiated.
    #[arg(required_unless_present("force"))]
    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help_heading = None)]
    pub rom: Option<PathBuf>,
}

/// Debugging options.
#[derive(Debug)]
#[derive(clap::Args)]
#[command(next_help_heading = "Debug")]
pub struct Debugger {
    /// Enable debugger.
    ///
    /// Enables the Game Boy Debugger (GBD), an interactive command-line
    /// debugger which accepts commands at a prompt to control the emulator.
    #[cfg(feature = "gbd")]
    #[arg(short = 'D', long = "debug")]
    #[arg(visible_alias = "gbd")]
    pub debug: bool,

    /// Enable VRAM windows.
    ///
    /// Enables debug windows for visually rendering contents of VRAM.
    #[cfg(feature = "gfx")]
    #[arg(long = "debug-vram")]
    pub vram: bool,

    /// Introspective tracing.
    #[cfg(feature = "trace")]
    #[command(flatten)]
    pub tracer: Option<trace::Trace>,
}

/// Introspective tracing.
#[cfg(feature = "trace")]
pub mod trace {
    use std::path::PathBuf;

    /// Introspective tracing.
    #[derive(Debug)]
    #[derive(clap::Args)]
    #[group(requires = "tracer")]
    pub struct Trace {
        /// Enable tracer.
        ///
        /// Enables tracing of emulated cycles using the specified format. For
        /// more details on these formats please see their corresponding
        /// projects.
        #[arg(name = "tracer")]
        #[arg(long)]
        #[arg(required = false)]
        #[arg(value_name = "FORMAT")]
        pub fmt: Format,

        /// Logfile to dump tracer output.
        ///
        /// An optional file for logging tracing output. If unspecified or "-",
        /// the standard output stream is used.
        #[arg(name = "trace-file")]
        #[arg(long)]
        #[arg(conflicts_with = "trace-diff")]
        #[arg(value_name = "PATH")]
        pub log: Option<PathBuf>,

        /// Logfile to diff tracer output.
        ///
        /// Instead of emitting trace logs, perform line-by-line comparison
        /// using the supplied tracing logfile. This will continue until the
        /// emulator either diverges from or reaches the end of the provided
        /// logfile.
        #[arg(name = "trace-diff")]
        #[arg(long)]
        #[arg(conflicts_with = "trace-file")]
        #[arg(value_name = "PATH")]
        pub cmp: Option<PathBuf>,
    }

    /// Tracing output format.
    #[derive(Copy, Clone, Debug)]
    #[derive(clap::ValueEnum)]
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
