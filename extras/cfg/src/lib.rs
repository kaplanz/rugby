//! Emulator configuration.

#![warn(clippy::pedantic)]

use ::merge::Merge;

use crate::opt::{Emulator, Frontend};

mod val;

pub mod env;
pub mod opt;

/// Configuration object.
#[derive(Debug, Default, Merge)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(
    feature = "facet",
    derive(facet::Facet),
    facet(default, deny_unknown_fields)
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, deny_unknown_fields)
)]
#[cfg_attr(
    all(feature = "facet", feature = "serde"),
    expect(clippy::unsafe_derive_deserialize)
)]
pub struct Config {
    /// Frontend options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub app: Frontend,

    /// Emulator options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub emu: Emulator,
}

/// Deserializing configuration.
#[cfg(feature = "toml")]
pub mod de {
    use std::str::FromStr;

    pub use toml::de::Error;

    use crate::Config;

    impl FromStr for Config {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            toml::from_str(s)
        }
    }
}
