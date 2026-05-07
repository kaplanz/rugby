//! Hardware model options.

use merge::Merge;

use crate::types::model::dmg;

/// Hardware model options.
///
/// Options for emulated hardware platforms.
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
pub struct Model {
    /// DMG options.
    #[cfg_attr(feature = "clap", arg(skip))]
    pub dmg: dmg::Dmg,
}
