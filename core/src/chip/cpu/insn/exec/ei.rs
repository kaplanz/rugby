use super::{Cpu, Exec, Ime, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0xfb {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Execute EI
    if let Ime::Disabled = cpu.etc.ime {
        cpu.etc.ime = Ime::WillEnable;
    }

    // Finish
    None
}
