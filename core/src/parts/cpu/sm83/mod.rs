//! SM83 processor core.

use std::fmt::{Debug, Display};

use log::{debug, error, trace, warn};
use rugby_arch::mem::{Memory, Ram};
use rugby_arch::mio::Bus;
use rugby_arch::reg::{Port, Register};
use rugby_arch::{Block, Shared};

use self::insn::Instruction;
use crate::api::part::proc::Processor;
use crate::dmg::mem::Sram;
use crate::dmg::pic;

pub mod insn;

/// Work RAM.
///
/// 8 KiB RAM used as general-purpose transient memory.
pub type Wram = Sram;

/// High RAM.
///
/// 127 byte RAM only accessible by the CPU.
pub type Hram = Ram<[u8; 0x007f]>;

/// Processor byte select.
///
/// See more details [here][regs].
///
/// [regs]: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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
    /// Processor bus.
    pub bus: Bus,
    /// Processor registers.
    pub reg: Control,
    /// Processor memory.
    pub mem: Bank,
    /// Processor internals.
    pub etc: Internal,
    /// Interrupt line.
    pub int: pic::Line,
}

/// Processor internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// Prefix instruction.
    prefix: bool,
    /// Execution stage.
    stage: Stage,
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
    /// Gets the CPU's execution stage.
    #[must_use]
    pub fn stage(&self) -> &Stage {
        &self.etc.stage
    }

    /// Read the byte at the given address.
    #[must_use]
    pub fn read(&self, addr: u16) -> u8 {
        self.bus
            .read(addr)
            .inspect_err(|err| warn!("failed to read [${addr:04x}] (default: `0xff`): {err}"))
            .unwrap_or(0xff)
    }

    /// Write to the byte at the given address.
    pub fn write(&mut self, addr: u16, data: u8) {
        let _ = self
            .bus
            .write(addr, data)
            .inspect_err(|err| warn!("failed to write [${addr:04x}] <- {data:#04x}: {err}"));
    }

    /// Fetch the next byte after PC.
    fn fetchbyte(&mut self) -> u8 {
        // Load PC
        let mut pc = self.reg.pc.load();
        // Read at PC
        let byte = self.read(pc);
        // Increment PC
        pc = pc.wrapping_add(1);
        self.reg.pc.store(pc);
        // Return fetched byte
        byte
    }

    /// Read the byte at HL.
    fn readbyte(&mut self) -> u8 {
        // Load value of HL
        let hl = self.reg.hl().load();
        // Read at HL
        self.read(hl)
    }

    /// Write to the byte at HL
    fn writebyte(&mut self, byte: u8) {
        // Load value of HL
        let hl = self.reg.hl().load();
        // Write to HL
        self.write(hl, byte);
    }

    /// Pop the byte at SP.
    fn popbyte(&mut self) -> u8 {
        // Load SP
        let mut sp = self.reg.sp.load();
        // Read at SP
        let byte = self.read(sp);
        // Increment SP
        sp = sp.wrapping_add(1);
        self.reg.sp.store(sp);
        // Return popped byte
        byte
    }

    /// Push to the byte at SP.
    fn pushbyte(&mut self, byte: u8) {
        // Increment SP
        let mut sp = self.reg.sp.load();
        sp = sp.wrapping_sub(1);
        self.reg.sp.store(sp);
        // Push to SP
        self.write(sp, byte);
    }
}

impl Block for Cpu {
    fn ready(&self) -> bool {
        self.etc.run == Status::Enabled
    }

    fn cycle(&mut self) {
        self.etc.stage = std::mem::take(&mut self.etc.stage).exec(self);
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
            Select16::AF => self.reg.af().load(),
            Select16::BC => self.reg.bc().load(),
            Select16::DE => self.reg.de().load(),
            Select16::HL => self.reg.hl().load(),
            Select16::SP => self.reg.sp.load(),
            Select16::PC => self.reg.pc.load(),
        }
    }

    fn store(&mut self, reg: Self::Select, value: u16) {
        match reg {
            Select16::AF => self.reg.af_mut().store(value),
            Select16::BC => self.reg.bc_mut().store(value),
            Select16::DE => self.reg.de_mut().store(value),
            Select16::HL => self.reg.hl_mut().store(value),
            Select16::SP => self.reg.sp.store(value),
            Select16::PC => self.reg.pc.store(value),
        }
    }
}

