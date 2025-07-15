//! Game ROM cartridge header.
//!
//! Encoded in the ROM at the address range `[$0100..$0150]` is the header, which
//! encodes both physical attributes describing the hardware of the cartridge,
//! flags describing console support, and characteristics of the software.
//!
//! See more details [here][header].
//!
//! [header]: https://gbdev.io/pandocs/The_Cartridge_Header.html

use std::array::TryFromSliceError;
use std::fmt::Display;
use std::str::Utf8Error;

use log::error;
use parts::{About, Board, Check, Memory, Region, Compat};
use thiserror::Error;

/// Nintendo logo.
///
/// ```text
/// ██▄  ██ ██        ▄▄                   ██
/// ██▀▄ ██ ▄▄ ▄▄ ▄▄ ▀██▀ ▄▄▄▄  ▄▄ ▄▄   ▄▄▄██  ▄▄▄▄
/// ██ ▀▄██ ██ ██▀ ██ ██ ██▄▄██ ██▀ ██ ██  ██ ██  ██
/// ██  ▀██ ██ ██  ██ ██ ▀█▄▄▄▄ ██  ██ ▀█▄▄██ ▀█▄▄█▀
/// ```
///
/// Compressed copy of Nintendo's logo rendered by the boot ROM. The console
/// will refuse to pass control to cartridges that do not contain an exact copy
/// of this data.
pub const LOGO: [u8; 0x30] = [
    0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d,
    0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99,
    0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
];

/// Calculates the header checksum.
#[must_use]
pub fn hchk(rom: &[u8]) -> u8 {
    rom[0x134..=0x14c]
        .iter()
        .copied()
        .fold(0, |accum, item| accum.wrapping_sub(item).wrapping_sub(1))
}

/// Calculates the global checksum.
#[must_use]
pub fn gchk(rom: &[u8]) -> u16 {
    rom.iter()
        .copied()
        .fold(0u16, |accum, item| accum.wrapping_add(u16::from(item)))
        .wrapping_sub(u16::from(rom[0x14e]))
        .wrapping_sub(u16::from(rom[0x14f]))
}

/// Cartridge header.
///
/// Information about the ROM and the cartridge containing it. Stored in the
/// address range `[$0100, $0150)`.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Header {
    /// Game information.
    pub about: About,
    /// Data integrity.
    pub check: Check,
    /// Mapper hardware.
    pub board: Board,
    /// Memory hardware.
    pub memory: Memory,
    /// Model compatibility.
    pub compat: Compat,
}

impl Header {
    /// Constructs a new `Header`.
    ///
    /// # Errors
    ///
    /// Returns an error if the ROM contained invalid header bytes.
    pub fn new(rom: &[u8]) -> Result<Self> {
        // Extract header bytes
        let head: &[u8; 0x50] = rom.get(0x100..0x150).ok_or(Error::Missing)?.try_into()?;

        // Construct header
        Ok(Self {
            about: About {
                title: parse::title(head)?,
                region: parse::region(head)?,
                version: parse::version(head),
            },
            check: Check {
                logo: parse::logo(head),
                hchk: parse::hchk(head),
                gchk: parse::gchk(head),
            },
            board: parse::board(head)?,
            memory: Memory {
                romsz: parse::romsz(head)?,
                ramsz: parse::ramsz(head)?,
            },
            compat: Compat {
                dmg: parse::dmg(head),
                cgb: parse::cgb(head),
                sgb: parse::sgb(head),
            },
        })
    }

    /// Constructs a new `Header`, checking cartridge integrity.
    ///
    /// # Errors
    ///
    /// Returns an error if the ROM contained invalid header bytes, or if
    /// cartridge integrity seems compromised. (This is detected using
    /// checksums.)
    pub fn checked(rom: &[u8]) -> Result<Self> {
        let this = Self::new(rom)?;

        // Validate header checksum
        let chk = self::hchk(rom);
        if chk != this.check.hchk {
            return Err(Error::HeaderChk {
                found: chk,
                expected: this.check.hchk,
            });
        }
        // Validate global checksum
        let chk = self::gchk(rom);
        if chk != this.check.gchk {
            return Err(Error::GlobalChk {
                found: chk,
                expected: this.check.gchk,
            });
        }

        Ok(this)
    }

