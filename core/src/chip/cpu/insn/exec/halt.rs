use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction, Status};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0x76 {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Handle pending interrupts
    if cpu.irq.pending() {
        // Skip halt mode entirely
        if cpu.etc.ime.enabled() {
            // Rewind onto this instruction
            let pc = cpu.reg.pc.load().wrapping_sub(1);
            cpu.reg.pc.store(pc);
        } else {
            // Perform HALT bug
            cpu.etc.halt_bug = true;
        }
        // Finish
        return None;
    }

    // Execute HALT
    cpu.etc.run = Status::Halted;

    // Await an interrupt
    cpu.step(wait)
}

fn wait(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Keep waiting while halted
    if cpu.etc.run == Status::Halted {
        return cpu.step(wait);
    }

    // Finish
    None
}
