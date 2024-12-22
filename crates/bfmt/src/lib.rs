//! Bytes human formatter.
//!
//! This utility library easily converts between bytes as an integer and human
//! size representations.
//!
//! # Examples
//!
//! ```
//! use bfmt::Size;
//!
//! # fn main() -> Result<(), bfmt::Error> {
//! // Parse bytes from string
//! let size: Size = "1.0625 KiB".parse()?;
//!
//! // Extract the integer bytes value
//! assert_eq!(u64::from(size), 1088);
//!
//! // Format back as a string
//! //
//! // Note: Format specifiers work generally as expected!
//! assert_eq!(format!("{size:>#10.3}"), "  1.088 KB");
//! #
//! # Ok(())
//! # }
//! ```

#![allow(non_upper_case_globals)]

use std::fmt::{self, Debug, Display, Formatter};
use std::num;
use std::str::FromStr;

use thiserror::Error;

/// byte size for 1 byte
pub const B: u64 = 1;
/// bytes size for 1 kilobyte
pub const KB: u64 = 1_000;
/// bytes size for 1 kibibyte
pub const KiB: u64 = 0x400;
/// bytes size for 1 megabyte
pub const MB: u64 = 1_000_000;
/// bytes size for 1 mebibyte
pub const MiB: u64 = 0x10_0000;
/// bytes size for 1 gigabyte
pub const GB: u64 = 1_000_000_000;
/// bytes size for 1 gibibyte
pub const GiB: u64 = 0x4000_0000;
/// bytes size for 1 terabyte
pub const TB: u64 = 1_000_000_000_000;
/// bytes size for 1 tebibyte
pub const TiB: u64 = 0x100_0000_0000;
/// bytes size for 1 petabyte
pub const PB: u64 = 1_000_000_000_000_000;
/// bytes size for 1 pebibyte
pub const PiB: u64 = 0x4_0000_0000_0000;
/// bytes size for 1 exabyte
pub const EB: u64 = 1_000_000_000_000_000_000;
/// bytes size for 1 exbibyte
pub const EiB: u64 = 0x1000_0000_0000_0000;

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by converting between byte size representations.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Unknown unit suffix.
    #[error("unknown unit suffix")]
    Unit,
    /// Failure parsing size.
    #[error("failure parsing size")]
    Size,
    /// Parse number error.
    #[error(transparent)]
    Num(#[from] num::ParseFloatError),
}

/// Byte size representation
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Unit {
    #[default]
    Byte,
    Kilo,
    Kibi,
    Mega,
    Mebi,
    Giga,
    Gibi,
    Tera,
    Tebi,
    Peta,
    Pebi,
    Exa,
    Exbi,
}

impl Unit {
    #[rustfmt::skip]
    pub const fn value(&self) -> u64 {
        match self {
            Unit::Byte => B,
            Unit::Kilo => KB,
            Unit::Kibi => KiB,
            Unit::Mega => MB,
            Unit::Mebi => MiB,
            Unit::Giga => GB,
            Unit::Gibi => GiB,
            Unit::Tera => TB,
            Unit::Tebi => TiB,
            Unit::Peta => PB,
            Unit::Pebi => PiB,
            Unit::Exa  => EB,
            Unit::Exbi => EiB,
        }
    }
}

impl Display for Unit {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Unit::Byte => "B",
            Unit::Kilo => "KB",
            Unit::Kibi => "KiB",
            Unit::Mega => "MB",
            Unit::Mebi => "MiB",
            Unit::Giga => "GB",
            Unit::Gibi => "GiB",
            Unit::Tera => "TB",
            Unit::Tebi => "TiB",
            Unit::Peta => "PB",
            Unit::Pebi => "PiB",
            Unit::Exa  => "EB",
            Unit::Exbi => "EiB",
        })
    }
}

impl FromStr for Unit {
    type Err = Error;

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B"   => Ok(Unit::Byte),
            "KB"  => Ok(Unit::Kilo),
            "KIB" => Ok(Unit::Kibi),
            "MB"  => Ok(Unit::Mega),
            "MIB" => Ok(Unit::Mebi),
            "GB"  => Ok(Unit::Giga),
            "GIB" => Ok(Unit::Gibi),
            "TB"  => Ok(Unit::Tera),
            "TIB" => Ok(Unit::Tebi),
            "PB"  => Ok(Unit::Peta),
            "PIB" => Ok(Unit::Pebi),
            "EB"  => Ok(Unit::Exa),
            "EIB" => Ok(Unit::Exbi),
            _ => Err(Error::Unit),
        }
    }
}

