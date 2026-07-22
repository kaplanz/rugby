use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, _: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0x00 {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Finish
    None
}
