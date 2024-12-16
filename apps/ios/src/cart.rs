//! Cartridge model.

use rugby::core::dmg;
use uniffi::Object;

use super::GameBoy;

/// Game cartridge.
#[derive(Object)]
pub struct Cartridge(inner::Model);

#[uniffi::export]
impl Cartridge {
    /// Constructs a new `Cartridge`.
    ///
    /// # Errors
    ///
    /// Returns an error when the cartridge could not be constructed. This will
    /// either be due to invalid header bytes or an unsupported cartridge type.
    #[uniffi::constructor]
    pub fn new(rom: &[u8]) -> Cartridge {
        Self(inner::Model(dmg::Cartridge::new(rom).unwrap()))
    }
}

#[uniffi::export]
impl GameBoy {
    /// Inserts a game cartridge.
    ///
    /// If a cartridge is already inserted, it will first be
    /// [ejected](Self::eject).
    #[uniffi::method]
    pub fn insert(&self, rom: &[u8]) {
        self.0
            .write()
            .unwrap()
            .insert(dmg::Cartridge::new(rom).unwrap());
    }
}

mod inner {
    //! Private model implementation.
    use std::ops::{Deref, DerefMut};

    use rugby::core::dmg::Cartridge;

    /// Inner emulator model.
    pub struct Model(pub(crate) Cartridge);

    impl Deref for Model {
        type Target = Cartridge;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Model {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    // SAFETY: `Cartridge` does not expose its thread-unsafe internals.
    unsafe impl Send for Model {}

    // SAFETY: `Cartridge` does not expose its thread-unsafe internals.
    unsafe impl Sync for Model {}
}
