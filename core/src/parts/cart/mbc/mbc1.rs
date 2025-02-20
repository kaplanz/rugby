use std::io;

use log::{debug, trace};
use rugby_arch::mem::{Error, Memory, Result};
use rugby_arch::mio::Device;
use rugby_arch::reg::Register;
use rugby_arch::{Block, Byte, Shared, Word};

use super::{Data, Mbc};

/// [MBC1][mbc1] cartridge type.
///
/// [mbc1]: https://gbdev.io/pandocs/MBC1.html
#[derive(Debug)]
pub struct Mbc1 {
    ctl: Control,
    rom: Shared<Rom>,
    ram: Shared<Ram>,
}

impl Mbc1 {
    /// Constructs a new `Mbc1`.
    #[must_use]
    pub fn new(rom: Data, ram: Data) -> Self {
        let ctl = Control::default();
        Self {
            rom: Shared::new(Rom::new(ctl.clone(), rom)),
            ram: Shared::new(Ram::new(ctl.clone(), ram)),
            ctl,
        }
    }
}

impl Block for Mbc1 {
    fn reset(&mut self) {
        self.ctl.reset();
    }
}

impl Mbc for Mbc1 {
    fn rom(&self) -> Device {
        self.rom.clone().into()
    }

    fn ram(&self) -> Device {
        self.ram.clone().into()
    }

    fn flash(&mut self, buf: &mut impl io::Read) -> io::Result<usize> {
        buf.read(&mut self.ram.borrow_mut().mem)
    }

    fn dump(&self, buf: &mut impl io::Write) -> io::Result<usize> {
        buf.write(&self.ram.borrow().mem)
    }
}

/// MBC1 registers.
///
/// |     Address     | Size | Name | Description          |
/// |:---------------:|------|------|----------------------|
/// | `$0000..=$1FFF` | 1bit | ENA  | RAM Enable.          |
/// | `$2000..=$3FFF` | 5bit | ROM  | ROM Bank Number.     |
/// | `$4000..=$5FFF` | 2bit | RAM  | RAM Bank Number.     |
/// | `$6000..=$7FFF` | 1bit | SEL  | Banking Mode Select. |
#[rustfmt::skip]
#[derive(Clone, Debug, Default)]
struct Control {
    /// RAM Enable.
    ena: Shared<Enable>,
    /// ROM Bank Number.
    rom: Shared<RomBank>,
    /// RAM Bank Number.
    ram: Shared<RamBank>,
    /// Banking Mode Select.
    sel: Shared<Select>,
}

impl Block for Control {
    fn reset(&mut self) {
        self.ena.take();
        self.rom.take();
        self.ram.take();
        self.sel.take();
    }
}

/// ROM Enable.
#[derive(Debug, Default)]
struct Enable(bool);

impl Enable {
    const MASK: u8 = 0x0f;
}

impl Memory for Enable {
    fn read(&self, _: Word) -> Result<Byte> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: Word, data: Byte) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Enable {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        Byte::from(self.0) & Self::MASK
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = Self::MASK & value == 0x0a;
        debug!("RAM Enable: {}", self.0);
    }
}

/// ROM Bank Number.
#[derive(Debug, Default)]
struct RomBank(Byte);

impl RomBank {
    const MASK: u8 = 0x1f;
}

impl Memory for RomBank {
    fn read(&self, _: Word) -> Result<Byte> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: Word, data: Byte) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for RomBank {
    type Value = Byte;

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
struct RamBank(Byte);

impl RamBank {
    const MASK: u8 = 0x03;
}

impl Memory for RamBank {
    fn read(&self, _: Word) -> Result<Byte> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: Word, data: Byte) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for RamBank {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 & Self::MASK
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = Self::MASK & value;
        debug!("RAM Bank Number: {:#04x}", self.0);
    }
}

/// Banking Mode Select.
#[derive(Debug, Default)]
struct Select(bool);

impl Select {
    const MASK: u8 = 0x01;
}

impl Memory for Select {
    fn read(&self, _: Word) -> Result<Byte> {
        Err(Error::Misuse)
    }

    fn write(&mut self, _: Word, data: Byte) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Select {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        Byte::from(self.0)
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = Self::MASK & value != 0;
        debug!("Banking Mode Select: {}", self.0);
    }
}

