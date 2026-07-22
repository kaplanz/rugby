use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0xc4 | 0xcc | 0xcd | 0xd4 | 0xdc => (),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }

    // Fetch Z <- [PC++]
    let z = cpu.fetchbyte();
    cpu.reg.z.store(z);

    // Proceed
    cpu.step(cycle3)
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Fetch W <- [PC++]
    let w = cpu.fetchbyte();
    cpu.reg.w.store(w);

    // Evaluate condition
    #[rustfmt::skip]
    let cond = match code {
        0xc4 => !cpu.reg.f.z(),
        0xcc =>  cpu.reg.f.z(),
        0xd4 => !cpu.reg.f.c(),
        0xdc =>  cpu.reg.f.c(),
        0xcd => true,
        code => unreachable!("unexpected opcode: {code:#04X}"),
    };

    // Check condition
    if cond {
        // Proceed
        cpu.step(cycle4)
    } else {
        // Proceed
        cpu.step(cycle7)
    }
}

fn cycle4(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Decrement SP
    let sp = cpu.reg.sp.load();
    let sp = cpu.blk.idu.dec(sp);
    cpu.reg.sp.store(sp);

    // Proceed
    cpu.step(cycle5)
}

fn cycle5(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Write [SP] <- upper(PC)
    let sp = cpu.reg.sp.load();
    let pc = cpu.reg.pc.load().to_le_bytes();
    cpu.blk.bus.write(sp, pc[1]);
    // Decrement SP
    let sp = cpu.blk.idu.dec(sp);
    cpu.reg.sp.store(sp);

    // Proceed
    cpu.step(cycle6)
}

fn cycle6(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Write [SP] <- lower(PC)
    let sp = cpu.reg.sp.load();
    let pc = cpu.reg.pc.load().to_le_bytes();
    cpu.blk.bus.write(sp, pc[0]);
    // Perform jump PC <- WZ
    let wz = cpu.reg.wz().load();
    cpu.reg.pc.store(wz);

    // Proceed
    cpu.step(cycle7)
}

fn cycle7(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle (untaken calls enter early)

    // Finish
    None
}
