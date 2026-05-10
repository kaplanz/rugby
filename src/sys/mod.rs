//! Game Boy models.

use std::io::{BufRead, Write};

pub use self::dmg::Dmg;
use crate::arch::Block;
use crate::core::api::audio::{Audio, Chiptune};
use crate::core::api::cable::Cable;
use crate::core::api::input::{Event, Input};
use crate::core::api::video::{Aspect, Video};
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

impl Audio for GameBoy {
    fn sample(&self) -> Chiptune {
        match self {
            Self::Dmg(dmg) => dmg.sample(),
        }
    }
}

impl Cable for GameBoy {
    fn rx(&mut self) -> &mut dyn BufRead {
        match self {
            Self::Dmg(dmg) => dmg.rx(),
        }
    }

    fn tx(&mut self) -> &mut dyn Write {
        match self {
            Self::Dmg(dmg) => dmg.tx(),
        }
    }
}

impl Input for GameBoy {
    type Button = <Dmg as Input>::Button;

    fn recv(&mut self, events: impl IntoIterator<Item = Event<Self::Button>>) {
        match self {
            Self::Dmg(dmg) => dmg.recv(events),
        }
    }
}

impl Video for GameBoy {
    const SIZE: Aspect = Dmg::SIZE;

    type Pixel = <Dmg as Video>::Pixel;

    fn vsync(&self) -> bool {
        match self {
            Self::Dmg(dmg) => dmg.vsync(),
        }
    }

    fn frame(&self) -> &[Self::Pixel] {
        match self {
            Self::Dmg(dmg) => dmg.frame(),
        }
    }
}
