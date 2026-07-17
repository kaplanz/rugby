//! Interrupt controller.

use std::fmt::Display;

use log::trace;
use rugby_arch::mem::Memory;
use rugby_arch::mio::{Bus, Mmio};
use rugby_arch::reg::{Port, Register as _};
use rugby_arch::{Block, Shared};

/// Interrupt source.
///
/// |  Source  | Handler |
/// |----------|---------|
/// | VBlank   | `$0040` |
/// | LCD STAT | `$0048` |
/// | Timer    | `$0050` |
/// | Serial   | `$0058` |
/// | Joypad   | `$0060` |
///
/// [sources]: https://gbdev.io/pandocs/Interrupt_Sources.html
#[must_use]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    /// Vertical blank.
    ///
    /// Requested by the [PPU] upon entry to the vertical blank.
    ///
    /// [ppu]:    super::ppu
    /// [vblank]: super::ppu::Mode::VBlank
    VBlank  = 0b0000_0001,
    /// LCD status.
    ///
    /// Requested by the [PPU] as configured by the [STAT] register.
    ///
    /// [ppu]:  super::ppu
    /// [stat]: super::ppu::File::stat
    LcdStat = 0b0000_0010,
    /// Timer overflow.
    ///
    /// Requested by the [timer] whenever the [TIMA] register overflows.
    ///
    /// [tima]:  super::tma::File::tima
    /// [timer]: super::tma
    Timer   = 0b0000_0100,
    /// Serial transfer.
    ///
    /// Requested by the [serial] interface upon completion of a transfer.
    ///
    /// [serial]: super::sio
    Serial  = 0b0000_1000,
    /// Joypad input.
    ///
    /// Requested by the [joypad] whenever any of the [control] bits 0:3
    /// transition from high to low. (Occurs when a [button] in the selected
    /// [mode] is pressed.)
    ///
    /// [button]:  super::joy::Button
    /// [control]: super::joy::Control
    /// [joypad]:  super::joy
    /// [mode]:    super::joy::Mode
    Joypad  = 0b0001_0000,
}

impl Interrupt {
    /// Returns the address of the interrupt handler.
    #[must_use]
    #[rustfmt::skip]
    pub fn handler(self) -> u8 {
        match self {
            Self::VBlank  => 0x40,
            Self::LcdStat => 0x48,
            Self::Timer   => 0x50,
            Self::Serial  => 0x58,
            Self::Joypad  => 0x60,
        }
    }

    /// Returns the interrupt as a vector value.
    #[must_use]
    pub fn vector(self) -> u8 {
        self as u8
    }

    /// Returns a string representation of the interrupt instruction.
    #[must_use]
    #[rustfmt::skip]
    pub fn repr(self) -> &'static str {
        match self {
            Self::VBlank  => "INT 40H",
            Self::LcdStat => "INT 48H",
            Self::Timer   => "INT 50H",
            Self::Serial  => "INT 58H",
            Self::Joypad  => "INT 60H",
        }
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.repr().fmt(f)
    }
}

impl TryFrom<u8> for Interrupt {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value.trailing_zeros() {
            0 => Self::VBlank,
            1 => Self::LcdStat,
            2 => Self::Timer,
            3 => Self::Serial,
            4 => Self::Joypad,
            _ => return Err(Error::Unknown),
        })
    }
}

impl From<Interrupt> for u8 {
    fn from(value: Interrupt) -> Self {
        value.vector()
    }
}

/// Interrupt register select.
///
/// See more details [here][regs].
///
/// [regs]: https://gbdev.io/pandocs/Interrupts.html
#[derive(Copy, Clone, Debug)]
pub enum Select {
    /// `[$FF0F]`: Interrupt flag.
    ///
    /// Determines if a corresponding interrupt is being requested.
    If,
    /// `[$FFFF]`: Interrupt enable.
    ///
    /// Sets whether a corresponding interrupt is can be triggered.
    Ie,
}

/// Programmable interrupt controller.
#[derive(Debug)]
pub struct Irq {
    /// Interrupt registers.
    pub reg: File,
    /// Interrupt line.
    pub line: Line,
}

impl Default for Irq {
    fn default() -> Self {
        let reg = File::default();
        Self {
            line: Line::new(&reg),
            reg,
        }
    }
}

impl Block for Irq {
    fn reset(&mut self) {
        self.reg.reset();
    }
}

impl Mmio for Irq {
    fn attach(&self, bus: &mut Bus) {
        self.reg.attach(bus);
    }
}

impl Port<u8> for Irq {
    type Select = Select;

    fn load(&self, reg: Self::Select) -> u8 {
        match reg {
            Select::If => self.reg.flg.load(),
            Select::Ie => self.reg.ena.load(),
        }
    }