    /// Constructs a new `Header`, ignoring invalid header bytes.
    ///
    /// # Errors
    ///
    /// Header construction will fail if the ROM is missing header bytes.
    pub fn unchecked(rom: &[u8]) -> Result<Self> {
        // Extract header bytes
        let head: &[u8; 0x50] = rom.get(0x100..0x150).ok_or(Error::Missing)?.try_into()?;

        // Construct header
        let ramsz = parse::ramsz(head);
        let this = Self {
            about: About {
                title: parse::title(head).unwrap_or_else(|err| {
                    let default = Option::default();
                    error!("{err} (default: {default:?})");
                    default
                }),
                region: parse::region(head).unwrap_or_else(|err| {
                    let default = Region::Japan;
                    error!("{err} (default: {default:?})");
                    default
                }),
                version: parse::version(head),
            },
            check: Check {
                logo: parse::logo(head),
                hchk: parse::hchk(head),
                gchk: parse::gchk(head),
            },
            board: parse::board(head).unwrap_or_else(|err| {
                let default = Board::None {
                    exram: matches!(ramsz, Ok(ramsz) if ramsz > 0),
                    power: false,
                };
                error!("{err} (default: {default:?})");
                default
            }),
            memory: Memory {
                romsz: parse::romsz(head).unwrap_or_else(|err| {
                    let default = 0x8000;
                    error!("{err} (default: {default:?})");
                    default
                }),
                ramsz: ramsz.unwrap_or_else(|err| {
                    let default = usize::default();
                    error!("{err} (default: {default:?})");
                    default
                }),
            },
            compat: Compat {
                dmg: parse::dmg(head),
                cgb: parse::cgb(head),
                sgb: parse::sgb(head),
            },
        };

        Ok(this)
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌──────────────────┐")?;
        writeln!(
            f,
            "│ {:^16} │",
            self.about.title.as_deref().unwrap_or("Unknown")
        )?;
        writeln!(f, "├──────────────────┤")?;
        writeln!(f, "│ DMG: {:>11} │", self.compat.dmg)?;
        writeln!(f, "│ CGB: {:>11} │", self.compat.cgb)?;
        writeln!(f, "│ SGB: {:>11} │", self.compat.sgb)?;
        writeln!(f, "├──────────────────┤")?;
        writeln!(f, "│ MBC: {:>11} │", self.board)?;
        writeln!(f, "├──────────────────┤")?;
        writeln!(f, "│ ROM: {:>11.0} │", bfmt::Size::from(self.memory.romsz))?;
        writeln!(f, "│ RAM: {:>11.0} │", bfmt::Size::from(self.memory.ramsz))?;
        writeln!(f, "├──────────────────┤")?;
        writeln!(f, "│ Region: {:>8} │", self.about.region)?;
        writeln!(f, "│ Version: {:>7} │", self.about.revision())?;
        writeln!(f, "├──────────────────┤")?;
        writeln!(f, "│ Header:       {:0>2x} │", self.check.hchk)?;
        writeln!(f, "│ Global:     {:0>4x} │", self.check.gchk)?;
        write!(f, "└──────────────────┘")
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = Error;

    fn try_from(rom: &[u8]) -> Result<Self, Self::Error> {
        Self::new(rom)
    }
}

/// Header fields.
pub mod parts {
    use std::fmt::Display;

    use super::Error;

    /// Game information.
    #[derive(Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct About {
        /// `[$0134..=$0143]`: Title.
        ///
        /// The title of the game in uppercase ASCII.
        ///
        /// See more details [here][title].
        ///
        /// [title]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0134-0143--title
        pub title: Option<String>,
        /// `[$014A]`: Destination code.
        ///
        /// Whether this version of the game is intended to be sold in Japan or
        /// elsewhere.
        ///
        /// See more details [here][region].
        ///
        /// [region]: https://gbdev.io/pandocs/The_Cartridge_Header.html#014a--destination-code
        pub region: Region,
        /// `[$014C]`: Mask ROM version number.
        ///
        /// Version number of the game.
        ///
        /// See more details [here][version].
        ///
        /// [version]: https://gbdev.io/pandocs/The_Cartridge_Header.html#014c--mask-rom-version-number
        pub version: u8,
    }

    impl About {
        /// Formats a version number as a revision string
        #[must_use]
        pub fn revision(&self) -> String {
            format!(
                "v{}.{}",
                ((self.version & 0xf0) >> 4) + 1,
                self.version & 0x0f
            )
        }
    }

