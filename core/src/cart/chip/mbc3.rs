use log::{debug, error, trace};
use rugby_arch::mem::{Error, Memory, Result};
use rugby_arch::mio::Device;
use rugby_arch::reg::Register;
use rugby_arch::{Block, Shared};

use super::{Data, Mbc};

/// [MBC3][mbc3] cartridge type.
///
/// [mbc3]: https://gbdev.io/pandocs/MBC3.html
#[derive(Clone, Debug)]
pub struct Mbc3 {
    reg: File,
    pub(super) rom: Shared<Rom>,
    pub(super) ram: Shared<Ram>,
    #[expect(unused)]
    rtc: Rtc,
}

impl Mbc3 {
    /// Constructs a new `Mbc3`.
    #[must_use]
    pub fn new(rom: Data, ram: Data) -> Self {
        let reg = File::default();
        Self {
            rom: Shared::new(Rom::new(reg.clone(), rom)),
            ram: Shared::new(Ram::new(reg.clone(), ram)),
            reg,
            rtc: Rtc,
        }
    }
}

impl Block for Mbc3 {
    fn reset(&mut self) {
        self.reg.reset();
    }
}

impl Mbc for Mbc3 {
    fn rom(&self) -> Device {
        self.rom.clone().into()
    }

    fn ram(&self) -> Device {
        self.ram.clone().into()
    }
}

/// MBC3 registers.
///
/// |     Address     | Size | Name | Description         |
/// |:---------------:|------|------|---------------------|
/// | `$0000..=$1FFF` | 1bit | ENA  | RAM + Timer Enable. |
/// | `$2000..=$3FFF` | 7bit | ROM  | ROM Bank Number.    |
/// | `$4000..=$5FFF` | 4bit | RAM  | RAM Bank Number.    |
/// | `$6000..=$7FFF` | 1bit | LCD  | Latch Clock Data.   |
#[rustfmt::skip]
#[derive(Clone, Debug, Default)]
struct File {
    /// RAM + Timer Enable.
    ena: Shared<Enable>,
    /// ROM Bank Number.
    rom: Shared<RomBank>,
    /// RAM Bank Number.
    ram: Shared<RamBank>,
    /// Latch Clock Data.
    lcd: Shared<Latch>,
}

impl Block for File {
    fn reset(&mut self) {
        self.ena.take();
        self.rom.take();
        self.ram.take();
        self.lcd.take();
    }
}

/// ROM + Timer Enable.
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
        debug!("RAM + Timer Enable: {}", self.0);
    }
}

/// ROM Bank Number.
#[derive(Debug, Default)]
struct RomBank(u8);

impl RomBank {
    const MASK: u8 = 0x7f;
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

/// RAM Bank Number.
#[derive(Debug, Default)]
struct RamBank(u8);

impl RamBank {
    const MASK: u8 = 0x0f;
}

impl Memory for RamBank {
    fn read(&self, _: u16) -> Result<u8> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: u16, data: u8) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for RamBank {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.0 & Self::MASK
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = Self::MASK & value;
        debug!("RAM Bank Number: {:#04x}", self.0);
    }
}

/// Latch Clock Data.
#[derive(Debug, Default)]
struct Latch(bool);

impl Latch {
    const MASK: u8 = 0x01;
}

impl Memory for Latch {
    fn read(&self, _: u16) -> Result<u8> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: u16, data: u8) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Latch {
    type Value = u8;

    fn load(&self) -> Self::Value {
        u8::from(self.0)
    }

    fn store(&mut self, value: Self::Value) {
        let value = Self::MASK & value != 0;
        if !self.0 && value {
            debug!("latched clock data");
        }
        self.0 = value;
    }
}

/// MBC3 ROM.
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
        trace!("Mbc3::write(${addr:04x}, {data:#04x})");
        match addr {
            // RAM Enable
            0x0000..=0x1fff => {
                // reg.ena <- data[3:0] == 0xA
                self.reg.ena.store(data);
            }
            // ROM Bank Number
            0x2000..=0x3fff => {
                // reg.rom[4:0] <- data[4:0]
                self.reg.rom.store(data);
            }
            // RAM Bank Number
            0x4000..=0x5fff => {
                // reg.rom[1:0] <- data[1:0]
                self.reg.ram.store(data);
            }
            // Banking Mode Select
            0x6000..=0x7fff => {
                // reg.sel <- data[3:0] == 0xA
                error!("unimplemented: Mbc3::write(${addr:04x}, {data:#04x})");
                self.reg.lcd.store(data);
            }
            _ => return Err(Error::Range),
        }
        Ok(())
    }
}

/// MBC3 RAM.
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

    /// Adjusts addresses by internal bank number.
    fn adjust(&self, addr: u16) -> usize {
        let bank = usize::from(self.reg.ram.load());
        let addr = usize::from(addr);
        ((bank << 13) | addr & 0x1fff) % self.mem.len().max(1)
    }
}

impl Memory for Ram {
    fn read(&self, addr: u16) -> Result<u8> {
        // Error when disabled
        if self.reg.ena.load() == 0 {
            return Err(Error::Disabled);
        }
        // Error when RTC is selected
        if self.reg.ram.load() & 0x08 != 0 {
            return Err(Error::Disabled);
        }
        // Perform adjusted read
        let index = self.adjust(addr);
        self.mem.get(index).ok_or(Error::Range).copied()
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        // Error when disabled
        if self.reg.ena.load() == 0 {
            return Err(Error::Disabled);
        }
        // Error when RTC is selected
        if self.reg.ram.load() & 0x08 != 0 {
            return Err(Error::Disabled);
        }
        // Perform adjusted write
        let index = self.adjust(addr);
        *self.mem.get_mut(index).ok_or(Error::Range)? = data;
        Ok(())
    }
}

/// MBC3 real-time clock.
#[derive(Clone, Debug)]
pub struct Rtc;
