//! Cartridge configuration.

use merge::Merge;

pub use crate::types::When;

/// Cartridge options.
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
#[cfg_attr(feature = "clap", command(next_help_heading = "Cart"))]
pub struct Cart {
    /// Check cartridge integrity.
    ///
    /// Verifies that both the header and global checksums match the data within
    /// the ROM.
    #[cfg_attr(feature = "clap", arg(short, long, conflicts_with("force")))]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub check: bool,

    /// Force cartridge construction.
    ///
    /// Causes the cartridge generation to always succeed, even if the ROM does
    /// not contain valid data.
    #[cfg_attr(feature = "clap", arg(short, long))]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub force: bool,

    /// Cartridge RAM persistence.
    ///
    /// This option can be used to override the cartridge's hardware support for
    /// persistent RAM. When enabled, RAM will be loaded and saved from a file
    /// with the same path and name as the ROM, but using the ".sav" extension.
    #[cfg_attr(
        feature = "clap",
        arg(short = 'S', long, value_name = "WHEN", value_enum)
    )]
    #[merge(strategy = merge::option::overwrite_none)]
    pub save: Option<When>,
}
