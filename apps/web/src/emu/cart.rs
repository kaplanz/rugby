use rugby::core::dmg;
use wasm_bindgen::prelude::*;

use super::GameBoy;

/// Game cartridge.
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

#[wasm_bindgen]
impl GameBoy {
    /// Inserts a game cartridge.
    ///
    /// If a cartridge is already inserted, it will first be
    /// [ejected](Self::eject).
    pub fn insert(&mut self, cart: Cartridge) {
        self.0.insert(cart.0);
    }

    /// Ejects the inserted game cartridge, if any.
    pub fn eject(&mut self) -> Option<Cartridge> {
        self.0.eject().map(Cartridge)
    }
}
