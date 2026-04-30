//! Cartridge configuration.

use std::path::PathBuf;

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
pub struct Cart {
    /// Cartridge ROM image file.
    ///
    /// A cartridge will be constructed from the data specified in this file.
    /// The cartridge header specifies precisely what hardware will be
    /// instantiated.
    #[cfg_attr(feature = "clap", arg(
        required_unless_present("force"),
        value_hint = clap::ValueHint::FilePath,
        help_heading = None,
    ))]
    #[cfg_attr(feature = "facet", facet(skip))]
    #[cfg_attr(feature = "serde", serde(skip))]
    #[merge(strategy = merge::option::overwrite_none)]
    pub rom: Option<PathBuf>,

    /// Check cartridge integrity.
    ///
    /// Verifies that both the header and global checksums match the data within
    /// the ROM.
    #[cfg_attr(
        feature = "clap",
        arg(short, long, conflicts_with("force"), help_heading = "Cartridge")
    )]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub check: bool,

    /// Force cartridge construction.
    ///
    /// Causes the cartridge generation to always succeed, even if the ROM does
    /// not contain valid data.
    #[cfg_attr(feature = "clap", arg(short, long, help_heading = "Cartridge"))]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub force: bool,

    /// Cartridge RAM persistence.
    ///
    /// This option can be used to override the cartridge's hardware support for
    /// persistent RAM. When enabled, RAM will be loaded and saved from a file
    /// with the same path and name as the ROM, but using the ".sav" extension.
    #[cfg_attr(
        feature = "clap",
        arg(
            short = 'S',
            long,
            value_name = "WHEN",
            value_enum,
            help_heading = "Cartridge"
        )
    )]
    #[merge(strategy = merge::option::overwrite_none)]
    pub save: Option<When>,
}

impl Cart {
    /// Cartridge RAM save file.
    ///
    /// The cartridge's RAM will be initialized from the data specified in this
    /// file.
    #[must_use]
    pub fn ram(&self) -> Option<PathBuf> {
        self.rom.as_ref().map(|path| path.with_extension("sav"))
    }
}
