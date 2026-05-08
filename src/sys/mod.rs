//! Game Boy models.

pub use self::dmg::Dmg;
use crate::arch::Block;
use crate::core::cart::Cartridge;

mod dmg;

/// Game Boy handheld game console.
#[derive(Debug)]
#[non_exhaustive]
pub enum GameBoy {
    /// Game Boy (DMG).
    Dmg(Dmg),
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::Dmg(Dmg::default())
    }
}

impl Block for GameBoy {
    fn ready(&self) -> bool {
        match self {
            Self::Dmg(dmg) => dmg.ready(),
        }
    }

    fn cycle(&mut self) {
        match self {
            Self::Dmg(dmg) => dmg.cycle(),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::Dmg(dmg) => dmg.reset(),
        }
    }
}

impl GameBoy {
    /// Gets the inserted cartridge, if any.
    #[must_use]
    pub fn cart(&self) -> Option<&Cartridge> {
        match self {
            Self::Dmg(dmg) => dmg.cart(),
        }
    }

    /// Inserts a cartridge.
    pub fn insert(&mut self, cart: Cartridge) {
        match self {
            Self::Dmg(dmg) => dmg.insert(cart),
        }
    }

    /// Ejects the inserted cartridge, if any.
    #[must_use]
    pub fn eject(&mut self) -> Option<Cartridge> {
        match self {
            Self::Dmg(dmg) => dmg.eject(),
        }
    }
}
