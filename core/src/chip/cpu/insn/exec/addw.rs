use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // ADD HL, r16
        0x09 | 0x19 | 0x29 | 0x39 => {
            // Load operands
            let op1 = cpu.reg.l.load();
            let op2 = match code {
                0x09 => cpu.reg.c.load(),
                0x19 => cpu.reg.e.load(),
                0x29 => cpu.reg.l.load(),
                0x39 => cpu.reg.sp.load().to_le_bytes()[0],
                code => unreachable!("unexpected opcode: {code:#04X}"),
            };

            // Execute ADDW (LSB)
            let (res, carry) = op1.overflowing_add(op2);
            cpu.reg.l.store(res);

            // Set flags
            cpu.reg.f.set_n(false);
            cpu.reg.f.set_h(0x0f < (op1 & 0x0f) + (op2 & 0x0f));
            cpu.reg.f.set_c(carry);

            // Proceed
            cpu.step(cycle3)
        }
        // ADD SP, e8
        0xe8 => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);

            // Proceed
            cpu.step(cycle3)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // ADD HL, r16
        0x09 | 0x19 | 0x29 | 0x39 => {
            // Load operands
            let op1 = cpu.reg.h.load();
            let op2 = match code {
                0x09 => cpu.reg.b.load(),
                0x19 => cpu.reg.d.load(),
                0x29 => cpu.reg.h.load(),
                0x39 => cpu.reg.sp.load().to_le_bytes()[1],
                code => unreachable!("unexpected opcode: {code:#04X}"),
            };
            let cin = cpu.reg.f.c() as u8;

            // Execute ADDW (MSB)
            let (res, carry0) = op1.overflowing_add(op2);
            let (res, carry1) = res.overflowing_add(cin);
            cpu.reg.h.store(res);

            // Set flags
            cpu.reg.f.set_n(false);
            cpu.reg.f.set_h(0x0f < (op1 & 0x0f) + (op2 & 0x0f) + cin);
            cpu.reg.f.set_c(carry0 | carry1);

            // Finish
            None
        }
        // ADD SP, e8
        0xe8 => {
            // Load operands
            let op1 = cpu.reg.sp.load().to_le_bytes()[0];
            let op2 = cpu.reg.z.load();

            // Execute ADDW (LSB)
            let res = op1.wrapping_add(op2);
            cpu.reg.z.store(res);

            // Set flags
            cpu.reg.f.set_z(false);
            cpu.reg.f.set_n(false);
            cpu.reg.f.set_h(0x0f < (op1 & 0x0f) + (op2 & 0x0f));
            cpu.reg.f.set_c(0xff < u16::from(op1) + u16::from(op2));

            // Proceed
            cpu.step(cycle4)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle4(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Recover the operand's sign
    let spl = cpu.reg.sp.load().to_le_bytes()[0];
    let e8 = cpu.reg.z.load().wrapping_sub(spl);
    // Prepare the adjustment
    let adj = if e8 & 0x80 == 0 { 0x00 } else { 0xff };
    let cin = cpu.reg.f.c() as u8;

    // Execute ADDW (MSB)
    let op1 = cpu.reg.sp.load().to_le_bytes()[1];
    let res = op1.wrapping_add(adj).wrapping_add(cin);
    cpu.reg.w.store(res);

    // Proceed
    cpu.step(cycle5)
}

fn cycle5(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Store SP <- WZ
    let wz = cpu.reg.wz().load();
    cpu.reg.sp.store(wz);

    // Finish
    None
}
