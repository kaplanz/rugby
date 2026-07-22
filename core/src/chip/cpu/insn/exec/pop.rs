use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0xc1 | 0xd1 | 0xe1 | 0xf1 => (),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }

    // Pop Z <- [SP++]
    let z = cpu.popbyte();
    cpu.reg.z.store(z);

    // Proceed
    cpu.step(cycle3)
}

fn cycle3(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Pop W <- [SP++]
    let w = cpu.popbyte();
    cpu.reg.w.store(w);

    // Proceed
    cpu.step(cycle4)
}

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Store r16 <- WZ
    let w = cpu.reg.w.load();
    let z = cpu.reg.z.load();
    match code {
        0xc1 => {
            cpu.reg.b.store(w);
            cpu.reg.c.store(z);
        }
        0xd1 => {
            cpu.reg.d.store(w);
            cpu.reg.e.store(z);
        }
        0xe1 => {
            cpu.reg.h.store(w);
            cpu.reg.l.store(z);
        }
        0xf1 => {
            cpu.reg.a.store(w);
            cpu.reg.f.store(z);
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }

    // Finish
    None
}
