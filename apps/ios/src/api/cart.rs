//! Cartridge API.

use std::sync::{Arc, Mutex};

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
        // Take unique ownership of this cartridge's game.
        let cart = cart
            .game
            .try_lock()
            .map_err(|err| err.to_string())?
            .take()
            .ok_or("cartridge is missing game data".to_string())?;
        // Insert the cartridge
        self.0.write().unwrap().insert(cart);
        Ok(())
    }

    /// Ejects the inserted game cartridge.
    ///
    /// If no cartridge was inserted, this is a no-op. Check the return value to
    /// see if a cartridge was ejected.
    #[uniffi::method]
    pub fn eject(&self) -> bool {
        self.0.write().unwrap().eject().is_some()
    }
}

/// Game cartridge.
///
/// Models the hardware specified by the provided ROM.
#[derive(Debug, uniffi::Object)]
pub struct Cartridge {
    /// Cartridge header information.
    info: Header,
    /// Internal cartridge model.
    game: Mutex<Option<dmg::Cartridge>>,
}

#[uniffi::export]
impl Cartridge {
    /// Constructs a new `Cartridge`.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge could not be constructed. This will
    /// either be due to invalid header bytes or an unsupported cartridge type.
    #[uniffi::constructor]
    pub fn new(rom: &[u8]) -> Result<Cartridge> {
        // Construct internal game cartridge
        let game = dmg::Cartridge::new(rom).map_err(|err| err.to_string())?;
        // Build external model from game
        Ok(Self {
            info: game.header().into(),
            game: Mutex::new(game.into()),
        })
    }

    /// Retrieves the cartridge header.
    #[uniffi::method]
    pub fn header(&self) -> Header {
        self.info.clone()
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
#[derive(Clone, Debug, uniffi::Record)]
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

impl From<&dmg::cart::header::Header> for Header {
    fn from(header: &dmg::cart::header::Header) -> Self {
        Self {
            title: header.title.clone(),
            dmg: header.dmg,
            cgb: header.cgb,
            sgb: header.sgb,
            cart: header.info.to_string(),
            romsz: bfmt::Size::from(header.romsz).to_string(),
            ramsz: bfmt::Size::from(header.ramsz).to_string(),
            region: header.region.to_string(),
            version: header.revision(),
            hchk: header.hchk,
            gchk: header.gchk,
        }
    }
}
