use std::error::Error;
use std::fmt::Display;
use std::str::Utf8Error;

use log::error;

#[derive(Debug)]
pub struct Header {
    pub title: String,
    pub cgb: u8,
    pub sgb: bool,
    pub cart: u8,
    pub romsz: u8,
    pub ramsz: u8,
    pub jpn: bool,
    pub version: u8,
    pub hchk: u8,
    pub gchk: u16,
}

impl TryFrom<&[u8]> for Header {
    type Error = ErrorKind;

    fn try_from(rom: &[u8]) -> Result<Self, Self::Error> {
        // Extract the header bytes
        let bytes: &[u8; 0x50] = rom
            .get(0x100..=0x14f)
            .ok_or(ErrorKind::MissingHeader)?
            .try_into()
            .unwrap();

        // Parse CGB flag
        let cgb = bytes[0x43];
        // Parse title
        let tlen = if cgb & 0x80 != 0 { 15 } else { 16 };
        let title = std::str::from_utf8(&bytes[0x34..0x34 + tlen])
            .map_err(ErrorKind::InvalidTitle)?
            .to_string();
        // Parse SGB flag
        let sgb = match bytes[0x46] {
            0x00 => Ok(false),
            0x03 => Ok(true),
            byte => Err(ErrorKind::InvalidSgb(byte)),
        }?;
        // Parse cartridge type
        let cart = bytes[0x47];
        // Parse ROM size
        let romsz = bytes[0x48];
        // Parse RAM size
        let ramsz = bytes[0x49];
        // Parse RAM size
        let jpn = match bytes[0x4a] {
            0x00 => Ok(true),
            0x01 => Ok(false),
            byte => Err(ErrorKind::InvalidDest(byte)),
        }?;
        // Parse mark ROM version number
        let version = bytes[0x4c];
        // Parse header checksum
        let hchk = bytes[0x4d];
        // Parse global checksum
        let gchk = u16::from_be_bytes(bytes[0x4e..=0x4f].try_into().unwrap());

        // Verify header checksum
        match bytes[0x34..=0x4c]
            .iter()
            .cloned()
            .fold(0u8, |accum, item| accum.wrapping_sub(item).wrapping_sub(1))
        {
            chk if chk == hchk => Ok(()),
            chk => Err(ErrorKind::HeaderChecksum(chk)),
        }?;
        // Verify global checksum
        match rom
            .iter()
            .cloned()
            .fold(0u16, |accum, item| accum.wrapping_add(item as u16))
            .wrapping_sub(bytes[0x4e] as u16)
            .wrapping_sub(bytes[0x4f] as u16)
        {
            chk if chk == gchk => (),
            chk => error!("{}", ErrorKind::GlobalChecksum(chk)),
        };

        Ok(Self {
            title,
            cgb,
            sgb,
            cart,
            romsz,
            ramsz,
            jpn,
            version,
            hchk,
            gchk,
        })
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    MissingHeader,
    InvalidTitle(Utf8Error),
    InvalidSgb(u8),
    InvalidDest(u8),
    HeaderChecksum(u8),
    GlobalChecksum(u16),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHeader => write!(f, "Missing Header"),
            Self::InvalidTitle(err) => write!(f, "Invalid Title: {err}"),
            Self::InvalidSgb(flag) => write!(f, "Invalid SGB Flag: {flag}"),
            Self::InvalidDest(code) => write!(f, "Invalid Destination Code: {code}"),
            Self::HeaderChecksum(chk) => write!(f, "Bad Header Checksum: {chk}"),
            Self::GlobalChecksum(chk) => write!(f, "Bad Global Checksum: {chk}"),
        }
    }
}

impl Error for ErrorKind {}
