use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0xc5 | 0xd5 | 0xe5 | 0xf5 => (),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }

    // Decrement SP
    let sp = cpu.reg.sp.load();
    let sp = cpu.blk.idu.dec(sp);
    cpu.reg.sp.store(sp);

    // Proceed
    cpu.step(cycle3)
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Load MSB
    let msb = match code {
        0xc5 => &cpu.reg.b,
        0xd5 => &cpu.reg.d,
        0xe5 => &cpu.reg.h,
        0xf5 => &cpu.reg.a,
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
    .load();

    // Write [SP] <- MSB
    let sp = cpu.reg.sp.load();
    cpu.blk.bus.write(sp, msb);
    // Decrement SP
    let sp = cpu.blk.idu.dec(sp);
    cpu.reg.sp.store(sp);

    // Proceed
    cpu.step(cycle4)
}

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Load LSB
    let lsb = match code {
        0xc5 => cpu.reg.c.load(),
        0xd5 => cpu.reg.e.load(),
        0xe5 => cpu.reg.l.load(),
        0xf5 => cpu.reg.f.load(),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    };

    // Write [SP] <- LSB
    let sp = cpu.reg.sp.load();
    cpu.blk.bus.write(sp, lsb);

    // Proceed
    cpu.step(cycle5)
}

fn cycle5(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
