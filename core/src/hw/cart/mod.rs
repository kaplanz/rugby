//! Game ROM cartridge.
//!
//! Encoded within the ROM is a [hardware specification][cartridge header] of
//! the physical cartridge on which the ROM is connected to the console.
//!
//! Additionally, one of several [memory bank controllers][mbcs] may be used to
//! expand the ROM and external RAM beyond the respective 32 KiB and 8 KiB
//! addressable bytes.
//!
//! [cartridge header]: https://gbdev.io/pandocs/The_Cartridge_Header.html
//! [mbcs]:             https://gbdev.io/pandocs/MBCs.html

use std::cmp::Ordering;
use std::iter;

use log::{debug, info, trace, warn};
use remus::bus::Bus;
use remus::dev::Null;
use remus::mem::{Ram, Rom};
use remus::{Block, Board, Device, Dynamic};
use thiserror::Error;

use self::header::Kind;
use self::mbc::{Mbc, Mbc1, NoMbc};

mod header;

pub mod mbc;

pub use self::header::{Error as HeaderError, Header};

/// Cartridge model.
///
/// Parses a [`Header`] from the ROM, then initializes the memory bank
/// controller ([`mbc`]).
#[derive(Debug)]
pub struct Cartridge {
    header: Header,
    mbc: Box<dyn Mbc>,
}

impl Cartridge {
    /// Constructs a new `Cartridge`.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge header cannot be parsed.
    pub fn new(rom: &[u8]) -> Result<Self, Error> {
        // Parse cartridge header
        let header = Header::try_from(rom)?;
        debug!("Header:\n{header}");

        // Construct memory bank controller
        let mbc = Self::mbc(&header, rom)?;

        Ok(Self { header, mbc })
    }

    /// Constructs a new `Cartridge` explicitly checking the entire header.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge header contained an error.
    pub fn checked(rom: &[u8]) -> Result<Self, Error> {
        // Check then parse cartridge header
        let header = Header::check(rom).and_then(|_| Header::try_from(rom))?;
        debug!("Header:\n{header}");

        // Construct memory bank controller
        let mbc = Self::mbc(&header, rom)?;

        Ok(Self { header, mbc })
    }

    /// Constructs a new `Cartridge` without checking the header.
    ///
    /// # Panics
    ///
    /// Panics if the memory bank controller could not be constructed.
    pub fn unchecked(rom: &[u8]) -> Self {
        // Parse cartridge header
        let header = Header::try_from(rom).ok().unwrap_or_else(Header::blank);
        debug!("Header:\n{header}");

        // Construct memory bank controller
        let mbc = Self::mbc(&header, rom).ok().unwrap();

        Self { header, mbc }
    }

    /// Constructs a blank `Cartridge`.
    #[must_use]
    pub fn blank() -> Self {
        // Construct a blank header
        let header = Header::blank();

        // Use null devices for the ROM, RAM
        let rom = Null::<0x8000>::with(0xff).to_dynamic();
        let eram = Null::<0x0>::new().to_dynamic();

        // Construct a membory bank controller
        let mbc = Box::new(NoMbc::with(rom, eram));

        Self { header, mbc }
    }

    /// Gets a reference to the cartridge's header.
    #[must_use]
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Gets a shared reference to the cartridge's ROM.
    #[must_use]
    pub fn rom(&self) -> Dynamic {
        self.mbc.rom()
    }

    /// Gets a shared reference to the cartridge's RAM.
    #[must_use]
    pub fn ram(&self) -> Dynamic {
        self.mbc.ram()
    }

