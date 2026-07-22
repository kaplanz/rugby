use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0x3f {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Execute CCF
    let c = cpu.reg.f.c();
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(!c);

    // Finish
    None
}
