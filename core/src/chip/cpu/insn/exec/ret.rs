use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // RET
        0xc9 => {
            // Pop Z <- [SP++]
            let z = cpu.popbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        // RET cc
        0xc0 | 0xc8 | 0xd0 | 0xd8 => {
            // Evaluate condition
            #[rustfmt::skip]
            let cond = match code {
                0xc0 => !cpu.reg.f.z(),
                0xc8 =>  cpu.reg.f.z(),
                0xd0 => !cpu.reg.f.c(),
                0xd8 =>  cpu.reg.f.c(),
                code => unreachable!("unexpected opcode: {code:#04X}"),
            };
            // Check condition
            if cond {
                // Proceed
                cpu.step(cycle3)
            } else {
                // Proceed
                cpu.step(cycle6)
            }
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code == 0xc9 {
        // Pop W <- [SP++]
        let w = cpu.popbyte();
        cpu.reg.w.store(w);
    } else {
        // Pop Z <- [SP++]
        let z = cpu.popbyte();
        cpu.reg.z.store(z);
    }

    // Proceed
    cpu.step(cycle4)
}

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code == 0xc9 {
        // Perform jump PC <- WZ
        let wz = cpu.reg.wz().load();
        cpu.reg.pc.store(wz);
        // Proceed
        cpu.step(cycle6)
    } else {
        // Pop W <- [SP++]
        let w = cpu.popbyte();
        cpu.reg.w.store(w);
        // Proceed
        cpu.step(cycle5)
    }
}

fn cycle5(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Perform jump PC <- WZ
    let wz = cpu.reg.wz().load();
    cpu.reg.pc.store(wz);

    // Proceed
    cpu.step(cycle6)
}

fn cycle6(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle (RET and untaken returns enter early)

    // Finish
    None
}