impl Processor for Cpu {
    type Insn = Instruction;

    fn insn(&self) -> Self::Insn {
        if let Stage::Execute(insn) = &self.etc.stage {
            insn.clone()
        } else {
            // Fetch opcode at PC
            let pc = self.reg.pc.load();
            let op = self.read(pc);
            // Construct instruction
            Instruction::decode(op)
        }
    }

    fn goto(&mut self, addr: u16) {
        self.reg.pc.store(addr);
    }

    fn exec(&mut self, code: u8) {
        // Create a new instruction...
        let mut insn = Ok(Some(Instruction::decode(code)));
        // ... then execute it until completion
        while let Ok(Some(work)) = insn {
            insn = work.exec(self);
        }
        // Report any errors
        if let Err(err) = insn {
            error!("{err}");
        }
    }

    fn run(&mut self, prog: &[u8]) {
        for &code in prog {
            self.exec(code);
        }
    }

    fn wake(&mut self) {
        self.etc.run = Status::Enabled;
    }
}

/// Processor memory.
///
/// |     Address     |  Size  | Name | Description   |
/// |:---------------:|--------|------|---------------|
/// | `$C000..=$DFFF` |  8 KiB | WRAM | Work RAM      |
/// | `$FF80..=$FFFE` |  127 B | HRAM | High RAM      |
#[derive(Clone, Debug)]
pub struct Bank {
    /// Work RAM.
    pub wram: Shared<Wram>,
    /// High RAM.
    pub hram: Shared<Hram>,
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
/// | Word | SP   | Stack pointer.                |
/// | Word | PC   | Program counter.              |
#[derive(Debug, Default)]
pub struct Control {
    /// Accumulator register.
    pub a: u8,
    /// Flags register.
    pub f: u8,
    /// General register B.
    pub b: u8,
    /// General register C.
    pub c: u8,
    /// General register D.
    pub d: u8,
    /// General register E.
    pub e: u8,
    /// Address (HI) byte.
    pub h: u8,
    /// Address (LO) byte.
    pub l: u8,
    /// Stack pointer.
    pub sp: u16,
    /// Program counter.
    pub pc: u16,
}

impl Control {
    /// Joint AF register.
    pub(crate) fn af(&self) -> Alias {
        Alias {
            hi: &self.a,
            lo: &self.f,
        }
    }

    /// Joint mutable AF register.
    pub(crate) fn af_mut(&mut self) -> AliasMut {
        AliasMut {
            hi: &mut self.a,
            lo: &mut self.f,
        }
    }

    /// Joint BC register.
    pub(crate) fn bc(&self) -> Alias {
        Alias {
            hi: &self.b,
            lo: &self.c,
        }
    }

    /// Joint mutable BC register.
    pub(crate) fn bc_mut(&mut self) -> AliasMut {
        AliasMut {
            hi: &mut self.b,
            lo: &mut self.c,
        }
    }

    /// Joint DE register.
    pub(crate) fn de(&self) -> Alias {
        Alias {
            hi: &self.d,
            lo: &self.e,
        }
    }

    /// Joint mutable DE register.
    pub(crate) fn de_mut(&mut self) -> AliasMut {
        AliasMut {
            hi: &mut self.d,
            lo: &mut self.e,
        }
    }

    /// Address register.
    pub(crate) fn hl(&self) -> Alias {
        Alias {
            hi: &self.h,
            lo: &self.l,
        }
    }

