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
        // Eject previous cartridge, if any
        if self.eject() {
            return Err("inserted cartiridge without previous eject"
                .to_string()
                .into());
        }
        // Take unique ownership of this cartridge's game.
        let game = cart
            .game
            .try_lock()
            .map_err(|err| err.to_string())?
            .take()
            .ok_or("cartridge is missing game data".to_string())?;
        // Insert the cartridge
        self.emu.write().unwrap().insert(game);
        // Retain cartridge
        self.pak.write().unwrap().replace(cart);

        Ok(())
    }

    /// Ejects the inserted game cartridge.
    ///
    /// If no cartridge was inserted, this is a no-op. Check the return value to
    /// see if a cartridge was ejected.
    #[uniffi::method]
    pub fn eject(&self) -> bool {
        // Retrieve cartridge
        let Some(cart) = self.pak.write().unwrap().take() else {
            return false;
        };
        // Eject inserted game
        let Some(game) = self.emu.write().unwrap().eject() else {
            return false;
        };
        // Ensure game matches cartridge
        if cart.header() != game.header().into() {
            return false;
        }
        // Restore ownership of this cartridge's game
        cart.game.lock().unwrap().replace(game);
        true
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

    /// Checks if the game is currently in use.
    fn busy(&self) -> bool {
        self.game.lock().is_ok_and(|cart| cart.is_some())
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

impl From<&dmg::cart::header::Header> for Header {
    fn from(header: &dmg::cart::header::Header) -> Self {
        Self {
            title: header.about.title.clone(),
            dmg: header.compat.dmg,
            cgb: header.compat.cgb,
            sgb: header.compat.sgb,
            cart: header.board.to_string(),
            romsz: bfmt::Size::from(header.memory.romsz).to_string(),
            ramsz: bfmt::Size::from(header.memory.ramsz).to_string(),
            region: header.about.region.to_string(),
            version: header.about.revision(),
            hchk: header.check.hchk,
            gchk: header.check.gchk,
        }
    }
}
