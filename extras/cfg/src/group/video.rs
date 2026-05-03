//! Video configuration.

use merge::Merge;
use rugby_pal as pal;

/// Video options.
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
#[cfg_attr(feature = "clap", command(next_help_heading = "Video"))]
pub struct Video {
    /// 2-bit color palette.
    ///
    /// Select from a list of preset 2-bit color palettes for the DMG model.
    /// Custom values can be defined in the configuration file.
    #[cfg_attr(
        feature = "clap",
        arg(
            short = 'p',
            long = "palette",
            visible_alias = "pal",
            value_name = "COLOR",
            value_enum
        )
    )]
    #[cfg_attr(feature = "facet", facet(rename = "palette"))]
    #[cfg_attr(feature = "serde", serde(rename = "palette"))]
    #[merge(strategy = merge::option::overwrite_none)]
    pub pal: Option<Palette>,
}

/// 2-bit color palette selection.
#[derive(Clone, Debug, Default)]
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
#[non_exhaustive]
#[repr(C)]
pub enum Palette {
    /// Nostalgic autumn sunsets.
    AutumnChill,
    /// Aquatic blues.
    BlkAqu,
    /// Winter snowstorm blues.
    BlueDream,
    /// Combining cool and warm tones.
    Coldfire,
    /// Soft and pastel coral hues.
    Coral,
    /// Cold metallic darks with warm dated plastic lights.
    Demichrome,
    /// Greens and warm browns with an earthy feel.
    Earth,
    /// Creamsicle inspired orange.
    IceCream,
    /// Old-school dot-matrix display.
    Legacy,
    /// Misty forest greens.
    Mist,
    /// Simple blacks and whites.
    #[default]
    Mono,
    /// William Morris's rural palette.
    Morris,
    /// Waterfront at dawn.
    PurpleDawn,
    /// Rusty red and brown hues.
    Rustic,
    /// Deep and passionate purples.
    VelvetCherry,
    /// Whatever colors you want!
    #[cfg_attr(feature = "clap", value(skip))]
    Custom(pal::Palette),
}

#[rustfmt::skip]
impl From<Palette> for pal::Palette {
    fn from(value: Palette) -> Self {
        match value {
            Palette::AutumnChill  => pal::AUTUMN_CHILL,
            Palette::BlkAqu       => pal::BLK_AQU,
            Palette::BlueDream    => pal::BLUE_DREAM,
            Palette::Coldfire     => pal::COLDFIRE,
            Palette::Coral        => pal::CORAL,
            Palette::Demichrome   => pal::DEMICHROME,
            Palette::Earth        => pal::EARTH,
            Palette::IceCream     => pal::ICE_CREAM,
            Palette::Legacy       => pal::LEGACY,
            Palette::Mist         => pal::MIST,
            Palette::Mono         => pal::MONO,
            Palette::Morris       => pal::MORRIS,
            Palette::PurpleDawn   => pal::PURPLE_DAWN,
            Palette::Rustic       => pal::RUSTIC,
            Palette::VelvetCherry => pal::VELVET_CHERRY,
            Palette::Custom(pal)  => pal,
        }
    }
}
