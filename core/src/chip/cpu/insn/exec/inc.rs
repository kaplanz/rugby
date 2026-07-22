use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0x34 => {
            // Read Z <- [HL]
            let z = cpu.readbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x3c => {
            // Prepare Z
            let z = match code {
                0x04 => cpu.reg.b.load(),
                0x0c => cpu.reg.c.load(),
                0x14 => cpu.reg.d.load(),
                0x1c => cpu.reg.e.load(),
                0x24 => cpu.reg.h.load(),
                0x2c => cpu.reg.l.load(),
                0x3c => cpu.reg.a.load(),
                code => unreachable!("unexpected opcode: {code:#04X}"),
            };
            cpu.reg.z.store(z);
            // Continue
            cycle3(code, cpu)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Execute INC
    let op1 = cpu.reg.z.load();
    let (res, f) = cpu.blk.alu.inc(op1, cpu.reg.f);
    cpu.reg.f = f;

    // Store result
    match code {
        0x34 => {
            // Write [HL]
            cpu.writebyte(res);
            // Proceed
            return cpu.step(cycle4);
        }
        0x04 => cpu.reg.b.store(res),
        0x0c => cpu.reg.c.store(res),
        0x14 => cpu.reg.d.store(res),
        0x1c => cpu.reg.e.store(res),
        0x24 => cpu.reg.h.store(res),
        0x2c => cpu.reg.l.store(res),
        0x3c => cpu.reg.a.store(res),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }

    // Finish
    None
}

fn cycle4(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
