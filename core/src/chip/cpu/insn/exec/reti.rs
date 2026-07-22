use rugby_arch::reg::Register;

use super::{Cpu, Exec, Ime, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0xd9 {
        unreachable!("unexpected opcode: {code:#04X}");
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

fn cycle4(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Perform jump PC <- WZ
    let wz = cpu.reg.wz().load();
    cpu.reg.pc.store(wz);

    // Enable interrupts.
    //
    // RETI enables IME at the end of the instruction itself, unlike EI which
    // defers by one instruction. To disable interrupts in RETI we use
    // `Enabled` to ensure a pending interrupt isn't missed for a cycle.
    cpu.etc.ime = Ime::Enabled;

    // Proceed
    cpu.step(cycle5)
}

fn cycle5(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