    /// Data integrity.
    #[derive(Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct Check {
        /// `[$0104..=$0133]`: Nintendo logo.
        ///
        /// Check for if the cartridge was officially licensed by Nintendo.
        /// Cartridges must include the [expected bytes](super::LOGO) to render
        /// Nintendo's logo in the boot ROM.
        ///
        /// See more details [here][logo].
        ///
        /// [logo]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0104-0133--nintendo-logo
        pub logo: bool,
        /// `[$014D]`: Header checksum.
        ///
        /// An 8-bit checksum computed from the cartridge header bytes
        /// `[$0134..=$014C]`.
        ///
        /// See more details [here][hchk].
        ///
        /// [hchk]: https://gbdev.io/pandocs/The_Cartridge_Header.html#014d--header-checksum
        pub hchk: u8,
        /// `[$014E..=$014F]`: Global checksum.
        ///
        /// A 16-bit checksum computed from the entire cartridge ROM (excluding
        /// these two bytes). This checksum is not usually verified.
        ///
        /// See more details [here][gchk].
        ///
        /// [gchk]: https://gbdev.io/pandocs/The_Cartridge_Header.html#014e-014f--global-checksum
        pub gchk: u16,
    }

    /// Memory hardware.
    #[derive(Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct Memory {
        /// `[$0148]`: ROM size.
        ///
        /// How much ROM is present on the cartridge.
        ///
        /// See more details [here][romsz].
        ///
        /// [romsz]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0148--rom-size
        pub romsz: usize,
        /// `[$0149]`: RAM size.
        ///
        /// How much RAM is present on the cartridge, if any.
        ///
        /// See more details [here][ramsz].
        ///
        /// [ramsz]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0149--ram-size
        pub ramsz: usize,
    }

    /// Model compatibility.
    #[derive(Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct Compat {
        /// `[$0143]`: DMG flag.
        ///
        /// Whether this cartridge is compatible with the DMG model.
        ///
        /// See more details [here][dmg].
        ///
        /// [dmg]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0143--cgb-flag
        pub dmg: bool,
        /// `[$0143]`: CGB flag.
        ///
        /// Whether this cartridge is compatible with the CGB model.
        ///
        /// See more details [here][cgb].
        ///
        /// [cgb]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0143--cgb-flag
        pub cgb: bool,
        /// `[$0146]`: SGB flag.
        ///
        /// Whether this cartridge is compatible with the SGB model.
        ///
        /// See more details [here][sgb].
        ///
        /// [sgb]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0146--sgb-flag
        pub sgb: bool,
    }

    /// Mapper hardware.
    ///
    /// `[$0147]`: Cartridge type.
    ///
    /// What kind of hardware is present on the cartridge.
    ///
    /// See more details [here][type].
    ///
    /// [type]: https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Deserialize, serde::Serialize),
        serde(tag = "chip", content = "spec")
    )]
    #[non_exhaustive]
    pub enum Board {
        #[cfg_attr(feature = "serde", serde(rename = "None"))]
        None { exram: bool, power: bool },
        #[cfg_attr(feature = "serde", serde(rename = "MBC1"))]
        Mbc1 { exram: bool, power: bool },
        #[cfg_attr(feature = "serde", serde(rename = "MBC2"))]
        Mbc2 { power: bool },
        #[cfg_attr(feature = "serde", serde(rename = "MBC3"))]
        Mbc3 {
            exram: bool,
            power: bool,
            clock: bool,
        },
        #[cfg_attr(feature = "serde", serde(rename = "MBC5"))]
        Mbc5 {
            exram: bool,
            power: bool,
            motor: bool,
        },
        #[cfg_attr(feature = "serde", serde(rename = "MBC6"))]
        Mbc6,
        #[cfg_attr(feature = "serde", serde(rename = "MBC7"))]
        Mbc7,
        #[cfg_attr(feature = "serde", serde(rename = "MMM01"))]
        Mmm01 { exram: bool, power: bool },
        #[cfg_attr(feature = "serde", serde(rename = "M161"))]
        M161,
        #[cfg_attr(feature = "serde", serde(rename = "HuC1"))]
        HuC1,
        #[cfg_attr(feature = "serde", serde(rename = "HuC3"))]
        HuC3,
        #[cfg_attr(feature = "serde", serde(rename = "Camera"))]
        Camera,
    }

    impl Board {
        /// Check if the cartridge has a power.
        #[must_use]
        pub fn has_battery(&self) -> bool {
            match self {
                Board::None { power, .. }
                | Board::Mbc1 { power, .. }
                | Board::Mbc2 { power, .. }
                | Board::Mbc3 { power, .. }
                | Board::Mbc5 { power, .. }
                | Board::Mmm01 { power, .. } => *power,
                _ => false,
            }
        }

        /// Check if the cartridge has any RAM.
        #[must_use]
        pub fn has_ram(&self) -> bool {
            match self {
                Board::None { exram, .. }
                | Board::Mbc1 { exram, .. }
                | Board::Mbc3 { exram, .. }
                | Board::Mbc5 { exram, .. }
                | Board::Mmm01 { exram, .. } => *exram,
                _ => false,
            }
        }
    }

    impl Display for Board {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::None   { .. } => "None",
                Self::Mbc1   { .. } => "MBC1",
                Self::Mbc2   { .. } => "MBC2",
                Self::Mbc3   { .. } => "MBC3",
                Self::Mbc5   { .. } => "MBC5",
                Self::Mbc6   { .. } => "MBC6",
                Self::Mbc7   { .. } => "MBC7",
                Self::Mmm01  { .. } => "MMM01",
                Self::M161   { .. } => "M161",
                Self::HuC1   { .. } => "HuC1",
                Self::HuC3   { .. } => "HuC3",
                Self::Camera { .. } => "Camera",
            }
            .fmt(f)
        }
    }

    impl TryFrom<u8> for Board {
        type Error = Error;

        #[expect(clippy::too_many_lines)]
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0x00 => Ok(Board::None {
                    exram: false,
                    power: false,
                }),
                0x01 => Ok(Board::Mbc1 {
                    exram: false,
                    power: false,
                }),
                0x02 => Ok(Board::Mbc1 {
                    exram: true,
                    power: false,
                }),
                0x03 => Ok(Board::Mbc1 {
                    exram: true,
                    power: true,
                }),
                0x05 => Ok(Board::Mbc2 { power: false }),
                0x06 => Ok(Board::Mbc2 { power: true }),
                0x08 => Ok(Board::None {
                    exram: true,
                    power: false,
                }),
                0x09 => Ok(Board::None {
                    exram: true,
                    power: true,
                }),
                0x0b => Ok(Board::Mmm01 {
                    exram: false,
                    power: false,
                }),
                0x0c => Ok(Board::Mmm01 {
                    exram: true,
                    power: false,
                }),
                0x0d => Ok(Board::Mmm01 {
                    exram: true,
                    power: true,
                }),
                0x0f => Ok(Board::Mbc3 {
                    exram: false,
                    power: true,
                    clock: true,
                }),
                0x10 => Ok(Board::Mbc3 {
                    exram: true,
                    power: true,
                    clock: true,
                }),
                0x11 => Ok(Board::Mbc3 {
                    exram: false,
                    power: false,
                    clock: false,
                }),
                0x12 => Ok(Board::Mbc3 {
                    exram: true,
                    power: false,
                    clock: false,
                }),
                0x13 => Ok(Board::Mbc3 {
                    exram: true,
                    power: true,
                    clock: false,
                }),
                0x19 => Ok(Board::Mbc5 {
                    exram: false,
                    power: false,
                    motor: false,
                }),
                0x1a => Ok(Board::Mbc5 {
                    exram: true,
                    power: false,
                    motor: false,
                }),
                0x1b => Ok(Board::Mbc5 {
                    exram: true,
                    power: true,
                    motor: false,
                }),
                0x1c => Ok(Board::Mbc5 {
                    exram: false,
                    power: false,
                    motor: true,
                }),
                0x1d => Ok(Board::Mbc5 {
                    exram: true,
                    power: false,
                    motor: true,
                }),
                0x1e => Ok(Board::Mbc5 {
                    exram: true,
                    power: true,
                    motor: true,
                }),
                0x20 => Ok(Board::Mbc6),
                0x22 => Ok(Board::Mbc7),
                0xfc => Ok(Board::Camera),
                0xfe => Ok(Board::HuC3),
                0xff => Ok(Board::HuC1),
                value => Err(Error::Kind(value)),
            }
        }
    }

    /// Destination code.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase")
    )]
    pub enum Region {
        World,
        Japan,
    }

    impl Display for Region {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            format!("{self:?}").fmt(f)
        }
    }
}

