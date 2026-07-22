//! Processor core.
//!
//! Implements the SM83, a custom Sharp processor used in the Game Boy family.
//! It is architecturally similar to the Intel 8080 and Zilog Z80, though it
//! is not binary-compatible with either.

use std::fmt::{Debug, Display};

use rugby_arch::mem::{Memory, Ram};
use rugby_arch::reg::{Port, Register};
use rugby_arch::{Block, Shared};

use self::blk::Hardware;
use self::insn::Instruction;
use crate::chip::irq;

pub mod blk;
pub mod insn;
pub mod reg;

/// High RAM.
///
/// 127 byte RAM only accessible by the CPU.
pub type Hram = Ram<[u8; 0x007f]>;

/// Processor byte select.
///
/// See more details [here][regs].
///
/// [regs]: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
#[derive(Copy, Clone, Debug)]
pub enum Select8 {
    /// Accumulator register.
    A,
    /// Flags register.
    F,
    /// General register B.
    B,
    /// General register C.
    C,
    /// General register D.
    D,
    /// General register E.
    E,
    /// Address (HI) byte.
    H,
    /// Address (LO) byte.
    L,
}

/// Processor word select.
///
/// See more details [here][regs].
///
/// [regs]: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
#[derive(Copy, Clone, Debug)]
pub enum Select16 {
    /// Joint AF register.
    AF,
    /// Joint BC register.
    BC,
    /// Joint DE register.
    DE,
    /// Address register.
    HL,
    /// Stack pointer.
    SP,
    /// Program counter.
    PC,
}

/// Central processing unit.
#[derive(Debug)]
pub struct Cpu {
    /// Hardware blocks.
    pub blk: Hardware,
    /// Processor registers.
    pub reg: File,
    /// Memory bank.
    pub mem: Bank,
    /// Processor internals.
    pub etc: Internal,
    /// Interrupt line.
    pub irq: irq::Line,
}

/// Processor internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// In-flight instruction.
    insn: Instruction,
    /// Mid-instruction marker.
    busy: bool,
    /// Running status.
    run: Status,
    /// Interrupt master enable.
    ime: Ime,
    /// [Halt bug][bug] flag.
    ///
    /// [bug]: https://gbdev.io/pandocs/halt.html#halt-bug
    halt_bug: bool,
}

impl Internal {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl Cpu {
    /// Checks whether the processor is between instructions.
    #[must_use]
    pub fn boundary(&self) -> bool {
        !self.etc.busy
    }

    /// Fetch the next byte after PC.
    fn fetchbyte(&mut self) -> u8 {
        // Load PC
        let mut pc = self.reg.pc.load();
        // Read at PC
        let byte = self.blk.bus.read(pc);
        // Increment PC
        pc = self.blk.idu.inc(pc);
        self.reg.pc.store(pc);
        // Return fetched byte
        byte
    }

    /// Read the byte at HL.
    fn readbyte(&mut self) -> u8 {
        // Load value of HL
        let hl = self.reg.hl().load();
        // Read at HL
        self.blk.bus.read(hl)
    }

    /// Write to the byte at HL
    fn writebyte(&mut self, byte: u8) {
        // Load value of HL
        let hl = self.reg.hl().load();
        // Write to HL
        self.blk.bus.write(hl, byte);
    }

    /// Pop the byte at SP.
    fn popbyte(&mut self) -> u8 {
        // Load SP
        let mut sp = self.reg.sp.load();
        // Read at SP
        let byte = self.blk.bus.read(sp);
        // Increment SP
        sp = self.blk.idu.inc(sp);
        self.reg.sp.store(sp);
        // Return popped byte
        byte
    }

    /// Push to the byte at SP.
    fn pushbyte(&mut self, byte: u8) {
        // Decrement SP
        let mut sp = self.reg.sp.load();
        sp = self.blk.idu.dec(sp);
        self.reg.sp.store(sp);
        // Push to SP
        self.blk.bus.write(sp, byte);
    }
}

impl Block for Cpu {
    fn ready(&self) -> bool {
        self.etc.run == Status::Enabled
    }

    fn cycle(&mut self) {
        insn::cycle(self);
    }

    fn reset(&mut self) {
        self.etc.reset();
        self.reg.reset();
    }
}

impl Port<u8> for Cpu {
    type Select = Select8;

