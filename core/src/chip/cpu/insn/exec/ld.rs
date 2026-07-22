use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction, help};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    #[expect(clippy::match_same_arms)]
    match code {
        // LD [r16], A
        0x02 | 0x12 | 0x22 | 0x32 => {
            // Load r16
            let r16 = match code {
                0x02 => cpu.reg.bc(),
                0x12 => cpu.reg.de(),
                0x22 | 0x32 => cpu.reg.hl(),
                code => unreachable!("unexpected opcode: {code:#04X}"),
            }
            .load();
            // Load A
            let op2 = cpu.reg.a.load();
            // Write A
            cpu.blk.bus.write(r16, op2);
            // Update HL
            match code {
                0x22 => {
                    let res = cpu.blk.idu.inc(r16);
                    cpu.reg.hl_mut().store(res);
                }
                0x32 => {
                    let res = cpu.blk.idu.dec(r16);
                    cpu.reg.hl_mut().store(res);
                }
                _ => {}
            }
            // Proceed
            cpu.step(cycle3)
        }
        // LD A, [r16]
        0x0a | 0x1a | 0x2a | 0x3a => {
            // Load r16
            let r16 = match code {
                0x0a => cpu.reg.bc(),
                0x1a => cpu.reg.de(),
                0x2a | 0x3a => cpu.reg.hl(),
                code => unreachable!("unexpected opcode: {code:#04X}"),
            }
            .load();
            // Read Z <- [r16]
            let z = cpu.blk.bus.read(r16);
            cpu.reg.z.store(z);
            // Update HL
            match code {
                0x2a => {
                    let res = cpu.blk.idu.inc(r16);
                    cpu.reg.hl_mut().store(res);
                }
                0x3a => {
                    let res = cpu.blk.idu.dec(r16);
                    cpu.reg.hl_mut().store(res);
                }
                _ => {}
            }
            // Proceed
            cpu.step(cycle3)
        }
        // LD r8, n8
        0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        // LD [HL], n8
        0x36 => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        // LD r8, [HL]
        0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x7e => {
            // Read Z <- [HL]
            let z = cpu.readbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        // HALT (unexpected opcode)
        0x76 => unreachable!("unexpected opcode: {code:#04X}"),
        // LD [HL], r8
        0x70..=0x77 => {
            // Load HL
            let addr = cpu.reg.hl().load();
            // Prepare op2
            let op2 = help::get_op8(cpu, code & 0x07);
            // Write op2
            cpu.blk.bus.write(addr, op2);
            // Proceed
            cpu.step(cycle3)
        }
        // LD r8, r8
        0x40..=0x7f => {
            // Prepare Z
            let z = help::get_op8(cpu, code & 0x07);
            cpu.reg.z.store(z);
            // Continue
            cycle3(code, cpu)
        }
        // LD [a16], A
        // LD A, [a16]
        0xea | 0xfa => {
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
        // LD [r16], A
        // LD [HL], r8
        0x02 | 0x12 | 0x22 | 0x32 | 0x70..=0x77 => {
            // Delay by 1 cycle

            // Finish
            None
        }
        // LD [HL], n8
        0x36 => {
            // Write [HL] <- Z
            let z = cpu.reg.z.load();
            cpu.writebyte(z);
            // Proceed
            cpu.step(cycle4)
        }
        // LD [a16], A
        // LD A, [a16]
        0xea | 0xfa => {
            // Fetch W <- [PC++]
            let w = cpu.fetchbyte();
            cpu.reg.w.store(w);
            // Proceed
            cpu.step(cycle4)
        }
        // Store r8 <- Z
        _ => {
            let z = cpu.reg.z.load();
            let op1 = match code {
                0x06 | 0x40..=0x47 => &mut cpu.reg.b,
                0x0e | 0x48..=0x4f => &mut cpu.reg.c,
                0x16 | 0x50..=0x57 => &mut cpu.reg.d,
                0x1e | 0x58..=0x5f => &mut cpu.reg.e,
                0x26 | 0x60..=0x67 => &mut cpu.reg.h,
                0x2e | 0x68..=0x6f => &mut cpu.reg.l,
                0x0a | 0x1a | 0x2a | 0x3a | 0x3e | 0x78..=0x7f => &mut cpu.reg.a,
                code => unreachable!("unexpected opcode: {code:#04X}"),
            };
            op1.store(z);
            // Finish
            None
        }
    }
}

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Load WZ
    let wz = cpu.reg.wz().load();
    match code {
        // LD [HL], n8
        0x36 => {
            // Delay by 1 cycle

            // Finish
            None
        }
        // LD [a16], A
        0xea => {
            // Write [WZ] <- A
            let op2 = cpu.reg.a.load();
            cpu.blk.bus.write(wz, op2);
            // Proceed
            cpu.step(cycle5)
        }
        // LD A, [a16]
        0xfa => {
            // Read Z <- [WZ]
            let z = cpu.blk.bus.read(wz);
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle5)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle5(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // LD [a16], A
        0xea => {
            // Delay by 1 cycle

            // Finish
            None
        }
        // LD A, [a16]
        0xfa => {
            // Store A <- Z
            let z = cpu.reg.z.load();
            cpu.reg.a.store(z);
            // Finish
            None
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}
