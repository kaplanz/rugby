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
    pub fn insert(&self, cart: Arc<Cartridge>) -> Result<()> {
        // Ensure cartridge slot is empty
        if self.inner.read().cart().is_some() {
            return Err("cartridge slot is not empty".to_string().into());
        }

        // Obtain internal cartridge model
        let cart = Arc::try_unwrap(cart).map_or_else(
            // consume cart if unique
            |cart| cart.inner.lock().clone(),
            // otherwise, clone inner
            |cart| cart.inner.into_inner(),
        );

        // Insert the cartridge
        self.inner.write().insert(cart);

        Ok(())
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
    dmg::Cartridge::check(data).map_err(|err| err.to_string().into())
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
            .map_err(|err| err.to_string().into())
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
            .map_err(|err| err.to_string().into())
    }

    /// Dumps contents of the cartridge's RAM.
    #[uniffi::method]
    pub fn dump(&self) -> Result<Vec<u8>> {
        // Dump RAM as data
        let mut buf = Vec::new();
        self.inner
            .lock()
            .dump(&mut buf)
            .map_err(|err| err.to_string())?;
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

/// Cartridge header.
///
/// Information about the cartridge ROM.
#[derive(Clone, Debug, Eq, PartialEq, uniffi::Record)]
pub struct Header {
    /// Internal game title.
    pub title: Option<String>,
    /// Compatible with DMG (Game Boy).
    pub dmg: bool,
    /// Compatible with CGB (Game Boy Color).
    pub cgb: bool,
    /// Compatible with SGB (Super Game Boy).
    pub sgb: bool,
    /// Cartridge description.
    pub cart: String,
    /// Cartridge ROM size
    pub romsz: String,
    /// Cartridge RAM size.
    pub ramsz: String,
    /// Destination code.
    pub region: String,
    /// Game revision number.
    pub version: String,
    /// Header checksum.
    pub hchk: u8,
    /// Global checksum.
    pub gchk: u16,
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
        .map_err(|err| err.to_string().into())
}

impl From<dmg::cart::Header> for Header {
    fn from(head: dmg::cart::Header) -> Self {
        Self {
            title: head.about.title.clone(),
            dmg: head.compat.dmg,
            cgb: head.compat.cgb,
            sgb: head.compat.sgb,
            cart: head.board.to_string(),
            romsz: bfmt::Size::from(head.memory.romsz).to_string(),
            ramsz: bfmt::Size::from(head.memory.ramsz).to_string(),
            region: head.about.region.to_string(),
            version: head.about.revision(),
            hchk: head.check.hchk,
            gchk: head.check.gchk,
        }
    }
}
