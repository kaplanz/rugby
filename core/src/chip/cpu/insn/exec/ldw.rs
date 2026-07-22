use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    #[expect(clippy::match_same_arms)]
    match code {
        // LD r16, n16
        // LD [a16], SP
        0x01 | 0x08 | 0x11 | 0x21 | 0x31 => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        // LD HL, SP + e8
        0xf8 => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        // LD SP, HL
        0xf9 => {
            // Store SP <- HL
            let hl = cpu.reg.hl().load();
            cpu.reg.sp.store(hl);
            // Proceed
            cpu.step(cycle3)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // LD SP, HL
        0xf9 => {
            // Delay by 1 cycle

            // Finish
            None
        }
        // LD r16, n16
        // LD [a16], SP
        0x01 | 0x08 | 0x11 | 0x21 | 0x31 => {
            // Fetch W <- [PC++]
            let w = cpu.fetchbyte();
            cpu.reg.w.store(w);
            // Proceed
            cpu.step(cycle4)
        }
        // LD HL, SP + e8
        0xf8 => {
            // Load operands
            let op1 = cpu.reg.sp.load().to_le_bytes()[0];
            let op2 = cpu.reg.z.load();

            // Execute LDW (LSB)
            let res = op1.wrapping_add(op2);
            cpu.reg.l.store(res);

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

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // LD r16, n16
        0x01 | 0x11 | 0x21 | 0x31 => {
            // Store r16 <- WZ
            let wz = cpu.reg.wz().load();
            match code {
                0x01 => cpu.reg.bc_mut().store(wz),
                0x11 => cpu.reg.de_mut().store(wz),
                0x21 => cpu.reg.hl_mut().store(wz),
                0x31 => cpu.reg.sp.store(wz),
                code => unreachable!("unexpected opcode: {code:#04X}"),
            }
            // Finish
            None
        }
        // LD [a16], SP
        0x08 => {
            // Write [WZ] <- lower(SP)
            let wz = cpu.reg.wz().load();
            let sp = cpu.reg.sp.load();
            cpu.blk.bus.write(wz, sp.to_le_bytes()[0]);
            // Increment WZ
            let wz = cpu.blk.idu.inc(wz);
            cpu.reg.wz_mut().store(wz);
            // Proceed
            cpu.step(cycle5)
        }
        // LD HL, SP + e8
        0xf8 => {
            // Prepare the adjustment
            let e8 = cpu.reg.z.load();
            let adj = if e8 & 0x80 == 0 { 0x00 } else { 0xff };
            let cin = cpu.reg.f.c() as u8;

            // Execute LDW (MSB)
            let op1 = cpu.reg.sp.load().to_le_bytes()[1];
            let res = op1.wrapping_add(adj).wrapping_add(cin);
            cpu.reg.h.store(res);

            // Finish
            None
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle5(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Write [WZ] <- upper(SP)
    let wz = cpu.reg.wz().load();
    let sp = cpu.reg.sp.load();
    cpu.blk.bus.write(wz, sp.to_le_bytes()[1]);

    // Proceed
    cpu.step(cycle6)
}

fn cycle6(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
