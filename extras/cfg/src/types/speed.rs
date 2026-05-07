//! Clock speed values.

use parse_display::{Display, FromStr};
use rugby_core::chip::ppu;
use rugby_core::dmg::CLOCK;

/// Simulated clock frequency.
///
/// Controls how fast the emulator runs.
#[derive(Clone, Debug, Default)]
#[derive(Display, FromStr)]
#[display(style = "kebab-case")]
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
    #[display("{0}x")]
    #[cfg_attr(feature = "facet", facet(rename = "x"))]
    #[cfg_attr(feature = "serde", serde(rename = "x"))]
    Ratio(f32),
    /// Clock frequency.
    ///
    /// Precise frequency (Hz) to clock the emulator. Must be an integer.
    #[display("{0}hz")]
    #[cfg_attr(feature = "facet", facet(rename = "hz"))]
    #[cfg_attr(feature = "serde", serde(rename = "hz"))]
    Clock(u32),
    /// Frame rate.
    ///
    /// Frequency that targets the supplied frame rate (FPS). Must be an
    /// integer.
    #[display("{0}fps")]
    #[cfg_attr(feature = "facet", facet(rename = "fps"))]
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
            Speed::Actual      => Some(CLOCK),
            Speed::Clock(freq) => Some(freq),
            #[expect(clippy::cast_possible_truncation)]
            #[expect(clippy::cast_precision_loss)]
            #[expect(clippy::cast_sign_loss)]
            Speed::Ratio(mult) => Some((CLOCK as f32 * mult) as u32),
            Speed::Frame(rate) => Some(u32::from(rate) * ppu::FRAME),
            Speed::Turbo       => None,
        }
    }
}

#[cfg(feature = "clap")]
pub use self::imp::ValueParser;

#[cfg(feature = "clap")]
mod imp {
    use clap::builder::{PossibleValue, StringValueParser, TypedValueParser};

    use super::Speed;

    #[derive(Clone)]
    pub struct ValueParser;

    impl TypedValueParser for ValueParser {
        type Value = Speed;

        fn parse_ref(
            &self,
            cmd: &clap::Command,
            arg: Option<&clap::Arg>,
            val: &std::ffi::OsStr,
        ) -> Result<Self::Value, clap::Error> {
            StringValueParser::new()
                .try_map(|s| s.parse::<Speed>())
                .parse_ref(cmd, arg, val)
        }

        fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue>>> {
            Some(Box::new(
                [
                    PossibleValue::new("actual").help("Actual hardware speed"),
                    PossibleValue::new("<mult>x").help("Speedup ratio"),
                    PossibleValue::new("<freq>hz").help("Clock frequency"),
                    PossibleValue::new("<rate>fps").help("Frame rate"),
                    PossibleValue::new("turbo").help("Maximum possible speed"),
                ]
                .into_iter(),
            ))
        }
    }
}
