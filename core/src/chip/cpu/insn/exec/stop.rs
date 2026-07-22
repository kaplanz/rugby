use rugby_arch::Block;

use super::{Cpu, Exec, Instruction, Status};

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(code: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Check opcode
    if code != 0x10 {
        unreachable!("unexpected opcode: {code:#04X}");
    }

    // Skip stop mode while a button is held
    if cpu.blk.bus.read(0xff00) & 0x0f == 0x0f {
        // Reset the divider
        cpu.blk.bus.write(0xff04, 0);
        // Enter stop mode
        cpu.etc.run = Status::Stopped;
    }
    // Consume the padding byte unless an interrupt is pending
    if !cpu.irq.pending() {
        cpu.fetchbyte();
    }

    // Park until woken
    if cpu.etc.run == Status::Stopped {
        return cpu.step(wait);
    }

    // Release the blocks for the composed fetch
    cpu.blk.cycle();
    // Finish
    None
}

fn wait(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Keep waiting while stopped
    if cpu.etc.run == Status::Stopped {
        return cpu.step(wait);
    }

    // Finish
    None
}
