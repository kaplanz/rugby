use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0x1f {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Execute RRA
    let acc = cpu.reg.a.load();
    let cin = cpu.reg.f.c();
    let carry = acc & 0x01 != 0;
    let res = ((cin as u8) << 7) | (acc >> 1);
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(false);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(carry);

    // Finish
    None
}
