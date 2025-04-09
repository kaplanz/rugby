//! Configurable values.

use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use rugby_core::dmg::{FREQ, ppu};
use rugby_pal as pal;
use thiserror::Error;

/// When to enable.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum When {
    /// Never enable.
    Never,
    /// Smartly enable.
    #[default]
    Auto,
    /// Always enable.
    Always,
}

/// Emulator palette selection.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
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

/// Simulated clock frequency.
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum Speed {
    /// Actual hardware speed.
    ///
    /// The real clock frequency used by the actual hardware. Equal to 4 MiHz
    /// (approx. 59.7 FPS).
    #[default]
    Actual,
    /// Speedup ratio.
    ///
    /// Multiple of the actual hardware speed. May be a floating point.
    #[cfg_attr(feature = "serde", serde(rename = "x"))]
    Ratio(f32),
    /// Clock frequency.
    ///
    /// Precise frequency (Hz) to clock the emulator. Must be an integer.
    #[cfg_attr(feature = "serde", serde(rename = "hz"))]
    Clock(u32),
    /// Frame rate.
    ///
    /// Frequency that targets the supplied frame rate (FPS). Must be an
    /// integer.
    #[cfg_attr(feature = "serde", serde(rename = "fps"))]
    Frame(u8),
    /// Maximum possible speed.
    ///
    /// Unconstrained, limited only by the host system's capabilities.
    Turbo,
}

impl Speed {
    /// Converts the `Speed` to its corresponding frequency.
    #[rustfmt::skip]
    #[must_use]
    pub fn freq(&self) -> Option<u32> {
        match *self {
            Speed::Actual      => Some(FREQ),
            Speed::Clock(freq) => Some(freq),
            #[expect(clippy::cast_possible_truncation)]
            #[expect(clippy::cast_precision_loss)]
            #[expect(clippy::cast_sign_loss)]
            Speed::Ratio(mult) => Some((FREQ as f32 * mult) as u32),
            Speed::Frame(rate) => Some(u32::from(rate) * ppu::RATE),
            Speed::Turbo       => None,
        }
    }
}

impl FromStr for Speed {
    type Err = ParseSpeedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure string is non-empty
        if s.is_empty() {
            Err(Self::Err::Empty)
        }
        // Match on unit variants
        //
        // Actual speed
        else if s == "actual" {
            Ok(Speed::Actual)
        }
        // Maximum speed
        else if s == "turbo" {
            Ok(Speed::Turbo)
        }
        // Try parsing with suffix
        //
        // Speedup ratio
        else if let Some(mult) = s.strip_suffix('x') {
            mult.parse().map(Speed::Ratio).map_err(Into::into)
        }
        // Clock frequency
        else if let Some(freq) = s.strip_suffix("hz") {
            freq.parse().map(Speed::Clock).map_err(Into::into)
        }
        // Frame rate
        else if let Some(rate) = s.strip_suffix("fps") {
            rate.parse().map(Speed::Frame).map_err(Into::into)
        }
        // Otherwise, unknown format
        else {
            Err(Self::Err::Unknown)
        }
    }
}

/// A type specifying categories of [`Color`] error.
#[derive(Clone, Debug, Error)]
pub enum ParseSpeedError {
    /// Parse string was empty.
    #[error("empty string")]
    Empty,
    /// Failure parsing an integer.
    #[error("invalid integer: {0}")]
    ParseInt(#[from] ParseIntError),
    /// Failure parsing a floating point.
    #[error("invalid float: {0}")]
    ParseFloat(#[from] ParseFloatError),
    /// Unknown parse format.
    #[error("unknown format")]
    Unknown,
}

#[cfg(feature = "clap")]
pub use imp::SpeedValueParser;

#[cfg(feature = "clap")]
mod imp {
    use std::ffi::OsStr;

    use clap::Error;
    use clap::builder::{PossibleValue, TypedValueParser};
    use clap::error::{ContextKind, ContextValue};

    use super::Speed;

    #[derive(Clone)]
    pub struct SpeedValueParser;

    impl TypedValueParser for SpeedValueParser {
        type Value = Speed;

        fn parse_ref(
            &self,
            cmd: &clap::Command,
            arg: Option<&clap::Arg>,
            val: &OsStr,
        ) -> Result<Self::Value, Error> {
            let val = val.to_str().ok_or_else(|| {
                clap::Error::new(clap::error::ErrorKind::InvalidUtf8).with_cmd(cmd)
            })?;
            val.parse::<Speed>().map_err(|_| {
                let mut err =
                    clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
                arg.map(|arg| {
                    err.insert(
                        ContextKind::InvalidArg,
                        ContextValue::String(arg.to_string()),
                    )
                });
                err.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(val.to_string()),
                );
                err
            })
        }

        fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue>>> {
            Some(Box::new(
                vec![
                    PossibleValue::new("actual").help("Actual hardware speed"),
                    PossibleValue::new("<freq>hz").help("Clock frequency (e.g. 6291456hz)"),
                    PossibleValue::new("<mult>x").help("Speedup ratio (e.g. 1.5x)"),
                    PossibleValue::new("<rate>fps").help("Frame rate (e.g. 90fps)"),
                    PossibleValue::new("turbo").help("Maximum possible speed"),
                ]
                .into_iter(),
            ))
        }
    }
}