    /// Constructs a memory bank controller from a parsed ROM and header.
    fn mbc(header: &Header, rom: &[u8]) -> Result<Box<dyn Mbc>, Error> {
        // Construct null device (for reuse where needed)
        let null = Null::<0>::new().to_dynamic();

        // Prepare external ROM
        let rom = {
            // Calculate buffer stats
            let read = rom.len();
            match read.cmp(&header.romsz) {
                Ordering::Less => {
                    warn!(
                        "Initialized {init} bytes; remaining {diff} bytes uninitialized",
                        init = read,
                        diff = header.romsz - read
                    );
                }
                Ordering::Equal => info!("Initialized {read} bytes"),
                Ordering::Greater => {
                    warn!(
                        "Initialized {init} bytes; remaining {diff} bytes truncated",
                        init = header.romsz,
                        diff = read - header.romsz
                    );
                }
            }
            rom.iter()
                .copied()
                .chain(iter::repeat(0xffu8))
                .take(header.romsz)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        };
        trace!("ROM:\n{rom}", rom = phex::Printer::<u8>::new(0, &rom));

        // Construct external ROM
        let rom = {
            match header.romsz {
                0x0000_8000 => Rom::<0x0000_8000>::from(
                    &*Box::<[_; 0x0000_8000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0001_0000 => Rom::<0x0001_0000>::from(
                    &*Box::<[_; 0x0001_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0002_0000 => Rom::<0x0002_0000>::from(
                    &*Box::<[_; 0x0002_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0004_0000 => Rom::<0x0004_0000>::from(
                    &*Box::<[_; 0x0004_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0008_0000 => Rom::<0x0008_0000>::from(
                    &*Box::<[_; 0x0008_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0010_0000 => Rom::<0x0010_0000>::from(
                    &*Box::<[_; 0x0010_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0020_0000 => Rom::<0x0020_0000>::from(
                    &*Box::<[_; 0x0020_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0040_0000 => Rom::<0x0040_0000>::from(
                    &*Box::<[_; 0x0040_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                0x0080_0000 => Rom::<0x0080_0000>::from(
                    &*Box::<[_; 0x0080_0000]>::try_from(rom).map_err(Error::Missing)?,
                )
                .to_dynamic(),
                _ => unreachable!(),
            }
        };

        // Construct external RAM
        let eram = match header.ramsz {
            0x0 => null.clone().to_dynamic(),
            0x2000 => Ram::<0x2000>::new().to_dynamic(),
            0x8000 => Ram::<0x8000>::new().to_dynamic(),
            0x20000 => Ram::<0x20000>::new().to_dynamic(),
            0x10000 => Ram::<0x10000>::new().to_dynamic(),
            _ => unreachable!(),
        };

        // Construct a memory bank controller
        let mbc: Box<dyn Mbc> = match &header.cart {
            &Kind::NoMbc { ram, .. } => {
                let eram = [null, eram][ram as usize].clone();
                Box::new(NoMbc::with(rom, eram))
            }
            &Kind::Mbc1 { ram, battery } => {
                let eram = [null, eram][ram as usize].clone();
                Box::new(Mbc1::with(rom, eram, battery))
            }
            cart => unimplemented!("{cart:?}"),
        };

        Ok(mbc)
    }
}

impl Block for Cartridge {
    fn reset(&mut self) {
        // Reset MBC
        self.mbc.reset();
    }
}

impl Board for Cartridge {
    #[rustfmt::skip]
    fn connect(&self, bus: &mut Bus) {
        // Extract devices
        let rom = self.rom();
        let ram = self.ram();

        // Map devices on bus // ┌──────┬────────┬────────────┬─────┐
                              // │ Addr │  Size  │    Name    │ Dev │
                              // ├──────┼────────┼────────────┼─────┤
        bus.map(0x0000, rom); // │ 0000 │ 32 KiB │ Cartridge  │ ROM │
        bus.map(0xa000, ram); // │ a000 │  8 KiB │ External   │ RAM │
                              // └──────┴────────┴────────────┴─────┘
    }

    fn disconnect(&self, bus: &mut Bus) {
        // Extract devices
        let rom = self.rom();
        let ram = self.ram();

        // Unmap devices on bus
        assert!(bus.unmap(0x0000, &rom).is_some());
        assert!(bus.unmap(0xa000, &ram).is_some());
    }
}

/// A type specifying categories of [`Cartridge`] error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("could not parse header")]
    Header(#[from] header::Error),
    #[error("cartridge missing bytes")]
    Missing(Box<[u8]>),
}
