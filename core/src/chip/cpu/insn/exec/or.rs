use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction, help};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0xb6 => {
            // Read Z <- [HL]
            let z = cpu.readbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        0xf6 => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        0xb0..=0xb7 => {
            // Prepare Z
            let z = help::get_op8(cpu, code & 0x07);
            cpu.reg.z.store(z);
            // Continue
            cycle3(code, cpu)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle3(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Execute OR
    let acc = cpu.reg.a.load();
    let op2 = cpu.reg.z.load();
    let (res, f) = cpu.blk.alu.or(acc, op2, cpu.reg.f);
    cpu.reg.a.store(res);
    cpu.reg.f = f;

    // Finish
    None
}
