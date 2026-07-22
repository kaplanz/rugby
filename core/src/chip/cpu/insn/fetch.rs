//! Instruction fetch implementations.
//!
//! Models the SM83's M1 machine cycle: the interrupt check, the opcode
//! read, and the table decode. Every instruction's function chain ends
//! by returning [`None`], letting the machine run [`cycle1`] within the
//! same cycle to overlap the fetch with any `Mx/M1` work.

#![allow(clippy::unnecessary_wraps)]

use log::{debug, trace};
use rugby_arch::reg::Register;

use super::{Cpu, Ime, Instruction};

/// Fetches and decodes the next instruction.
///
/// Checks for pending interrupts, reads the opcode at `PC`, and decodes
/// it, returning the decoded instruction's chain.
pub fn cycle1(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Log previous register state
    trace!("registers:\n{}", cpu.reg);

    // Check for pending interrupts
    let int = (cpu.etc.ime == Ime::Enabled)
        .then(|| cpu.irq.fetch())
        .flatten();
    if let Some(int) = int {
        // Service the interrupt
        let insn = Instruction::vector(int);
        debug!("${pc:04x}: {insn}", pc = int.handler());
        // Disable interrupts
        cpu.etc.ime = Ime::Disabled;
        // Return the dispatch
        return Some(insn);
    }

    // Fetch the next opcode
    let pc = cpu.reg.pc.load();
    let op = cpu.fetchbyte();
    // Decode the instruction
    let insn = Instruction::decode(op);

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

    // Return the instruction
    Some(insn)
}
