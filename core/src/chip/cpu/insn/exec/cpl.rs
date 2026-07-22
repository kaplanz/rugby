use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0x2f {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Execute CPL
    let acc = cpu.reg.a.load();
    let res = !acc;
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_n(true);
    cpu.reg.f.set_h(true);

    // Finish
    None
}