/// Byte size representation
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Size(u64);

impl From<u64> for Size {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Size> for u64 {
    fn from(value: Size) -> Self {
        value.0
    }
}

impl From<usize> for Size {
    fn from(value: usize) -> Self {
        Self(value.try_into().unwrap())
    }
}

impl From<Size> for usize {
    fn from(value: Size) -> Self {
        value.0.try_into().unwrap()
    }
}

impl Size {
    #![allow(non_snake_case)]

    pub fn Byte<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * B)
    }

    pub fn Kilo<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * KB)
    }

    pub fn Kibi<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * KiB)
    }

    pub fn Mega<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * MB)
    }

    pub fn Mebi<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * MiB)
    }

    pub fn Giga<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * GB)
    }

    pub fn Gibi<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * GiB)
    }

    pub fn Tera<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * TB)
    }

    pub fn Tebi<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * TiB)
    }

    pub fn Peta<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * PB)
    }

    pub fn Pebi<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * PiB)
    }

    pub fn Exa<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * EB)
    }

    pub fn Exbi<V: Into<u64>>(value: V) -> Self {
        Self(value.into() * EiB)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Alternate flag specifies SI units
        let si = f.alternate();

        // Determine nearest unitary prefix
        let unit = if si {
            let order = self
                .0
                .checked_ilog10()
                .map(|log| log / 3)
                .unwrap_or_default();
            match order {
                0 => Unit::Byte,
                1 => Unit::Kilo,
                2 => Unit::Mega,
                3 => Unit::Giga,
                4 => Unit::Tera,
                5 => Unit::Peta,
                6 => Unit::Exa,
                _ => unreachable!(),
            }
        } else {
            let order = self
                .0
                .checked_ilog2()
                .map(|log| log / 10)
                .unwrap_or_default();
            match order {
                0 => Unit::Byte,
                1 => Unit::Kibi,
                2 => Unit::Mebi,
                3 => Unit::Gibi,
                4 => Unit::Tebi,
                5 => Unit::Pebi,
                6 => Unit::Exbi,
                _ => unreachable!(),
            }
        };
        let ustr = unit.to_string();

        // Convert value to chosen unit
        let base = (self.0 as f64) / (unit.value() as f64);

        // Extract format specifiers
        let precn = f.precision();

        // Format base value
        let base = if let Some(precn) = precn {
            format!("{base:.precn$}")
        } else {
            format!("{base}")
        };

        // Format base with unit
        let size = format!("{base} {ustr}");

        // Write with padding
        f.pad_integral(true, "", &size)
    }
}

