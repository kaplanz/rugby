use rugby_arch::mio::Device;
use rugby_arch::{Block, Shared};

use super::{Data, Mbc};

/// Cartridge ROM.
type Rom = rugby_arch::mem::Rom<Data>;
/// Cartridge RAM.
type Ram = rugby_arch::mem::Ram<Data>;

/// Bare ROM + RAM.
#[derive(Debug)]
pub struct Bare {
    pub(super) rom: Shared<Rom>,
    pub(super) ram: Shared<Ram>,
}

impl Bare {
    /// Constructs a new `Bare`.
    #[must_use]
    pub fn new(rom: Data, ram: Data) -> Self {
        Self {
            rom: Shared::new(Rom::from(rom)),
            ram: Shared::new(Ram::from(ram)),
        }
    }
}

impl Block for Bare {}

impl Mbc for Bare {
    fn rom(&self) -> Device {
        self.rom.clone().into()
    }

    fn ram(&self) -> Device {
        self.ram.clone().into()
    }
}
