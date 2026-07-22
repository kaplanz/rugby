//! Instruction fetch implementations.
//!
//! Models the SM83's M1 machine cycle: the interrupt check, the opcode
//! read, and the table decode. Every instruction's function chain ends
//! by returning [`None`], letting the machine run [`cycle1`] within the
//! same cycle to overlap the fetch with any `Mx/M1` work.

#![allow(clippy::unnecessary_wraps)]

use log::{debug, trace};
use rugby_arch::Block;
use rugby_arch::reg::Register;

use super::{Cpu, Ime, Instruction, exec};

/// Fetches and decodes the next instruction.
///
/// Checks for pending interrupts, reads the opcode at `PC`, and decodes
/// it, installing the decoded instruction's chain.
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
        // Install the dispatch
        return install(insn, cpu);
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

    // Install the instruction
    install(insn, cpu)
}

/// Fetches and decodes a prefixed instruction.
///
/// Models the second machine cycle of a `PREFIX`-family instruction:
/// reads the opcode at `PC` and decodes it via the prefix table. No
/// interrupt check occurs, so an interrupt can never dispatch between
/// `$CB` and its opcode byte.
pub(super) fn prefix(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Fetch the prefixed opcode
    let pc = cpu.reg.pc.load();
    let op = cpu.fetchbyte();
    // Decode via the prefix table
    let insn = Instruction::prefix(op);

    // Log the instruction
    debug!("${pc:04x}: {insn}");

    // Install the instruction
    install(insn, cpu)
}

/// Installs a decoded instruction's chain.
fn install(insn: Instruction, cpu: &mut Cpu) -> Option<Instruction> {
    let Instruction { code, legacy, .. } = insn;
    cpu.etc.insn = insn;
    if legacy.is_some() {
        // Release the blocks for the co-hosted stage
        cpu.blk.cycle();
        // Legacy entry: co-host the first stage (transitional)
        match exec::legacy(code, cpu) {
            // Proceed to the next stage
            Some(next) => Some(next),
            // Completed within the fetch cycle
            None => cpu.step(cycle1),
        }
    } else {
        // Chained entry: the first work cycle runs next
        Some(insn)
    }
}