    fn store(&mut self, reg: Self::Select, value: u8) {
        match reg {
            Select::If => self.reg.flg.store(value),
            Select::Ie => self.reg.ena.store(value),
        }
    }
}

/// Control registers.
///
/// | Address | Size | Name | Description      |
/// |:-------:|------|------|------------------|
/// | `$FF0F` | Byte | IF   | Interrupt flag   |
/// | `$FFFF` | Byte | IE   | Interrupt enable |
#[derive(Debug, Default)]
pub struct File {
    /// Interrupt flag.
    pub flg: Shared<Flag>,
    /// Interrupt enable.
    pub ena: Shared<Enable>,
}

impl Block for File {
    fn reset(&mut self) {
        self.flg.take();
        self.ena.take();
    }
}

impl Mmio for File {
    fn attach(&self, bus: &mut Bus) {
        bus.map(0xff0f..=0xff0f, self.flg.clone().into());
        bus.map(0xffff..=0xffff, self.ena.clone().into());
    }
}

/// Interrupt flag register.
///
/// Each [interrupt kind](Interrupt) has a corresponding bit position in a
/// control register as follows:
///
/// | Bit |  Source  |
/// |-----|----------|
/// |  0  | VBlank   |
/// |  1  | LCD STAT |
/// |  2  | Timer    |
/// |  3  | Serial   |
/// |  4  | Joypad   |
#[derive(Debug, Default)]
pub struct Flag(u8);

impl Flag {
    const MASK: u8 = 0b0001_1111;
}

impl Memory for Flag {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl rugby_arch::reg::Register for Flag {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.0.load() | !Self::MASK
    }

    fn store(&mut self, value: Self::Value) {
        self.0.store(value & Self::MASK);
    }
}

/// Interrupt enable register.
///
/// Unlike the [interrupt flag](Flag), all 8 bits are readable and
/// writable. Only the low 5 bits participate in interrupt handling.
pub type Enable = u8;

/// Interrupt line.
#[derive(Clone, Debug)]
pub struct Line {
    /// Interrupt flag.
    flg: Shared<Flag>,
    /// Interrupt enable.
    ena: Shared<Enable>,
}

impl Line {
    /// Constructs a new `Line`.
    fn new(reg: &File) -> Self {
        Self {
            flg: reg.flg.clone(),
            ena: reg.ena.clone(),
        }
    }

    /// Checks if an interrupt is pending.
    #[must_use]
    pub fn pending(&self) -> bool {
        let flg = self.flg.load();
        let ena = self.ena.load();
        (flg & ena & Flag::MASK) != 0
    }

    /// Fetches the first pending interrupt.
    #[must_use]
    pub fn fetch(&self) -> Option<Interrupt> {
        let flg = self.flg.load();
        let ena = self.ena.load();
        (flg & ena)
            .try_into()
            .ok()
            .inspect(|int| trace!("interrupt pending: {int:?}"))
    }

    /// Raises an interrupt.
    pub fn raise(&mut self, int: Interrupt) {
        let flg = self.flg.load() | (int as u8);
        self.flg.store(flg);
        trace!("interrupt requested: {int:?}");
    }

    /// Clears an interrupt.
    pub fn clear(&mut self, int: Interrupt) {
        let flg = self.flg.load() & !(int as u8);
        self.flg.store(flg);
        trace!("interrupt acknowledged: {int:?}");
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by an [interrupt](Interrupt).
#[derive(Debug)]
#[derive(thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Unknown interrupt.
    #[error("unknown interrupt")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn byte_from_interrupt_works() {
        assert_eq!(u8::from(Interrupt::VBlank),  0b0000_0001);
        assert_eq!(u8::from(Interrupt::LcdStat), 0b0000_0010);
        assert_eq!(u8::from(Interrupt::Timer),   0b0000_0100);
        assert_eq!(u8::from(Interrupt::Serial),  0b0000_1000);
        assert_eq!(u8::from(Interrupt::Joypad),  0b0001_0000);
    }

    #[rustfmt::skip]
    #[test]
    fn interrupt_try_from_byte_works() {
        assert!(matches!(Interrupt::try_from(0b0000_0001), Ok(Interrupt::VBlank)));
        assert!(matches!(Interrupt::try_from(0b0000_0010), Ok(Interrupt::LcdStat)));
        assert!(matches!(Interrupt::try_from(0b0000_0100), Ok(Interrupt::Timer)));
        assert!(matches!(Interrupt::try_from(0b0000_1000), Ok(Interrupt::Serial)));
        assert!(matches!(Interrupt::try_from(0b0001_0000), Ok(Interrupt::Joypad)));
    }
}
