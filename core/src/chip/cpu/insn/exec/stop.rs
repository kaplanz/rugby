use super::{Cpu, Error, Execute, Operation, Return, Status};

pub const fn default() -> Operation {
    Operation::Stop(Stop::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Stop {
    #[default]
    Execute,
}

impl Execute for Stop {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Stop> for Operation {
    fn from(value: Stop) -> Self {
        Self::Stop(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x10 {
        return Err(Error::Opcode(code));
    }

    // Skip stop mode while a button is held
    if cpu.read(0xff00) & 0x0f == 0x0f {
        // Reset the divider
        cpu.write(0xff04, 0);
        // Enter stop mode
        cpu.etc.run = Status::Stopped;
    }
    // Consume the padding byte unless an interrupt is pending
    if !cpu.irq.pending() {
        cpu.fetchbyte();
    }

    // Finish
    Ok(None)
}
