//! Cartridge model.

use rugby::core::dmg;
use uniffi::{Object, Record};

use super::GameBoy;

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

/// Game cartridge.
#[derive(Debug, Object)]
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

    #[uniffi::method]
    fn info(&self) -> Info {
        self.0.header().into()
    }
}

/// Game information.
#[derive(Debug, Record)]
pub struct Info {
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

impl From<&dmg::cart::header::Header> for Info {
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

/// Private model implementation.
mod inner {
    use std::ops::{Deref, DerefMut};

    use rugby::core::dmg::Cartridge;

    /// Inner emulator model.
    #[derive(Debug)]
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
