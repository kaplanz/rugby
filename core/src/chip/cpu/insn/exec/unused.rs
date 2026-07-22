use log::error;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Log the illegal instruction
    error!("illegal instruction: {code:#04X}");
    // Hang the processor
    cpu.step(hang)
}

fn hang(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Hang forever
    cpu.step(hang)
}