    fn load(&self, reg: Self::Select) -> u8 {
        match reg {
            Select8::A => self.reg.a.load(),
            Select8::F => self.reg.f.load(),
            Select8::B => self.reg.b.load(),
            Select8::C => self.reg.c.load(),
            Select8::D => self.reg.d.load(),
            Select8::E => self.reg.e.load(),
            Select8::H => self.reg.h.load(),
            Select8::L => self.reg.l.load(),
        }
    }

    fn store(&mut self, reg: Self::Select, value: u8) {
        match reg {
            Select8::A => self.reg.a.store(value),
            Select8::F => self.reg.f.store(value),
            Select8::B => self.reg.b.store(value),
            Select8::C => self.reg.c.store(value),
            Select8::D => self.reg.d.store(value),
            Select8::E => self.reg.e.store(value),
            Select8::H => self.reg.h.store(value),
            Select8::L => self.reg.l.store(value),
        }
    }
}

impl Port<u16> for Cpu {
    type Select = Select16;

    fn load(&self, reg: Self::Select) -> u16 {
        match reg {
            Select16::AF => u16::from_le_bytes([self.reg.f.load(), self.reg.a]),
            Select16::BC => self.reg.bc().load(),
            Select16::DE => self.reg.de().load(),
            Select16::HL => self.reg.hl().load(),
            Select16::SP => self.reg.sp.load(),
            Select16::PC => self.reg.pc.load(),
        }
    }

    fn store(&mut self, reg: Self::Select, value: u16) {
        match reg {
            Select16::AF => {
                let [lo, hi] = value.to_le_bytes();
                self.reg.a = hi;
                self.reg.f.store(lo);
            }
            Select16::BC => self.reg.bc_mut().store(value),
            Select16::DE => self.reg.de_mut().store(value),
            Select16::HL => self.reg.hl_mut().store(value),
            Select16::SP => self.reg.sp.store(value),
            Select16::PC => self.reg.pc.store(value),
        }
    }
}

impl Cpu {
    #[must_use]
    pub fn insn(&self) -> Instruction {
        self.etc.insn
    }

    pub fn goto(&mut self, addr: u16) {
        self.reg.pc.store(addr);
    }

    pub fn exec(&mut self, code: u8) {
        // Create a new instruction...
        let mut insn = Some(Instruction::decode(code));
        // ... then execute it until completion
        while let Some(work) = insn {
            insn = work.exec(self);
        }
    }

    pub fn run(&mut self, prog: &[u8]) {
        for &code in prog {
            self.exec(code);
        }
    }

    pub fn wake(&mut self) {
        self.etc.run = Status::Enabled;
    }

    /// Checks if the processor is halted.
    #[must_use]
    pub fn halted(&self) -> bool {
        self.etc.run == Status::Halted
    }

