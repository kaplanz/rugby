//! Emulation configuration.

use std::path::PathBuf;

use merge::Merge;

pub use crate::val::When;

/// Emulator options.
#[derive(Debug, Default, Merge)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(
    feature = "facet",
    derive(facet::Facet),
    facet(default, deny_unknown_fields)
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
#[cfg_attr(
    all(feature = "facet", feature = "serde"),
    expect(clippy::unsafe_derive_deserialize)
)]
pub struct Emulator {
    /// Booting options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub boot: Boot,

    /// Cartridge options.
    #[cfg_attr(feature = "clap", command(flatten))]
    pub cart: Cart,
}

/// Booting options.
#[derive(Debug, Default, Merge)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(
    feature = "facet",
    derive(facet::Facet),
    facet(default, deny_unknown_fields)
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
#[cfg_attr(
    all(feature = "facet", feature = "serde"),
    expect(clippy::unsafe_derive_deserialize)
)]
pub struct Boot {
    /// Boot ROM image file.
    ///
    /// Path to the boot ROM image.
    ///
    /// Relative paths are resolved relative to the application's data directory
    /// (`$XDG_DATA_HOME/rugby`), typically `~/.local/share/rugby/`. Absolute
    /// paths are used as-is.
    ///
    /// If the path to the image file is specified within the configuration, it
    /// can be selected by passing `-b/--boot` without specifying an argument.
    /// Otherwise, the path to the image file must be provided.
    ///
    /// May be negated with `--no-boot`.
    #[cfg_attr(feature = "clap", arg(
        name = "boot",
        short, long,
        num_args(0..=1),
        value_name = "PATH",
        value_hint = clap::ValueHint::FilePath,
    ))]
    #[merge(strategy = merge::option::overwrite_none)]
    pub rom: Option<PathBuf>,

    /// Skip running boot ROM.
    ///
    /// Negates `-b/--boot`.
    #[cfg_attr(
        feature = "clap",
        arg(
            hide = true,
            long = "no-boot",
            overrides_with = "boot",
            default_value_t = true,
            default_value_if("boot", clap::builder::ArgPredicate::IsPresent, "false"),
        )
    )]
    #[cfg_attr(feature = "facet", facet(skip))]
    #[cfg_attr(feature = "serde", serde(skip))]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub skip: bool,
}

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
    derive(serde::Deserialize),
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
    /// The cartridge's RAM be initialized from the data specified in this file.
    #[must_use]
    pub fn ram(&self) -> Option<PathBuf> {
        self.rom.as_ref().map(|path| path.with_extension("sav"))
    }
}
