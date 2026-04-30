//! Clock speed values.

use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use rugby_core::chip::ppu;
use rugby_core::dmg::CLOCK;

/// Simulated clock frequency.
#[derive(Clone, Debug, Default)]
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
    #[cfg_attr(feature = "facet", facet(rename = "x"))]
    #[cfg_attr(feature = "serde", serde(rename = "x"))]
    Ratio(f32),
    /// Clock frequency.
    ///
    /// Precise frequency (Hz) to clock the emulator. Must be an integer.
    #[cfg_attr(feature = "facet", facet(rename = "hz"))]
    #[cfg_attr(feature = "serde", serde(rename = "hz"))]
    Clock(u32),
    /// Frame rate.
    ///
    /// Frequency that targets the supplied frame rate (FPS). Must be an
    /// integer.
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

impl FromStr for Speed {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(Self::Err::Empty)
        } else if s == "actual" {
            Ok(Speed::Actual)
        } else if s == "turbo" {
            Ok(Speed::Turbo)
        } else if let Some(mult) = s.strip_suffix('x') {
            mult.parse().map(Speed::Ratio).map_err(Into::into)
        } else if let Some(freq) = s.strip_suffix("hz") {
            freq.parse().map(Speed::Clock).map_err(Into::into)
        } else if let Some(rate) = s.strip_suffix("fps") {
            rate.parse().map(Speed::Frame).map_err(Into::into)
        } else {
            Err(Self::Err::Unknown)
        }
    }
}

/// A type specifying categories of [`Speed`] parse error.
#[derive(Clone, Debug)]
#[derive(thiserror::Error)]
pub enum ParseError {
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
pub(crate) use self::imp::ValueParser;

#[cfg(feature = "clap")]
mod imp {
    use std::ffi::OsStr;

    use clap::Error;
    use clap::builder::{PossibleValue, TypedValueParser};
    use clap::error::{ContextKind, ContextValue};

    use super::Speed;

    #[derive(Clone)]
    pub struct ValueParser;

    impl TypedValueParser for ValueParser {
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
