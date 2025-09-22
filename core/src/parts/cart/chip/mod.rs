//! Memory bank controllers.
//!
//! Implementations of various kinds of cartridge hardware.

#![allow(clippy::module_name_repetitions)]

use std::fmt::Debug;
use std::io;

use log::{debug, trace};
use rugby_arch::Block;
use rugby_arch::mio::{Bus, Device, Mmio};

use super::{Board, Error, Header, Result};

mod mbc1;
mod mbc3;
mod mbc5;
mod none;

pub use self::mbc1::Mbc1;
pub use self::mbc3::Mbc3;
pub use self::mbc5::Mbc5;
pub use self::none::None;

/// Memory data.
type Data = Box<[u8]>;

/// Memory bank controller.
pub trait Mbc {
    /// Gets the contents of the cartridge's ROM.
    fn rom(&self) -> Device;

    /// Gets the contents of the cartridge's RAM.
    fn ram(&self) -> Device;
}

/// Cartridge body.
///
/// Contains the cartridge's ROM and RAM, modelling cartridge-specific hardware.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) enum Chip {
    None(None),
    Mbc1(Mbc1),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
}

impl Chip {
    /// Constructs a new `Body`.
    ///
    /// # Errors
    ///
    /// Returns an error if unsupported cartridge type is specified in the
    /// header.
    pub fn new(head: &Header, rom: &[u8]) -> Result<Self> {
        // Initialize ROM
        let rom = init::rom(head, rom);
        if !rom.is_empty() {
            trace!("cart ROM:\n{}", hexd::Printer::<u8>::new(0, &rom));
        }
        // Initialize RAM
        let ram = init::ram(head);
        // Construct body
        match &head.board {
            &Board::None { .. } => Ok(Chip::None(None::new(rom, ram))),
            &Board::Mbc1 { .. } => Ok(Chip::Mbc1(Mbc1::new(rom, ram))),
            &Board::Mbc3 { .. } => Ok(Chip::Mbc3(Mbc3::new(rom, ram))),
            &Board::Mbc5 { .. } => Ok(Chip::Mbc5(Mbc5::new(rom, ram))),
            kind => Err(Error::Unsupported(kind.clone())),
        }
    }

    /// Checks a if ROM has can reasonably be constructed.
    ///
    /// # Errors
    ///
    /// Returns an error if unsupported cartridge type is specified in the
    /// header.
    #[expect(unused_variables)]
    pub fn check(head: &Header, rom: &[u8]) -> Result<()> {
        match &head.board {
            Board::None { .. } | Board::Mbc1 { .. } | Board::Mbc3 { .. } | Board::Mbc5 { .. } => {
                Ok(())
            }
            kind => Err(Error::Unsupported(kind.clone())),
        }
    }
}

impl Block for Chip {
    fn ready(&self) -> bool {
        match self {
            Chip::None(mbc) => mbc.ready(),
            Chip::Mbc1(mbc) => mbc.ready(),
            Chip::Mbc3(mbc) => mbc.ready(),
            Chip::Mbc5(mbc) => mbc.ready(),
        }
    }

    fn cycle(&mut self) {
        match self {
            Chip::None(mbc) => mbc.cycle(),
            Chip::Mbc1(mbc) => mbc.cycle(),
            Chip::Mbc3(mbc) => mbc.cycle(),
            Chip::Mbc5(mbc) => mbc.cycle(),
        }
    }

    fn reset(&mut self) {
        match self {
            Chip::None(mbc) => mbc.reset(),
            Chip::Mbc1(mbc) => mbc.reset(),
            Chip::Mbc3(mbc) => mbc.reset(),
            Chip::Mbc5(mbc) => mbc.reset(),
        }
    }
}

impl Mbc for Chip {
    fn rom(&self) -> Device {
        match self {
            Chip::None(mbc) => mbc.rom(),
            Chip::Mbc1(mbc) => mbc.rom(),
            Chip::Mbc3(mbc) => mbc.rom(),
            Chip::Mbc5(mbc) => mbc.rom(),
        }
    }

    fn ram(&self) -> Device {
        match self {
            Chip::None(mbc) => mbc.ram(),
            Chip::Mbc1(mbc) => mbc.ram(),
            Chip::Mbc3(mbc) => mbc.ram(),
            Chip::Mbc5(mbc) => mbc.ram(),
        }
    }
}

