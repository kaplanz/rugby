use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => (),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }

    // Decrement SP
    let sp = cpu.reg.sp.load();
    let sp = cpu.blk.idu.dec(sp);
    cpu.reg.sp.store(sp);

    // Proceed
    cpu.step(cycle3)
}

fn cycle3(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Write [SP] <- upper(PC)
    let sp = cpu.reg.sp.load();
    let pc = cpu.reg.pc.load().to_le_bytes();
    cpu.blk.bus.write(sp, pc[1]);
    // Decrement SP
    let sp = cpu.blk.idu.dec(sp);
    cpu.reg.sp.store(sp);

    // Proceed
    cpu.step(cycle4)
}

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Write [SP] <- lower(PC)
    let sp = cpu.reg.sp.load();
    let pc = cpu.reg.pc.load().to_le_bytes();
    cpu.blk.bus.write(sp, pc[0]);

    // Decode vector
    let v8 = match code {
        0xc7 => 0x00,
        0xcf => 0x08,
        0xd7 => 0x10,
        0xdf => 0x18,
        0xe7 => 0x20,
        0xef => 0x28,
        0xf7 => 0x30,
        0xff => 0x38,
        code => unreachable!("unexpected opcode: {code:#04X}"),
    };
    // Perform jump PC <- vector
    let v16 = u16::from_le_bytes([v8, 0x00]);
    cpu.reg.pc.store(v16);

    // Proceed
    cpu.step(cycle5)
}

fn cycle5(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