    /// Checks if the processor is stopped.
    #[must_use]
    pub fn stopped(&self) -> bool {
        self.etc.run == Status::Stopped
    }
}

/// Processor memory.
///
/// |     Address     |  Size  | Name | Description   |
/// |:---------------:|--------|------|---------------|
/// | `$FF80..=$FFFE` |  127 B | HRAM | High RAM      |
#[derive(Clone, Debug)]
#[derive(Memory)]
pub struct Bank {
    /// High RAM.
    #[mmap(0xff80..=0xfffe, mask = 0x007f)]
    pub hram: Shared<Hram>,
}

impl Default for Bank {
    fn default() -> Self {
        Self {
            hram: Shared::new(Hram::from([u8::default(); 0x007f])),
        }
    }
}

/// Processor registers.
///
/// | Size | Name | Description                   |
/// |------|------|-------------------------------|
/// | Byte | A    | Accumulator register.         |
/// | Byte | F    | Flags register.               |
/// | Byte | B    | General register B.           |
/// | Byte | C    | General register C.           |
/// | Byte | D    | General register D.           |
/// | Byte | E    | General register E.           |
/// | Byte | H    | Address (HI) byte.            |
/// | Byte | L    | Address (LO) byte.            |
/// | Byte | W    | Private register W.           |
/// | Byte | Z    | Private register Z.           |
/// | Word | SP   | Stack pointer.                |
/// | Word | PC   | Program counter.              |
#[derive(Debug, Default)]
pub struct File {
    /// Accumulator register.
    pub a: reg::A,
    /// Flags register.
    pub f: reg::F,
    /// General register B.
    pub b: reg::B,
    /// General register C.
    pub c: reg::C,
    /// General register D.
    pub d: reg::D,
    /// General register E.
    pub e: reg::E,
    /// Address (HI) byte.
    pub h: reg::H,
    /// Address (LO) byte.
    pub l: reg::L,
    /// Private register W.
    pub w: reg::W,
    /// Private register Z.
    pub z: reg::Z,
    /// Stack pointer.
    pub sp: reg::Sp,
    /// Program counter.
    pub pc: reg::Pc,
}

impl File {
    /// Joint BC register.
    pub(crate) fn bc(&'_ self) -> Alias<'_> {
        Alias {
            hi: &self.b,
            lo: &self.c,
        }
    }

    /// Joint mutable BC register.
    pub(crate) fn bc_mut(&'_ mut self) -> AliasMut<'_> {
        AliasMut {
            hi: &mut self.b,
            lo: &mut self.c,
        }
    }

    /// Joint DE register.
    pub(crate) fn de(&'_ self) -> Alias<'_> {
        Alias {
            hi: &self.d,
            lo: &self.e,
        }
    }

    /// Joint mutable DE register.
    pub(crate) fn de_mut(&'_ mut self) -> AliasMut<'_> {
        AliasMut {
            hi: &mut self.d,
            lo: &mut self.e,
        }
    }

    /// Address register.
    pub(crate) fn hl(&'_ self) -> Alias<'_> {
        Alias {
            hi: &self.h,
            lo: &self.l,
        }
    }

    /// Mutable address register.
    pub(crate) fn hl_mut(&'_ mut self) -> AliasMut<'_> {
        AliasMut {
            hi: &mut self.h,
            lo: &mut self.l,
        }
    }

    /// Joint WZ register.
    pub(crate) fn wz(&'_ self) -> Alias<'_> {
        Alias {
            hi: &self.w,
            lo: &self.z,
        }
    }

    /// Joint mutable WZ register.
    pub(crate) fn wz_mut(&'_ mut self) -> AliasMut<'_> {
        AliasMut {
            hi: &mut self.w,
            lo: &mut self.z,
        }
    }
}

impl Block for File {
    fn reset(&mut self) {
        std::mem::take(&mut self.pc);
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌───┬────┬───┬────┐")?;
        writeln!(
            f,
            "│ A │ {:02x} │ F │ {:02x} │",
            self.a.load(),
            self.f.load()
        )?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(
            f,
            "│ B │ {:02x} │ C │ {:02x} │",
            self.b.load(),
            self.c.load()
        )?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(
            f,
            "│ D │ {:02x} │ E │ {:02x} │",
            self.d.load(),
            self.e.load()
        )?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(
            f,
            "│ H │ {:02x} │ L │ {:02x} │",
            self.h.load(),
            self.l.load()
        )?;
        writeln!(f, "├───┴────┼───┴────┤")?;
        writeln!(f, "│   SP   │  {:04x}  │", self.sp.load())?;
        writeln!(f, "├────────┼────────┤")?;
        writeln!(f, "│   PC   │  {:04x}  │", self.pc.load())?;
        write!(f, "└────────┴────────┘")
    }
}

/// Aliased register.
#[derive(Copy, Clone)]
pub(crate) struct Alias<'a> {
    pub lo: &'a u8,
    pub hi: &'a u8,
}

impl Alias<'_> {
    pub fn load(&self) -> u16 {
        let value = [*self.lo, *self.hi];
        u16::from_le_bytes(value)
    }
}

impl Debug for Alias<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.load(), f)
    }
}

impl Display for Alias<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.load(), f)
    }
}

/// Aliased register.
pub(crate) struct AliasMut<'a> {
    pub lo: &'a mut u8,
    pub hi: &'a mut u8,
}

impl Register for AliasMut<'_> {
    type Value = u16;

    fn load(&self) -> Self::Value {
        let value = [*self.lo, *self.hi];
        u16::from_le_bytes(value)
    }

    fn store(&mut self, value: Self::Value) {
        let [lo, hi] = value.to_le_bytes();
        *self.lo = lo;
        *self.hi = hi;
    }
}

impl Debug for AliasMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.load(), f)
    }
}

impl Display for AliasMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.load(), f)
    }
}

/// Processor running status.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Status {
    /// Enabled; normal execution.
    #[default]
    Enabled,
    /// Halted; awaiting interrupt.
    Halted,
    /// Stopped; very low-power.
    Stopped,
}

/// Interrupt master enable.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum Ime {
    /// Prevent interrupts.
    #[default]
    Disabled,
    /// Allow interrupts.
    Enabled,
    /// Pending allow.
    WillEnable,
}

impl Ime {
    fn enabled(self) -> bool {
        self == Self::Enabled
    }
}