impl Mmio for Chip {
    fn attach(&self, bus: &mut Bus) {
        bus.map(0x0000..=0x7fff, self.rom());
        bus.map(0xa000..=0xbfff, self.ram());
    }

    fn detach(&self, bus: &mut Bus) {
        assert!(bus.unmap(&self.rom()));
        assert!(bus.unmap(&self.ram()));
    }
}

impl Chip {
    /// Flashes data onto the cartridge's RAM.
    ///
    /// # Errors
    ///
    /// May generate an I/O error indicating that the operation could not be
    /// completed. If an error is returned then no bytes were read.
    pub fn flash(&mut self, buf: &mut impl io::Read) -> io::Result<usize> {
        let mut flash = |ram: &mut [u8]| {
            buf.read(ram).inspect(|&nbytes| {
                debug!("loaded {size}", size = bfmt::Size::from(nbytes));
                trace!("cart RAM:\n{}", hexd::Printer::<u8>::new(0, ram));
            })
        };
        match self {
            Chip::None(mbc) => flash(mbc.ram.borrow_mut().inner_mut()),
            Chip::Mbc1(mbc) => flash(mbc.ram.borrow_mut().mem.as_mut()),
            Chip::Mbc3(mbc) => flash(mbc.ram.borrow_mut().mem.as_mut()),
            Chip::Mbc5(mbc) => flash(mbc.ram.borrow_mut().mem.as_mut()),
        }
    }

    /// Dumps contents of the cartridge's RAM.
    ///
    /// # Errors
    ///
    /// May generate an I/O error indicating that the operation could not be
    /// completed. If an error is returned then no bytes were written.
    pub fn dump(&self, buf: &mut impl io::Write) -> io::Result<usize> {
        let mut dump = |ram: &[u8]| {
            buf.write(ram).inspect(|&nbytes| {
                debug!("dumped {size}", size = bfmt::Size::from(nbytes));
                trace!("cart RAM:\n{}", hexd::Printer::<u8>::new(0, ram));
            })
        };
        match self {
            Chip::None(mbc) => dump(mbc.ram.borrow_mut().inner_mut()),
            Chip::Mbc1(mbc) => dump(mbc.ram.borrow_mut().mem.as_mut()),
            Chip::Mbc3(mbc) => dump(mbc.ram.borrow_mut().mem.as_mut()),
            Chip::Mbc5(mbc) => dump(mbc.ram.borrow_mut().mem.as_mut()),
        }
    }
}

mod init {
    use std::cmp::Ordering;
    use std::iter;

    use log::{debug, warn};

    use super::{Data, Header};

    /// Constructs a new ROM.
    pub fn rom(head: &Header, rom: &[u8]) -> Data {
        let read = rom.len();
        match read.cmp(&head.memory.romsz) {
            Ordering::Less => {
                warn!(
                    "loaded {init}; remaining {diff} uninitialized",
                    init = bfmt::Size::from(read),
                    diff = bfmt::Size::from(head.memory.romsz - read),
                );
            }
            Ordering::Equal => debug!("loaded {read}", read = bfmt::Size::from(read)),
            Ordering::Greater => {
                warn!(
                    "loaded {init}; remaining {diff} truncated",
                    init = bfmt::Size::from(head.memory.romsz),
                    diff = bfmt::Size::from(read - head.memory.romsz),
                );
            }
        }
        rom.iter()
            .copied()
            // pad missing values with open bus value
            .chain(iter::repeat(0xff))
            // truncate based on recorded cartridge size
            .take(head.memory.romsz)
            // collect as a heap-allocated slice
            .collect::<Box<_>>()
    }

    /// Constructs a new RAM.
    pub fn ram(head: &Header) -> Data {
        if head.board.has_ram() && head.memory.ramsz == 0 {
            warn!("cartridge supports RAM, but specified size is zero");
        }
        if !head.board.has_ram() && head.memory.ramsz > 0 {
            warn!(
                "cartridge does not support RAM, but specified size is non-zero (found: {})",
                head.memory.ramsz
            );
        }
        vec![0; head.memory.ramsz].into_boxed_slice()
    }
}
