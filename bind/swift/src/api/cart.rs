//! Cartridge API.

use std::io;
use std::sync::Arc;

use parking_lot::Mutex;
use rugby::core::dmg;

use super::GameBoy;
use crate::Result;

#[uniffi::export]
impl GameBoy {
    /// Inserts a game cartridge.
    ///
    /// If a cartridge is already inserted, it will first be
    /// [ejected](Self::eject).
    #[uniffi::method]
    pub fn insert(&self, cart: Arc<Cartridge>) {
        // Obtain internal cartridge model
        let cart = Arc::try_unwrap(cart).map_or_else(
            // consume cart if unique
            |cart| cart.inner.lock().clone(),
            // otherwise, clone inner
            |cart| cart.inner.into_inner(),
        );

        // Insert the cartridge
        self.inner.write().insert(cart);
    }

    /// Ejects the inserted game cartridge.
    ///
    /// If no cartridge was inserted, this is a no-op. Check the return value to
    /// see if a cartridge was ejected.
    #[uniffi::method]
    pub fn eject(&self) -> Option<Arc<Cartridge>> {
        // Eject the cartridge
        self.inner.write().eject().map(|cart| {
            Arc::new(Cartridge {
                inner: Mutex::new(cart),
            })
        })
    }
}

/// Game cartridge.
///
/// Models the hardware specified by the provided ROM.
#[derive(Debug, uniffi::Object)]
pub struct Cartridge {
    /// Internal cartridge model.
    inner: Mutex<dmg::Cartridge>,
}

/// Checks a if ROM has can reasonably be constructed.
///
/// # Errors
///
/// Returns an error if the ROM contained invalid header bytes, or if
/// cartridge integrity seems compromised. (This is detected using
/// checksums.)
#[uniffi::export]
pub fn check(data: &[u8]) -> Result<()> {
    dmg::Cartridge::check(data).map_err(Into::into)
}

#[uniffi::export]
impl Cartridge {
    // Constructs a new `Cartridge`.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge could not be constructed. This will
    /// either be due to invalid header bytes or an unsupported cartridge type.
    #[uniffi::constructor]
    pub fn new(data: &[u8]) -> Result<Self> {
        // Construct internal game cartridge
        dmg::Cartridge::new(data)
            .map(|cart| Self {
                // Build external model from game
                inner: Mutex::new(cart),
            })
            .map_err(Into::into)
    }

    /// Retrieves the cartridge header.
    #[uniffi::method]
    pub fn header(&self) -> Header {
        self.inner.lock().header().clone().into()
    }

    /// Flashes data onto the cartridge's RAM.
    #[uniffi::method]
    pub fn flash(&self, save: &[u8]) -> Result<()> {
        // Flash data to RAM
        self.inner
            .lock()
            .flash(&mut io::Cursor::new(save))
            .map(|_| ())
            .map_err(Into::into)
    }

    /// Dumps contents of the cartridge's RAM.
    #[uniffi::method]
    pub fn dump(&self) -> Result<Vec<u8>> {
        // Dump RAM as data
        let mut buf = Vec::new();
        self.inner.lock().dump(&mut buf)?;
        Ok(buf)
    }
}

// SAFETY: `Cartridge` does not expose its thread-unsafe internals.
//
// This invariant is respected as long as no part of this API exposes any
// internal reference-counting pointers.
unsafe impl Send for Cartridge {}

// SAFETY: `Cartridge` does not expose its thread-unsafe internals.
//
// This invariant is respected as long as no part of this API exposes any
// internal reference-counting pointers.
unsafe impl Sync for Cartridge {}

use parts::{About, Board, Check, Compat, Memory};

/// Cartridge header.
///
/// See [`rugby::core::dmg::cart::Header`]
#[derive(Clone, Debug, Eq, PartialEq, uniffi::Record)]
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

/// Constructs a new `Header`.
///
/// # Errors
///
/// Returns an error if the ROM contained invalid header bytes.
#[uniffi::export]
pub fn header(data: &[u8]) -> Result<Header> {
    dmg::cart::Header::new(data)
        .map(Into::into)
        .map_err(Into::into)
}

/// Calculates the header checksum.
#[uniffi::export]
pub fn hchk(data: &[u8]) -> u8 {
    dmg::cart::head::hchk(data)
}

/// Calculates the global checksum.
#[uniffi::export]
pub fn gchk(data: &[u8]) -> u16 {
    dmg::cart::head::gchk(data)
}

impl From<dmg::cart::Header> for Header {
    fn from(
        dmg::cart::Header {
            about,
            check,
            board,
            memory,
            compat,
        }: dmg::cart::Header,
    ) -> Self {
        Self {
            about,
            check,
            board: board.into(),
            memory: memory.into(),
            compat,
        }
    }
}

/// Header fields.
pub mod parts {
    pub use parts::About;
    use rugby::core::dmg::cart::head::parts;

    /// Game information.
    ///
    /// See [`rugby::core::dmg::cart::head::parts::About`]
    #[uniffi::remote(Record)]
    pub struct About {
        pub title: Option<String>,
        pub region: Region,
        pub version: u8,
    }

    pub use parts::Check;

    /// Data integrity.
    ///
    /// See [`rugby::core::dmg::cart::head::parts::Check`]
    #[uniffi::remote(Record)]
    pub struct Check {
        pub logo: bool,
        pub hchk: u8,
        pub gchk: u16,
    }

    /// Memory hardware.
    ///
    /// See [`rugby::core::dmg::cart::head::parts::Memory`]
    #[derive(Clone, Debug, Eq, PartialEq, uniffi::Record)]
    pub struct Memory {
        pub romsz: u32,
        pub ramsz: u32,
    }

    impl From<parts::Memory> for Memory {
        fn from(parts::Memory { romsz, ramsz }: parts::Memory) -> Self {
            Self {
                romsz: romsz as u32,
                ramsz: ramsz as u32,
            }
        }
    }

    pub use parts::Compat;

    /// Model compatibility.
    ///
    /// See [`rugby::core::dmg::cart::head::parts::Compat`]
    #[uniffi::remote(Record)]
    pub struct Compat {
        pub dmg: bool,
        pub cgb: bool,
        pub sgb: bool,
    }

    pub use parts::Board;

    /// Mapper hardware.
    ///
    /// See [`rugby::core::dmg::cart::head::parts::Board`]
    #[uniffi::remote(Enum)]
    pub enum Board {
        None {
            exram: bool,
            power: bool,
        },
        Mbc1 {
            exram: bool,
            power: bool,
        },
        Mbc2 {
            power: bool,
        },
        Mbc3 {
            exram: bool,
            power: bool,
            clock: bool,
        },
        Mbc5 {
            exram: bool,
            power: bool,
            motor: bool,
        },
        Mbc6,
        Mbc7,
        Mmm01 {
            exram: bool,
            power: bool,
        },
        M161,
        HuC1,
        HuC3,
        Camera,
    }

    pub use parts::Region;

    /// Destination code.
    ///
    /// See [`rugby::core::dmg::cart::head::parts::Region`]
    #[uniffi::remote(Enum)]
    pub enum Region {
        World,
        Japan,
    }
}