    /// Mutable address register.
    pub(crate) fn hl_mut(&mut self) -> AliasMut {
        AliasMut {
            hi: &mut self.h,
            lo: &mut self.l,
        }
    }
}

impl Block for Control {
    fn reset(&mut self) {
        std::mem::take(&mut self.pc);
    }
}

impl Display for Control {
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
#[derive(Clone, Copy)]
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

/// Processor flags.
///
/// | Bit | Name | Explanation         |
/// |-----|------|---------------------|
/// |   7 | Z    | Zero flag.          |
/// |   6 | N    | Subtraction flag.   |
/// |   5 | H    | Half-carry flag.    |
/// |   4 | C    | Carry flag.         |
///
/// See more details [here][flag].
///
/// [flag]: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html#the-flags-register-lower-8-bits-of-af-register
#[derive(Clone, Copy, Debug)]
pub enum Flag {
    /// Zero flag.
    Z = 0b1000_0000,
    /// Subtraction flag.
    N = 0b0100_0000,
    /// Half-carry flag.
    H = 0b0010_0000,
    /// Carry flag.
    C = 0b0001_0000,
}

impl Flag {
    /// Gets the value of the corresponding bit to the flag.
    #[must_use]
    pub fn get(self, value: &u8) -> bool {
        value & self as u8 != 0
    }

    /// Sets the value of the corresponding bit from the flag.
    pub fn set(self, value: &mut u8, enable: bool) {
        *value ^= (*value & self as u8) ^ (!u8::from(enable).wrapping_sub(1) & self as u8);
    }
}

/// Processor running status.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Status {
    /// Enabled; normal execution.
    #[default]
    Enabled,
    /// Halted; awaiting interrupt.
    Halted,
    /// Stopped; very low-power.
    Stopped,
}

/// Processor execution stage.
#[derive(Clone, Debug, Default)]
pub enum Stage {
    /// Fetch and decode.
    #[default]
    Fetch,
    /// Execute instruction.
    Execute(Instruction),
    /// Completed execution.
    Done,
}

impl Stage {
    fn exec(mut self, cpu: &mut Cpu) -> Self {
        // If done, proceed to fetch this cycle
        if let Stage::Done = self {
            // Log previous register stage
            trace!("registers:\n{}", cpu.reg);

            // Check for pending interrupts
            let int = (cpu.etc.ime == Ime::Enabled)
                .then(|| cpu.int.fetch())
                .flatten();

            // Handle pending interrupt...
            if let Some(int) = int {
                // Acknowledge the interrupt
                cpu.int.clear(int);
                // Skip `Stage::Fetch`
                let insn = Instruction::int(int);
                debug!("${pc:04x}: {insn}", pc = int.handler());
                self = Stage::Execute(insn);
            }
            // ... or fetch next instruction
            else {
                // Proceed to `Stage::Fetch`
                self = Stage::Fetch;
            }
        }

        // If fetch, proceed to execute this cycle
        if let Stage::Fetch = self {
            // Read the next instruction
            let pc = cpu.reg.pc.load();
            let op = cpu.fetchbyte();

            // Decode the instruction
            let insn = if std::mem::take(&mut cpu.etc.prefix) {
                Instruction::prefix
            } else {
                Instruction::decode
            }(op);

            // Check for HALT bug
            if cpu.etc.halt_bug {
                // Service the bug by rolling back the PC
                let mut pc = cpu.reg.pc.load();
                pc = pc.wrapping_sub(1);
                cpu.reg.pc.store(pc);
                cpu.etc.halt_bug = false;
            }

            // Log the instruction
            debug!("${pc:04x}: {insn}");

            // Enable interrupts (after EI, RETI)
            if let Ime::WillEnable = cpu.etc.ime {
                cpu.etc.ime = Ime::Enabled;
            }

            // Proceed to `Stage::Execute(_)`
            self = Stage::Execute(insn);
        }

        // Execute the current stage
        if let Stage::Execute(insn) = self {
            // Execute a cycle of the instruction
            let insn = insn.exec(cpu);
            // Proceed to next stage
            self = match insn {
                Ok(Some(insn)) => Stage::Execute(insn),
                Ok(None) => {
                    if cpu.etc.prefix {
                        // Atomically handle prefix instructions
                        Stage::Fetch
                    } else {
                        // Otherwise, conclude the instruction
                        Stage::Done
                    }
                }
                Err(err) => {
                    // Log the error
                    error!("{err}");
                    // Stop the CPU
                    cpu.etc.run = Status::Stopped;
                    // Continue to next instruction
                    Stage::Done
                }
            };
        }

        self
    }
}

/// Interrupt master enable.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
