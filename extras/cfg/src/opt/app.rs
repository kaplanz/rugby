//! Application configuration.

use std::path::Path;

use crate::Join;
pub use crate::val::{Palette, Speed};

/// Frontend options.
#[derive(Debug, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Frontend {
    /// Audio sample rate.
    ///
    /// Defines the sample rate to use for audio output. On most systems, this
    /// should be in the range of 8 Khz to 96 KHz. Unless you have a specific
    /// use case, there is no reason to change the default value.
    #[cfg_attr(
        feature = "clap",
        clap(short, long = "audio", value_name = "RATE", default_value_t = 48_000)
    )]
    #[cfg_attr(feature = "serde", serde(rename = "audio"))]
    #[expect(clippy::doc_markdown)]
    pub aux: u32,

    /// Logging filter.
    ///
    /// A comma-separated list of logging directives.
    #[cfg_attr(feature = "clap", clap(
        short, long,
        env = crate::env::LOG,
        value_name = "FILTER",
        help_heading = "Logging",
    ))]
    pub log: Option<String>,

    /// 2-bit color palette.
    ///
    /// Select from a list of preset 2-bit color palettes for the DMG model.
    /// Custom values can be defined in the configuration file.
    #[cfg_attr(
        feature = "clap",
        clap(short, long = "palette", value_name = "COLOR", value_enum)
    )]
    #[cfg_attr(feature = "serde", serde(rename = "palette"))]
    pub pal: Option<Palette>,

    /// Simulated clock speed.
    ///
    /// Select from a list of possible speeds to simulate the emulator's clock.
    /// Custom values can be defined in the configuration file.
    #[cfg_attr(
        feature = "clap",
        clap(short, long = "speed", value_name = "SPEED"),
        clap(value_parser = crate::val::SpeedValueParser)
    )]
    #[cfg_attr(feature = "serde", serde(rename = "speed"))]
    pub spd: Option<Speed>,
}

impl Join for Frontend {
    fn rebase(&mut self, _: &Path) {}

    fn merge(&mut self, other: Self) {
        self.log = self.log.take().or(other.log);
        self.pal = self.pal.take().or(other.pal);
        self.spd = self.spd.take().or(other.spd);
    }
}
