//! Game cartridge.
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

use rugby_arch::mem::{self, Memory};
use rugby_arch::{Block, Shared};

use self::chip::Chip;
use self::head::parts::Board;

pub mod chip;
pub mod head;

pub use self::head::Header;

/// Cartridge slot.
///
/// Dispatches accesses to the inserted cartridge, leaving an empty slot
/// unmapped.
#[derive(Clone, Debug, Default)]
pub struct Slot(Shared<Option<Cartridge>>);

impl Slot {
    /// Constructs a new, empty `Slot`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the inserted cartridge, if any.
    #[must_use]
    pub fn get(&self) -> Option<Cartridge> {
        self.0.borrow().clone()
    }

    /// Inserts a cartridge into the slot.
    pub fn insert(&mut self, cart: Cartridge) {
        *self.0.borrow_mut() = Some(cart);
    }

    /// Ejects the inserted cartridge, if any.
    pub fn eject(&mut self) -> Option<Cartridge> {
        self.0.borrow_mut().take()
    }
}

impl Block for Slot {
    fn reset(&mut self) {
        if let Some(cart) = self.0.borrow_mut().as_mut() {
            cart.reset();
        }
    }
}

impl Memory for Slot {
    fn read(&self, addr: u16) -> mem::Result<u8> {
        self.0
            .borrow()
            .as_ref()
            .map_or(Err(mem::Error::Range), |cart| cart.read(addr))
    }

    fn write(&mut self, addr: u16, data: u8) -> mem::Result<()> {
        self.0
            .borrow_mut()
            .as_mut()
            .map_or(Err(mem::Error::Range), |cart| cart.write(addr, data))
    }
}

/// Game cartridge.
///
/// Parses a [`Header`] from the ROM, then initializes the appropriate
/// memory bank controller.
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

impl Memory for Cartridge {
    fn read(&self, addr: u16) -> mem::Result<u8> {
        self.chip.read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) -> mem::Result<()> {
        self.chip.write(addr, data)
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by constructing a [cartridge](Cartridge).
#[derive(Debug)]
#[derive(thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Bad cartridge header.
    #[error("bad cartridge header")]
    Header(#[from] head::Error),
    /// Unsupported cartridge type.
    #[error("unsupported cartridge: {0}")]
    Unsupported(Board),
}