/// Field parsing.
pub mod parse {
    #![expect(clippy::missing_errors_doc)]

    use log::warn;

    use super::parts::{Board, Region};
    use super::{Error, LOGO, Result};

    /// Parse the `logo` field from the header.
    #[must_use]
    pub fn logo(head: &[u8; 0x50]) -> bool {
        head[0x04..=0x33] == LOGO
    }

    /// Parse the `title` field from the header.
    pub fn title(head: &[u8; 0x50]) -> Result<Option<String>> {
        let tlen = if head[0x43] & 0x80 == 0 { 16 } else { 15 };
        std::str::from_utf8(&head[0x34..0x34 + tlen])
            .map_err(Error::Title)
            .map(|title| {
                title
                    .split('\0')
                    .next()
                    .filter(|s| !s.is_empty())
                    .map(String::from)
            })
    }

    /// Parse the `dmg` field from the header.
    #[must_use]
    pub fn dmg(head: &[u8; 0x50]) -> bool {
        (head[0x43] & 0xc0) != 0xc0
    }

    /// Parse the `cgb` field from the header.
    #[must_use]
    pub fn cgb(head: &[u8; 0x50]) -> bool {
        head[0x43] & 0x80 != 0x00
    }

    /// Parse the `sgb` field from the header.
    #[must_use]
    pub fn sgb(head: &[u8; 0x50]) -> bool {
        match head[0x46] {
            0x00 => false,
            0x03 => true,
            byte => {
                warn!("non-standard SGB flag: {byte:#04x}");
                false
            }
        }
    }

