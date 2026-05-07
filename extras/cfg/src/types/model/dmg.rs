//! DMG options.

use std::fmt::Display;
use std::str::FromStr;

use merge::Merge;

use super::ParseError;

/// DMG options.
#[derive(Clone, Debug, Default, Merge)]
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
pub struct Dmg {
    /// DMG-CPU revision.
    #[merge(strategy = merge::option::overwrite_none)]
    pub rev: Option<Rev>,
}

impl Display for Dmg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(rev) = self.rev {
            write!(f, ":{rev}")
        } else {
            Ok(())
        }
    }
}

impl FromStr for Dmg {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Dmg { rev: None })
        } else if let Some(rev) = s.strip_prefix(':') {
            Ok(Dmg {
                rev: Some(rev.parse().map_err(|_| ParseError::Revision)?),
            })
        } else {
            Err(ParseError::Revision)
        }
    }
}

/// DMG-CPU revision.
#[derive(Copy, Clone, Debug, Default)]
#[derive(parse_display::Display, parse_display::FromStr)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "clap", value(rename_all = "verbatim"))]
#[cfg_attr(
    feature = "facet",
    derive(facet::Facet),
    facet(rename_all = "UPPERCASE")
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "UPPERCASE")
)]
#[repr(C)]
pub enum Rev {
    /// DMG-CPU 0.
    ///
    /// A rare early production variant. Differs from later revisions in its
    /// boot ROM and post-boot register state.
    #[cfg_attr(feature = "clap", value(name = "0"))]
    #[cfg_attr(feature = "facet", facet(rename = "0"))]
    #[cfg_attr(feature = "serde", serde(rename = "0"))]
    #[display("0")]
    Zero,
    /// DMG-CPU A.
    ///
    /// The first mass-produced revision. Shares the same boot ROM as B and C
    /// with no known software-observable differences between them.
    A,
    /// DMG-CPU B.
    ///
    /// A silicon revision of A. Shares the same boot ROM as A and C with no
    /// known software-observable differences between them.
    B,
    /// DMG-CPU C.
    ///
    /// A silicon revision of B. Shares the same boot ROM as A and B with no
    /// known software-observable differences between them.
    #[default]
    C,
}
