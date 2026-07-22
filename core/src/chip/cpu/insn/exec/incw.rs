use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Load r16
    let op1 = match code {
        0x03 => cpu.reg.bc().load(),
        0x13 => cpu.reg.de().load(),
        0x23 => cpu.reg.hl().load(),
        0x33 => cpu.reg.sp.load(),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    };

    // Execute INCW
    let res = cpu.blk.idu.inc(op1);
    match code {
        0x03 => cpu.reg.bc_mut().store(res),
        0x13 => cpu.reg.de_mut().store(res),
        0x23 => cpu.reg.hl_mut().store(res),
        0x33 => cpu.reg.sp.store(res),
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }

    // Proceed
    cpu.step(cycle3)
}

fn cycle3(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
