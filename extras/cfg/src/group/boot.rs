//! Boot configuration.

use std::path::PathBuf;

use merge::Merge;

/// Boot ROM options.
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
