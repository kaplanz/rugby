use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0xc2 | 0xc3 | 0xca | 0xd2 | 0xda => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        0xe9 => {
            // Perform jump PC <- HL
            let hl = cpu.reg.hl().load();
            cpu.reg.pc.store(hl);
            // Finish
            None
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Fetch W <- [PC++]
    let w = cpu.fetchbyte();
    cpu.reg.w.store(w);

    // Evaluate condition
    #[rustfmt::skip]
    let cond = match code {
        0xc2 => !cpu.reg.f.z(),
        0xca =>  cpu.reg.f.z(),
        0xd2 => !cpu.reg.f.c(),
        0xda =>  cpu.reg.f.c(),
        0xc3 => true,
        code => unreachable!("unexpected opcode: {code:#04X}"),
    };

    // Check condition
    if cond {
        // Proceed
        cpu.step(cycle4)
    } else {
        // Proceed
        cpu.step(cycle5)
    }
}

fn cycle4(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Perform jump PC <- WZ
    let wz = cpu.reg.wz().load();
    cpu.reg.pc.store(wz);

    // Proceed
    cpu.step(cycle5)
}

fn cycle5(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle (untaken jumps enter early)

    // Finish
    None
}
