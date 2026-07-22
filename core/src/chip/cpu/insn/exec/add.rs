use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction, help};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0x86 => {
            // Read Z <- [HL]
            let z = cpu.readbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        0xc6 => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        0x80..=0x87 => {
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
    // Execute ADD
    let acc = cpu.reg.a.load();
    let op2 = cpu.reg.z.load();
    let (res, f) = cpu.blk.alu.add(acc, op2, cpu.reg.f);
    cpu.reg.a.store(res);
    cpu.reg.f = f;

    // Finish
    None
}
