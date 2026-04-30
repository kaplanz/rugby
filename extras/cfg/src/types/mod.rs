//! Common field value types.

pub mod speed;

/// Specifies when an option should be active.
///
/// Used to override automatic hardware detection for features that may
/// or may not be present on a given cartridge.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "facet",
    derive(facet::Facet),
    facet(rename_all = "kebab-case")
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
#[repr(C)]
pub enum When {
    /// Never enable.
    Never,
    /// Enable automatically based on hardware support.
    #[default]
    Auto,
    /// Always enable.
    Always,
}
