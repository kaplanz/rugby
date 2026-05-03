//! Configuration schema for `rugby`.
//!
//! This crate defines [`Config`], a common configuration schema for emulator
//! frontends.
//!
//! # Features
//!
//! - [`clap`]: Parse CLI arguments and environment variables.
//! - [`toml`]: Load configuration from a TOML document.
//!
//! # Layering
//!
//! To layer configuration from different sources, call [`Config::merge`] to
//! perform a layered merge. A merge is always performed right onto left,
//! meaning the left's values are overwritten by the right's, if present.

#![warn(clippy::pedantic)]

use merge::Merge;

pub mod env;
pub mod fmt;
pub mod group;
pub mod types;

pub use self::group::{Audio, Boot, Cable, Cart, Input, Palette, Video};

/// Emulator configuration.
///
/// Aggregates configuration sections and options into a single object. Designed
/// for layered use by [merging](Self::merge) configuration objects from
/// different sources.
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
    /// Logging filter.
    ///
    /// A comma-separated list of logging directives.
    #[cfg_attr(feature = "clap", arg(
        short, long,
        env = crate::env::LOG,
        value_name = "FILTER",
        help_heading = None,
    ))]
    #[merge(strategy = merge::option::overwrite_none)]
    pub log: Option<String>,

    /// Audio options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub audio: group::Audio,

    /// Video options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub video: group::Video,

    /// Input options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub input: group::Input,

    /// Cable options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub cable: group::Cable,

    /// Boot ROM options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub boot: group::Boot,

    /// Cartridge options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub cart: group::Cart,
}
