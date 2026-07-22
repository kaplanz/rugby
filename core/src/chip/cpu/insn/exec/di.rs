use super::{Cpu, Exec, Ime, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0xf3 {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Execute DI
    cpu.etc.ime = Ime::Disabled;

    // Finish
    None
}
