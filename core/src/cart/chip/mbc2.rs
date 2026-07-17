use log::{debug, trace};
use rugby_arch::mem::{Error, Memory, Result};
use rugby_arch::mio::Device;
use rugby_arch::reg::Register;
use rugby_arch::{Block, Shared};

use super::{Data, Mbc};

/// [MBC2][mbc2] cartridge type.
///
/// [mbc2]: https://gbdev.io/pandocs/MBC2.html
#[derive(Clone, Debug)]
pub struct Mbc2 {
    reg: File,
    pub(super) rom: Shared<Rom>,
    pub(super) ram: Shared<Ram>,
}

impl Mbc2 {
    /// Constructs a new `Mbc2`.
    #[must_use]
    pub fn new(rom: Data, ram: Data) -> Self {
        let reg = File::default();
        Self {
            rom: Shared::new(Rom::new(reg.clone(), rom)),
            ram: Shared::new(Ram::new(reg.clone(), ram)),
            reg,
        }
    }
}

impl Block for Mbc2 {
    fn reset(&mut self) {
        self.reg.reset();
    }
}

impl Mbc for Mbc2 {
    fn rom(&self) -> Device {
        self.rom.clone().into()
    }

    fn ram(&self) -> Device {
        self.ram.clone().into()
    }
}

/// MBC2 registers.
///
/// |     Address     | Size | Name | Description      |
/// |:---------------:|------|------|------------------|
/// | `$0000..=$3FFF` | 1bit | ENA  | RAM Enable.      |
/// | `$0000..=$3FFF` | 4bit | ROM  | ROM Bank Number. |
///
/// # Note
///
/// Bit 8 of the address selects the register: ENA if clear, ROM if set.
#[rustfmt::skip]
#[derive(Clone, Debug, Default)]
struct File {
    /// RAM Enable.
    ena: Shared<Enable>,
    /// ROM Bank Number.
    rom: Shared<RomBank>,
}

impl Block for File {
    fn reset(&mut self) {
        self.ena.take();
        self.rom.take();
    }
}

/// RAM Enable.
#[derive(Debug, Default)]
struct Enable(bool);

impl Enable {
    const MASK: u8 = 0x0f;
}

impl Memory for Enable {
    fn read(&self, _: u16) -> Result<u8> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: u16, data: u8) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Enable {
    type Value = u8;

    fn load(&self) -> Self::Value {
        u8::from(self.0)
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = Self::MASK & value == 0x0a;
        debug!("RAM Enable: {}", self.0);
    }
}

/// ROM Bank Number.
#[derive(Debug, Default)]
struct RomBank(u8);

impl RomBank {
    const MASK: u8 = 0x0f;
}

impl Memory for RomBank {
    fn read(&self, _: u16) -> Result<u8> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: u16, data: u8) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for RomBank {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.0 & Self::MASK
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = Self::MASK & value;
        debug!("ROM Bank Number: {:#04x}", self.0);
    }
}

/// MBC2 ROM.
#[derive(Debug)]
pub(super) struct Rom {
    reg: File,
    pub(super) mem: Data,
}

impl Rom {
    /// Constructs a new `Rom`.
    fn new(reg: File, mem: Data) -> Self {
        Self { reg, mem }
    }

    /// Adjusts addresses by internal bank number.
    fn adjust(&self, addr: u16) -> usize {
        let bank = match usize::from(self.reg.rom.load()) {
            0 => 1,
            x => x,
        };
        let addr = usize::from(addr);
        ((bank << 14) | addr & 0x3fff) % self.mem.len().max(0x8000)
    }
}

impl Memory for Rom {
    fn read(&self, addr: u16) -> Result<u8> {
        let index = match addr {
            0x0000..=0x3fff => usize::from(addr),
            0x4000..=0x7fff => self.adjust(addr),
            _ => return Err(Error::Range),
        };
        self.mem.get(index).ok_or(Error::Range).copied()
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        trace!("Mbc2::write(${addr:04x}, {data:#04x})");
        match addr {
            0x0000..=0x3fff => {
                // RAM Enable
                if addr & 0x0100 == 0 {
                    // reg.ena <- data[3:0] == 0xA
                    self.reg.ena.store(data);
                }
                // ROM Bank Number
                else {
                    // reg.rom[3:0] <- data[3:0]
                    self.reg.rom.store(data);
                }
            }
            _ => return Err(Error::Range),
        }
        Ok(())
    }
}

/// MBC2 RAM.
///
/// # Note
///
/// The controller contains 512 half-bytes of built-in RAM. Only the lower
/// nibble of each byte is driven. Accesses echo every 512 bytes across the
/// full address range.
#[derive(Debug)]
pub(super) struct Ram {
    reg: File,
    pub(super) mem: Data,
}

impl Ram {
    /// Constructs a new `Ram`.
    fn new(reg: File, mem: Data) -> Self {
        Self { reg, mem }
    }

    /// Adjusts addresses by internal memory size.
    fn adjust(addr: u16) -> usize {
        usize::from(addr) & 0x01ff
    }
}

impl Memory for Ram {
    fn read(&self, addr: u16) -> Result<u8> {
        // Error when disabled
        if self.reg.ena.load() == 0 {
            return Err(Error::Disabled);
        }
        // Perform adjusted read
        let index = Self::adjust(addr);
        let data = self.mem.get(index).ok_or(Error::Range).copied()?;
        // NOTE: The upper nibble is undriven, so it always reads as ones.
        Ok(0xf0 | data)
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        // Error when disabled
        if self.reg.ena.load() == 0 {
            return Err(Error::Disabled);
        }
        // Perform adjusted write
        let index = Self::adjust(addr);
        *self.mem.get_mut(index).ok_or(Error::Range)? = data & 0x0f;
        Ok(())
    }
}
