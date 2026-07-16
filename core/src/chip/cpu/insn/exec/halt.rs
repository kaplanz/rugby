use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return, Status};

pub const fn default() -> Operation {
    Operation::Halt(Halt::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Halt {
    #[default]
    Execute,
}

impl Execute for Halt {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Halt> for Operation {
    fn from(value: Halt) -> Self {
        Self::Halt(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x76 {
        return Err(Error::Opcode(code));
    }

    // Handle pending interrupts
    if cpu.int.pending() {
        // Skip halt mode entirely
        if cpu.etc.ime.enabled() {
            // Rewind onto this instruction
            let pc = cpu.reg.pc.load().wrapping_sub(1);
            cpu.reg.pc.store(pc);
        } else {
            // Perform HALT bug
            cpu.etc.halt_bug = true;
        }
    } else {
        // Execute HALT
        cpu.etc.run = Status::Halted;
    }

    // Finish
    Ok(None)
}
