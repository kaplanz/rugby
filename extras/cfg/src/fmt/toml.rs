//! TOML support for [`Config`].

use std::str::FromStr;

pub use toml::de::Error;

use crate::Config;

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Parses a [`Config`] from a TOML string.
///
/// # Errors
///
/// Returns an error if the input is not valid TOML or does not match the
/// [`Config`] schema.
pub fn parse(s: &str) -> Result<Config> {
    s.parse()
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}
