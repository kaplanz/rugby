//! Game ROM cartridge.
//!
//! Encoded within the ROM is a [header] describing the physical cartridge
//! hardware upon which the ROM resides.
//!
//! Additionally, one of several supported [memory bank controllers][mbcs] may
//! be used to expand the ROM and external RAM beyond the respective 32 KiB and
//! 8 KiB addressable bytes.
//!
//! [header]: https://gbdev.io/pandocs/The_Cartridge_Header.html
//! [mbcs]:   https://gbdev.io/pandocs/MBCs.html

use std::io;

use rugby_arch::Block;
use rugby_arch::mio::{Bus, Mmio};
use thiserror::Error;

use self::chip::Chip;
use self::head::parts::Board;

pub mod chip;
pub mod head;

pub use self::head::Header;

/// Game cartridge.
///
/// Parses a [`Header`] from the ROM, then initializes the memory bank
/// controller ([`mbc`]).
#[derive(Clone, Debug)]
pub struct Cartridge {
    /// Cartridge header.
    pub(crate) head: Header,
    /// Cartridge hardware.
    pub(crate) chip: Chip,
}

impl Cartridge {
    /// Constructs a new `Cartridge`.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge could not be constructed. This will
    /// either be due to invalid header bytes or an unsupported cartridge type.
    pub fn new(rom: &[u8]) -> Result<Self> {
        let head = Header::new(rom)?;
        Ok(Self {
            chip: Chip::new(&head, rom)?,
            head,
        })
    }

    /// Checks a if ROM has can reasonably be constructed.
    ///
    /// # Errors
    ///
    /// Returns an error if the ROM contained invalid header bytes, or if
    /// cartridge integrity seems compromised. (This is detected using
    /// checksums.)
    pub fn check(rom: &[u8]) -> Result<()> {
        let head = Header::new(rom)?;
        Chip::check(&head, rom)
    }

    /// Constructs a new `Cartridge`, checking for cartridge integrity.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge could not be constructed. This will
    /// either be due to invalid header bytes or an unsupported cartridge type.
    pub fn checked(rom: &[u8]) -> Result<Self> {
        let head = Header::checked(rom)?;
        Ok(Self {
            chip: Chip::new(&head, rom)?,
            head,
        })
    }

    /// Constructs a new `Cartridge` without checking the header.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge could not be constructed. This will
    /// either be due to missing header bytes or an unsupported cartridge type.
    pub fn unchecked(rom: &[u8]) -> Result<Self> {
        let head = Header::unchecked(rom)?;
        Ok(Self {
            chip: Chip::new(&head, rom)?,
            head,
        })
    }

    /// Reads the cartridge's title from the header.
    ///
    /// # Note
    ///
    /// If the header's title field is either unspecified or invalid, a
    /// placeholder of "Unknown" is used instead.
    #[must_use]
    pub fn title(&self) -> &str {
        self.head.about.title.as_deref().unwrap_or("Unknown")
    }

    /// Gets the cartridge's parsed header.
    #[must_use]
    pub fn header(&self) -> &Header {
        &self.head
    }

    /// Flashes data onto the cartridge's RAM.
    ///
    /// # Errors
    ///
    /// May generate an I/O error indicating that the operation could not be
    /// completed. If an error is returned then no bytes were read.
    pub fn flash(&mut self, buf: &mut impl io::Read) -> io::Result<usize> {
        self.chip.flash(buf)
    }

    /// Dumps contents of the cartridge's RAM.
    ///
    /// # Errors
    ///
    /// May generate an I/O error indicating that the operation could not be
    /// completed. If an error is returned then no bytes were written.
    pub fn dump(&self, buf: &mut impl io::Write) -> io::Result<usize> {
        self.chip.dump(buf)
    }
}

impl Block for Cartridge {
    fn reset(&mut self) {
        self.chip.reset();
    }
}

impl Mmio for Cartridge {
    fn attach(&self, bus: &mut Bus) {
        self.chip.attach(bus);
    }

    fn detach(&self, bus: &mut Bus) {
        self.chip.detach(bus);
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by constructing a [cartridge](Cartridge).
#[derive(Debug, Error)]
pub enum Error {
    /// Bad cartridge header.
    #[error("bad cartridge header")]
    Header(#[from] head::Error),
    /// Unsupported cartridge type.
    #[error("unsupported cartridge: {0}")]
    Unsupported(Board),
}