/// MBC1 ROM.
#[derive(Debug)]
struct Rom {
    ctl: Control,
    mem: Data,
}

impl Rom {
    /// Constructs a new `Rom`.
    fn new(ctl: Control, mem: Data) -> Self {
        Self { ctl, mem }
    }

    /// Adjusts addresses by internal bank number.
    fn adjust0(&self, addr: Word) -> usize {
        let bank = {
            let lo = 0;
            let hi = if self.ctl.sel.load() == 0 {
                0
            } else {
                usize::from(self.ctl.ram.load())
            };
            (hi << 5) | lo
        };
        let addr = usize::from(addr);
        ((bank << 14) | addr & 0x3fff) % self.mem.len().max(0x8000)
    }

    /// Adjusts addresses by internal bank number.
    fn adjust1(&self, addr: Word) -> usize {
        let bank = {
            let lo = match usize::from(self.ctl.rom.load()) {
                0 => 1,
                x => x,
            };
            let hi = usize::from(self.ctl.ram.load());
            (hi << 5) | lo
        };
        let addr = usize::from(addr);
        ((bank << 14) | addr & 0x3fff) % self.mem.len().max(0x8000)
    }
}

impl Memory for Rom {
    fn read(&self, addr: Word) -> Result<Byte> {
        // Translate address
        let index = match addr {
            0x0000..=0x3fff => self.adjust0(addr),
            0x4000..=0x7fff => self.adjust1(addr),
            _ => return Err(Error::Range),
        };
        if usize::from(addr) != index {
            trace!("address translation: ${addr:04x} -> ${index:04x}");
        }
        // Perform read
        self.mem.get(index).ok_or(Error::Range).copied()
    }

    fn write(&mut self, addr: Word, data: Byte) -> Result<()> {
        trace!("Mbc1::write(${addr:04x}, {data:#04x})");
        match addr {
            // RAM Enable
            0x0000..=0x1fff => {
                // ctl.ena <- data[3:0] == 0xA
                self.ctl.ena.store(data);
            }
            // ROM Bank Number
            0x2000..=0x3fff => {
                // ctl.rom[4:0] <- data[4:0]
                self.ctl.rom.store(data);
            }
            // RAM Bank Number
            0x4000..=0x5fff => {
                // ctl.rom[1:0] <- data[1:0]
                self.ctl.ram.store(data);
            }
            // Banking Mode Select
            0x6000..=0x7fff => {
                // ctl.sel <- data[0]
                self.ctl.sel.store(data);
            }
            _ => return Err(Error::Range),
        }
        Ok(())
    }
}

/// MBC1 RAM.
#[derive(Debug)]
struct Ram {
    ctl: Control,
    mem: Data,
}

impl Ram {
    /// Constructs a new `Ram`.
    fn new(ctl: Control, mem: Data) -> Self {
        Self { ctl, mem }
    }

    /// Adjusts addresses by internal bank number.
    fn adjust(&self, addr: Word) -> usize {
        let bank = {
            if self.ctl.sel.load() == 0 {
                0
            } else {
                usize::from(self.ctl.ram.load())
            }
        };
        let addr = usize::from(addr);
        ((bank << 13) | addr & 0x1fff) % self.mem.len().max(0x2000)
    }
}

impl Memory for Ram {
    fn read(&self, addr: Word) -> Result<Byte> {
        // Error when disabled
        if self.ctl.ena.load() == 0 {
            return Err(Error::Disabled);
        }
        // Translate address
        let index = self.adjust(addr);
        if usize::from(addr) != index {
            trace!("address translation: ${addr:04x} -> ${index:04x}");
        }
        // Perform read
        self.mem.get(index).ok_or(Error::Range).copied()
    }

    fn write(&mut self, addr: Word, data: Byte) -> Result<()> {
        // Error when disabled
        if self.ctl.ena.load() == 0 {
            return Err(Error::Disabled);
        }
        // Translate address
        let index = self.adjust(addr);
        if usize::from(addr) != index {
            trace!("address translation: ${addr:04x} -> ${index:04x}");
        }
        // Perform write
        *self.mem.get_mut(index).ok_or(Error::Range)? = data;

        Ok(())
    }
}
