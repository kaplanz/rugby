//! Cartridge API.

use rugby::core::dmg;
use wasm_bindgen::prelude::*;

use super::GameBoy;

#[wasm_bindgen]
impl GameBoy {
    /// Inserts a game cartridge.
    ///
    /// If a cartridge is already inserted, it will first be
    /// [ejected](Self::eject).
    pub fn insert(&mut self, cart: Cartridge) {
        self.0.insert(cart.0);
    }

    /// Ejects the inserted game cartridge.
    ///
    /// If no cartridge was inserted, this is a no-op. Check the return value to
    /// see if a cartridge was ejected.
    pub fn eject(&mut self) -> bool {
        self.0.eject().is_some()
    }
}

/// Game cartridge.
///
/// Models the hardware specified by the provided ROM.
#[derive(Debug)]
#[wasm_bindgen]
pub struct Cartridge(dmg::Cartridge);

#[wasm_bindgen]
impl Cartridge {
    /// Constructs a new `Cartridge`.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge could not be constructed. This will
    /// either be due to invalid header bytes or an unsupported cartridge type.
    #[wasm_bindgen(constructor)]
    pub fn new(rom: &[u8]) -> Result<Cartridge, JsError> {
        Ok(Self(dmg::Cartridge::new(rom)?))
    }
}