impl FromStr for Size {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (base, unit) = s.trim().split_once(" ").ok_or(Error::Size)?;
        let unit: Unit = unit.parse()?;
        let value: f64 = base.parse()?;
        let bytes = (value * (unit.value() as f64)).round() as u64;
        Ok(Self(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn derived_unit_works() {
        assert_eq!("1 B",   format!("{}",   Size::Byte(B)));
        assert_eq!("1 KB",  format!("{:#}", Size::Byte(KB)));
        assert_eq!("1 KiB", format!("{}",   Size::Byte(KiB)));
        assert_eq!("1 MB",  format!("{:#}", Size::Byte(MB)));
        assert_eq!("1 MiB", format!("{}",   Size::Byte(MiB)));
        assert_eq!("1 GB",  format!("{:#}", Size::Byte(GB)));
        assert_eq!("1 GiB", format!("{}",   Size::Byte(GiB)));
        assert_eq!("1 TB",  format!("{:#}", Size::Byte(TB)));
        assert_eq!("1 TiB", format!("{}",   Size::Byte(TiB)));
        assert_eq!("1 PB",  format!("{:#}", Size::Byte(PB)));
        assert_eq!("1 PiB", format!("{}",   Size::Byte(PiB)));
        assert_eq!("1 EB",  format!("{:#}", Size::Byte(EB)));
        assert_eq!("1 EiB", format!("{}",   Size::Byte(EiB)));
    }

    #[test]
    #[rustfmt::skip]
    fn literal_unit_works() {
        assert_eq!("1 B",   format!("{}",   Size::Byte(1u64)));
        assert_eq!("1 KB",  format!("{:#}", Size::Kilo(1u64)));
        assert_eq!("1 KiB", format!("{}",   Size::Kibi(1u64)));
        assert_eq!("1 MB",  format!("{:#}", Size::Mega(1u64)));
        assert_eq!("1 MiB", format!("{}",   Size::Mebi(1u64)));
        assert_eq!("1 GB",  format!("{:#}", Size::Giga(1u64)));
        assert_eq!("1 GiB", format!("{}",   Size::Gibi(1u64)));
        assert_eq!("1 TB",  format!("{:#}", Size::Tera(1u64)));
        assert_eq!("1 TiB", format!("{}",   Size::Tebi(1u64)));
        assert_eq!("1 PB",  format!("{:#}", Size::Peta(1u64)));
        assert_eq!("1 PiB", format!("{}",   Size::Pebi(1u64)));
        assert_eq!("1 EB",  format!("{:#}", Size::Exa (1u64)));
        assert_eq!("1 EiB", format!("{}",   Size::Exbi(1u64)));
    }

    #[test]
    #[rustfmt::skip]
    fn format_value_works() {
        // unaligned
        assert_eq!("     999 B", format!("{:#10}",   Size::Byte(999u64)));
        assert_eq!("      1 KB", format!("{:#10}",   Size::Byte(1000u64)));
        assert_eq!("  1.001 KB", format!("{:#10.3}", Size::Byte(1001u64)));
        assert_eq!("    1023 B", format!("{:10}",    Size::Byte(1023u64)));
        assert_eq!("     1 KiB", format!("{:10}",    Size::Byte(1024u64)));
        assert_eq!(" 1.001 KiB", format!("{:10.3}",  Size::Byte(1025u64)));
        // aligned left
        assert_eq!("999 B     ", format!("{:<#10}",   Size::Byte(999u64)));
        assert_eq!("1 KB      ", format!("{:<#10}",   Size::Byte(1000u64)));
        assert_eq!("1.001 KB  ", format!("{:<#10.3}", Size::Byte(1001u64)));
        assert_eq!("1023 B    ", format!("{:<10}",    Size::Byte(1023u64)));
        assert_eq!("1 KiB     ", format!("{:<10}",    Size::Byte(1024u64)));
        assert_eq!("1.001 KiB ", format!("{:<10.3}",  Size::Byte(1025u64)));
        // aligned center
        assert_eq!("  999 B   ", format!("{:^#10}",   Size::Byte(999u64)));
        assert_eq!("   1 KB   ", format!("{:^#10}",   Size::Byte(1000u64)));
        assert_eq!(" 1.001 KB ", format!("{:^#10.3}", Size::Byte(1001u64)));
        assert_eq!("  1023 B  ", format!("{:^10}",    Size::Byte(1023u64)));
        assert_eq!("  1 KiB   ", format!("{:^10}",    Size::Byte(1024u64)));
        assert_eq!("1.001 KiB ", format!("{:^10.3}",  Size::Byte(1025u64)));
        // aligned right
        assert_eq!("     999 B", format!("{:>#10}",   Size::Byte(999u64)));
        assert_eq!("      1 KB", format!("{:>#10}",   Size::Byte(1000u64)));
        assert_eq!("  1.001 KB", format!("{:>#10.3}", Size::Byte(1001u64)));
        assert_eq!("    1023 B", format!("{:>10}",    Size::Byte(1023u64)));
        assert_eq!("     1 KiB", format!("{:>10}",    Size::Byte(1024u64)));
        assert_eq!(" 1.001 KiB", format!("{:>10.3}",  Size::Byte(1025u64)));
    }

    #[test]
    #[rustfmt::skip]
    fn parsed_value_works() {
        assert_eq!(    "999 B".parse(), Ok(Size::Byte(999u64)));
        assert_eq!(     "1 KB".parse(), Ok(Size::Kilo(1u64)));
        assert_eq!( "1.001 KB".parse(), Ok(Size::Byte(1001u64)));
        assert_eq!(   "1023 B".parse(), Ok(Size::Byte(1023u64)));
        assert_eq!(    "1 KiB".parse(), Ok(Size::Kibi(1u64)));
        assert_eq!("1.001 KiB".parse(), Ok(Size::Byte(1025u64)));
    }

    #[test]
    #[rustfmt::skip]
    fn parsed_bad_failure() {
        assert_eq!(      Size::from_str(  "999B"), Err(Error::Size));
        assert_eq!(      Size::from_str("999 QB"), Err(Error::Unit));
        assert!(matches!(Size::from_str("9.0a B"), Err(Error::Num(_))));
    }
}