    /// Parse the `board` field from the header.
    pub fn board(head: &[u8; 0x50]) -> Result<Board> {
        head[0x47].try_into()
    }

    /// Parse the `romsz` field from the header.
    pub fn romsz(head: &[u8; 0x50]) -> Result<usize> {
        match head[0x48] {
            byte @ 0x00..=0x08 => Ok(0x8000 << byte),
            byte => Err(Error::Rom(byte)),
        }
    }

    /// Parse the `ramsz` field from the header.
    pub fn ramsz(head: &[u8; 0x50]) -> Result<usize> {
        match head[0x49] {
            0x00 => Ok(0x00000),
            0x01 => Ok(0x00800),
            0x02 => Ok(0x02000),
            0x03 => Ok(0x08000),
            0x04 => Ok(0x20000),
            0x05 => Ok(0x10000),
            byte => Err(Error::Ram(byte)),
        }
    }

    /// Parse the `region` field from the header.
    pub fn region(head: &[u8; 0x50]) -> Result<Region> {
        match head[0x4a] {
            0x00 => Ok(Region::Japan),
            0x01 => Ok(Region::World),
            byte => Err(Error::Region(byte)),
        }
    }

    /// Parse the `version` field from the header.
    #[must_use]
    pub fn version(head: &[u8; 0x50]) -> u8 {
        head[0x4c]
    }

    /// Parse the `hchk` field from the header.
    #[must_use]
    pub fn hchk(head: &[u8; 0x50]) -> u8 {
        head[0x4d]
    }

    /// Parse the `gchk` field from the header.
    #[must_use]
    pub fn gchk(head: &[u8; 0x50]) -> u16 {
        u16::from_be_bytes([head[0x4e], head[0x4f]])
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by parsing a cartridge [header](Header).
#[derive(Debug, Error)]
pub enum Error {
    /// Missing header data.
    #[error("missing header data")]
    Missing,
    /// Slice conversion failed.
    #[error(transparent)]
    Slice(#[from] TryFromSliceError),
    /// Invalid bytes in title.
    #[error("invalid bytes in title")]
    Title(#[from] Utf8Error),
    /// Unknown hardware.
    #[error("unknown hardware: {0:#04x}")]
    Kind(u8),
    /// Invalid ROM size.
    #[error("invalid ROM size: {0:#04x}")]
    Rom(u8),
    /// Invalid RAM size.
    #[error("invalid RAM size: {0:#04x}")]
    Ram(u8),
    /// Destination code.
    #[error("destination code: {0:#04x}")]
    Region(u8),
    /// Bad header checksum.
    #[error("bad header checksum (found {found:#04x}, expected {expected:#04x})")]
    HeaderChk { found: u8, expected: u8 },
    /// Bad global checksum.
    #[error("bad global checksum (found {found:#06x}, expected {expected:#06x})")]
    GlobalChk { found: u16, expected: u16 },
}
