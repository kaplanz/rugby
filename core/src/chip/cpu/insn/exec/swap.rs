use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction, help};

pub const fn default() -> Exec {
    cycle3
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0x36 => {
            // Read Z <- [HL]
            let z = cpu.readbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle4)
        }
        0x30..=0x37 => {
            // Prepare Z
            let z = help::get_op8(cpu, code & 0x07);
            cpu.reg.z.store(z);
            // Continue
            cycle4(code, cpu)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Execute SWAP
    let op1 = cpu.reg.z.load();
    let res = ((op1 & 0xf0) >> 4) | ((op1 & 0x0f) << 4);

    // Set flags
    cpu.reg.f.set_z(res == 0);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(false);

    // Check opcode
    match code {
        0x36 => {
            // Write [HL]
            cpu.writebyte(res);
            // Proceed
            cpu.step(cycle5)
        }
        0x30..=0x37 => {
            // Store r8
            help::set_op8(cpu, code & 0x07, res);
            // Finish
            None
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle5(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
