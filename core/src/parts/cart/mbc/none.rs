use rugby_arch::mio::Device;
use rugby_arch::{Block, Shared};

use super::{Data, Mbc};

/// Cartridge ROM.
type Rom = rugby_arch::mem::Rom<Data>;
/// Cartridge RAM.
type Ram = rugby_arch::mem::Ram<Data>;

/// [No MBC][none] cartridge type.
///
/// [none]: https://gbdev.io/pandocs/nombc.html
#[derive(Debug)]
pub struct None {
    pub(super) rom: Shared<Rom>,
    pub(super) ram: Shared<Ram>,
}

impl None {
    /// Constructs a new `None`.
    #[must_use]
    pub fn new(rom: Data, ram: Data) -> Self {
        Self {
            rom: Shared::new(Rom::from(rom)),
            ram: Shared::new(Ram::from(ram)),
        }
    }
}

impl Block for None {}

impl Mbc for None {
    fn rom(&self) -> Device {
        self.rom.clone().into()
    }

    fn ram(&self) -> Device {
        self.ram.clone().into()
    }
}
