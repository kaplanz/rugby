//! Audio configuration.

use merge::Merge;

/// Audio options.
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
#[cfg_attr(feature = "clap", command(next_help_heading = "Audio"))]
pub struct Audio {
    /// Audio sample rate.
    ///
    /// Defines the sample rate to use for audio output. On most systems, this
    /// should be in the range of 8 KHz to 96 KHz. Unless you have a specific
    /// use case, there is no reason to change the default value.
    #[cfg_attr(
        feature = "clap",
        arg(long = "sample-rate", value_name = "RATE", default_value_t = 48_000)
    )]
    #[merge(strategy = merge::num::overwrite_zero)]
    #[expect(clippy::doc_markdown)]
    pub rate: u32,
}
