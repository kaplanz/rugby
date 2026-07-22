use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Fetch Z <- [PC++]
    let z = cpu.fetchbyte();
    cpu.reg.z.store(z);

    // Evaluate condition
    #[rustfmt::skip]
    let cond = match code {
        0x20 => !cpu.reg.f.z(),
        0x28 =>  cpu.reg.f.z(),
        0x30 => !cpu.reg.f.c(),
        0x38 =>  cpu.reg.f.c(),
        0x18 => true,
        code => unreachable!("unexpected opcode: {code:#04X}"),
    };

    // Check condition
    if cond {
        // Proceed
        cpu.step(cycle3)
    } else {
        // Proceed
        cpu.step(cycle5)
    }
}

fn cycle3(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Load operands
    let pc = cpu.reg.pc.load().to_le_bytes();
    let e8 = cpu.reg.z.load();
    let sign = e8 & 0x80 != 0;

    // Execute JR (LSB)
    let (res, carry) = pc[0].overflowing_add(e8);
    cpu.reg.z.store(res);

    // Adjust MSB with the carry
    #[rustfmt::skip]
    let adj = match (carry, sign) {
        (true, false) => 1u8,
        (false, true) => 0xff,
        _             => 0,
    };
    cpu.reg.w.store(pc[1].wrapping_add(adj));

    // Proceed
    cpu.step(cycle4)
}

fn cycle4(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Perform jump PC <- WZ
    let wz = cpu.reg.wz().load();
    cpu.reg.pc.store(wz);

    // Finish
    None
}

fn cycle5(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle (untaken jumps only)

    // Finish
    None
}
