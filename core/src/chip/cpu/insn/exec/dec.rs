use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0x35 => {
            // Read Z <- [HL]
            let z = cpu.readbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x3d => {
            // Prepare Z
            let z = match code {
                0x05 => cpu.reg.b.load(),
                0x0d => cpu.reg.c.load(),
                0x15 => cpu.reg.d.load(),
                0x1d => cpu.reg.e.load(),
                0x25 => cpu.reg.h.load(),
                0x2d => cpu.reg.l.load(),
                0x3d => cpu.reg.a.load(),
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
    // Execute DEC
    let op1 = cpu.reg.z.load();
    let (res, f) = cpu.blk.alu.dec(op1, cpu.reg.f);
    cpu.reg.f = f;

    // Store result
    match code {
        0x35 => {
            // Write [HL]
            cpu.writebyte(res);
            // Proceed
            return cpu.step(cycle4);
        }
        0x05 => cpu.reg.b.store(res),
        0x0d => cpu.reg.c.store(res),
        0x15 => cpu.reg.d.store(res),
        0x1d => cpu.reg.e.store(res),
        0x25 => cpu.reg.h.store(res),
        0x2d => cpu.reg.l.store(res),
        0x3d => cpu.reg.a.store(res),
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
