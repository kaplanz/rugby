//! Instruction set architecture.

use std::fmt::{Debug, Display};

use log::trace;
use rugby_arch::Block;

use self::exec::Exec;
use super::{Cpu, Ime, Status};
use crate::chip::irq::Interrupt;

mod exec;
mod fetch;
mod table;

/// Executes a single M-cycle of the in-flight instruction.
///
/// # Panics
///
/// Cannot panic.
pub fn cycle(cpu: &mut Cpu) {
    trace!("{:?}", cpu.etc.insn);

    // Trigger a fresh M-cycle
    cpu.blk.cycle();

    // Execute the in-flight stage
    let Instruction { code, exec, .. } = cpu.etc.insn;
    cpu.etc.insn = if let Some(next) = exec(code, cpu) {
        // Proceed with in-flight instruction
        cpu.etc.busy = true;
        next
    } else {
        // Concurrently fetch next instruction
        cpu.etc.busy = false;
        fetch::cycle1(code, cpu).expect("fetch will always succeed")
    }
}

impl Cpu {
    /// Proceeds to the given stage of the in-flight instruction.
    #[allow(clippy::unnecessary_wraps)]
    fn step(&self, exec: Exec) -> Option<Instruction> {
        Some(Instruction {
            exec,
            ..self.etc.insn
        })
    }
}

/// Processor instruction.
///
/// Stores the currently executing instruction together with its state. Can be
/// started and resumed.
#[derive(Clone, Copy)]
pub struct Instruction {
    code: u8,
    exec: Exec,
    repr: &'static str,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            code: 0x00,
            exec: fetch::cycle1,
            repr: "FETCH",
        }
    }
}

impl Instruction {
    /// Constructs a new `Instruction` with the given opcode.
    #[must_use]
    pub fn decode(code: u8) -> Self {
        table::DECODE[code as usize]
    }

    /// Constructs a new prefix `Instruction` with the given opcode.
    #[must_use]
    pub fn prefix(code: u8) -> Self {
        table::PREFIX[code as usize]
    }

    /// Constructs a new interrupt `Instruction`.
    #[must_use]
    pub fn vector(int: Interrupt) -> Self {
        Self {
            code: int as u8,
            exec: exec::int::default(),
            repr: int.repr(),
        }
    }

    /// Gets the instruction's opcode.
    #[must_use]
    pub fn opcode(&self) -> u8 {
        self.code
    }

    /// Executes a single stage of the instruction.
    pub fn exec(self, cpu: &mut Cpu) -> Option<Self> {
        // Install the instruction
        cpu.etc.insn = self;
        // Begin M-cycle
        cpu.blk.cycle();
        // Execute operation
        trace!("{self:?}");
        // Return the next stage
        (self.exec)(self.code, cpu)
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("code", &format_args!("{:02X?}", self.code))
            .field("repr", &self.repr)
            .field("exec", &format_args!("{:02X?}", self.exec))
            .finish_non_exhaustive()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl From<Interrupt> for Instruction {
    fn from(value: Interrupt) -> Self {
        Self {
            code: value as u8,
            exec: exec::int::default(),
            repr: value.repr(),
        }
    }
}

/// Helper functions.
mod help {
    use rugby_arch::reg::Register;

    use super::Cpu;

    /// Get an 8-bit register operand.
    pub fn get_op8(cpu: &mut Cpu, idx: u8) -> u8 {
        match idx {
            0x0 => cpu.reg.b.load(),
            0x1 => cpu.reg.c.load(),
            0x2 => cpu.reg.d.load(),
            0x3 => cpu.reg.e.load(),
            0x4 => cpu.reg.h.load(),
            0x5 => cpu.reg.l.load(),
            0x6 => cpu.readbyte(),
            0x7 => cpu.reg.a.load(),
            _ => panic!("Illegal register."),
        }
    }

    /// Set an 8-bit register operand.
    pub fn set_op8(cpu: &mut Cpu, idx: u8, value: u8) {
        match idx {
            0x0 => cpu.reg.b.store(value),
            0x1 => cpu.reg.c.store(value),
            0x2 => cpu.reg.d.store(value),
            0x3 => cpu.reg.e.store(value),
            0x4 => cpu.reg.h.store(value),
            0x5 => cpu.reg.l.store(value),
            0x6 => cpu.writebyte(value),
            0x7 => cpu.reg.a.store(value),
            _ => panic!("Illegal register."),
        }
    }
}

#[cfg(test)]
mod tests;
