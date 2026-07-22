use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // LDH [a8], A
        // LDH A, [a8]
        0xe0 | 0xf0 => {
            // Fetch Z <- [PC++]
            let z = cpu.fetchbyte();
            cpu.reg.z.store(z);
            // Proceed
            cpu.step(cycle3)
        }
        // LDH [C], A
        0xe2 => {
            // Load Z <- C
            let z = cpu.reg.c.load();
            cpu.reg.z.store(z);
            // Write [$FF00 + Z] <- A
            write(cpu);
            // Proceed
            cpu.step(cycle3)
        }
        // LDH A, [C]
        0xf2 => {
            // Load Z <- C
            let z = cpu.reg.c.load();
            cpu.reg.z.store(z);
            // Read Z <- [$FF00 + Z]
            read(cpu);
            // Proceed
            cpu.step(cycle4)
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn cycle3(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // LDH [a8], A
        0xe0 => {
            // Write [$FF00 + Z] <- A
            write(cpu);
            // Proceed
            cpu.step(cycle4)
        }
        // LDH A, [a8]
        0xf0 => {
            // Read Z <- [$FF00 + Z]
            read(cpu);
            // Proceed
            cpu.step(cycle4)
        }
        // LDH [C], A
        0xe2 => {
            // Delay by 1 cycle

            // Finish
            None
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}

fn read(cpu: &mut Cpu) {
    // Calculate absolute address
    let addr = u16::from_be_bytes([0xff, cpu.reg.z.load()]);

    // Read Z <- [$FF00 + Z]
    let z = cpu.blk.bus.read(addr);
    cpu.reg.z.store(z);
}

fn write(cpu: &mut Cpu) {
    // Calculate absolute address
    let addr = u16::from_be_bytes([0xff, cpu.reg.z.load()]);

    // Write [$FF00 + Z] <- A
    let op2 = cpu.reg.a.load();
    cpu.blk.bus.write(addr, op2);
}

fn cycle4(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    match code {
        // LDH [a8], A
        0xe0 => {
            // Delay by 1 cycle

            // Finish
            None
        }
        // LDH A, [a8]
        // LDH A, [C]
        0xf0 | 0xf2 => {
            // Store A <- Z
            let z = cpu.reg.z.load();
            cpu.reg.a.store(z);
            // Finish
            None
        }
        code => unreachable!("unexpected opcode: {code:#04X}"),
    }
}
