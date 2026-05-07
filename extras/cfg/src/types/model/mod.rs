//! Hardware model values.

pub mod dmg;

/// Hardware model selection.
///
/// Selects the emulated hardware platform.
#[derive(Clone, Debug)]
#[derive(parse_display::Display, parse_display::FromStr)]
pub enum Model {
    /// DMG options.
    #[display("dmg{0}")]
    Dmg(dmg::Dmg),
}

impl Default for Model {
    fn default() -> Self {
        Model::Dmg(dmg::Dmg::default())
    }
}

/// A type specifying categories of [`Model`] parse error.
#[derive(Clone, Debug)]
#[derive(thiserror::Error)]
pub enum ParseError {
    /// Unknown platform.
    #[error("unknown platform")]
    Platform,
    /// Unknown revision.
    #[error("unknown revision")]
    Revision,
}

#[cfg(feature = "clap")]
pub use self::imp::ValueParser;

#[cfg(feature = "clap")]
mod imp {
    use clap::ValueEnum;
    use clap::builder::{PossibleValue, StringValueParser, TypedValueParser};

    use super::{Model, dmg};

    #[derive(Clone)]
    pub struct ValueParser;

    impl TypedValueParser for ValueParser {
        type Value = Model;

        fn parse_ref(
            &self,
            cmd: &clap::Command,
            arg: Option<&clap::Arg>,
            val: &std::ffi::OsStr,
        ) -> Result<Self::Value, clap::Error> {
            StringValueParser::new()
                .try_map(|s| s.parse::<Model>())
                .parse_ref(cmd, arg, val)
        }

        fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue>>> {
            Some(Box::new(
                dmg::Rev::value_variants()
                    .iter()
                    .filter_map(|rev| rev.to_possible_value().map(|pv| (rev, pv)))
                    .map(|(rev, pv)| {
                        pv.get_help().into_iter().fold(
                            PossibleValue::new(
                                Model::Dmg(dmg::Dmg { rev: Some(*rev) }).to_string(),
                            ),
                            |val, help| val.help(help.clone()),
                        )
                    }),
            ))
        }
    }
}
