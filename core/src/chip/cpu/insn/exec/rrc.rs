use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction, help};

pub const fn default() -> Exec {
    cycle3
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        0x0e => {
            // Read Z <- [HL]
            let z = cpu.readbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle4)
        }
        0x08..=0x0f => {
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
    // Execute RRC
    let op1 = cpu.reg.z.load();
    let carry = op1 & 0x01 != 0;
    let res = ((carry as u8) << 7) | (op1 >> 1);

    // Set flags
    cpu.reg.f.set_z(res == 0);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(carry);

    // Check opcode
    match code {
        0x0e => {
            // Write [HL]
            cpu.writebyte(res);
            // Proceed
            cpu.step(cycle5)
        }
        0x08..=0x0f => {
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
