use log::debug;
use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle1
}

/// Fetches and decodes a prefixed instruction.
///
/// Models the second machine cycle of a `PREFIX`-family instruction:
/// reads the opcode at `PC` and decodes it via the prefix table. No
/// interrupt check occurs, so an interrupt can never dispatch between
/// `$CB` and its opcode byte.
fn cycle1(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Fetch the prefixed opcode
    let pc = cpu.reg.pc.load();
    let op = cpu.fetchbyte();
    // Decode via the prefix table
    let insn = Instruction::prefix(op);

    // Log the instruction
    debug!("${pc:04x}: {insn}");

    // Return the instruction
    Some(insn)
}
