use enuf::Enuf;
use remus::Cell;

use super::{help, Cpu, Error, Execute, Flag, Operation, Return};

pub const fn default() -> Operation {
    Operation::Rl(Rl::Fetch)
}

#[derive(Clone, Debug, Default)]
pub enum Rl {
    #[default]
    Fetch,
    Execute(u8),
    Delay,
}

impl Execute for Rl {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Fetch        => fetch(code, cpu),
            Self::Execute(op1) => execute(code, cpu, op1),
            Self::Delay        => delay(code, cpu),
        }
    }
}

impl From<Rl> for Operation {
    fn from(value: Rl) -> Self {
        Self::Rl(value)
    }
}

fn fetch(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    match code {
        0x16 => {
            // Read [HL]
            let op1 = cpu.readbyte();
            // Proceed
            Ok(Some(Rl::Execute(op1).into()))
        }
        0x10..=0x17 => {
            // Prepare op2
            let op1 = help::get_op8(cpu, code & 0x07);
            // Continue
            execute(code, cpu, op1)
        }
        code => Err(Error::Opcode(code)),
    }
}

fn execute(code: u8, cpu: &mut Cpu, op1: u8) -> Return {
    // Execute RL
    let flags = &mut cpu.file.f.load();
    let cin = Flag::C.get(flags);
    let carry = op1 & 0x80 != 0;
    let res = op1 << 1 | (cin as u8);

    // Set flags
    let flags = &mut cpu.file.f.load();
    Flag::Z.set(flags, res == 0);
    Flag::N.set(flags, false);
    Flag::H.set(flags, false);
    Flag::C.set(flags, carry);
    cpu.file.f.store(*flags);

    // Check opcode
    match code {
        0x16 => {
            // Write [HL]
            cpu.writebyte(res);
            // Proceed
            Ok(Some(Rl::Delay.into()))
        }
        0x10..=0x17 => {
            // Store r8
            help::set_op8(cpu, code & 0x07, res);
            // Finish
            Ok(None)
        }
        code => Err(Error::Opcode(code)),
    }
}

fn delay(_: u8, _: &mut Cpu) -> Return {
    // Delay by 1 cycle

    // Finish
    Ok(None)
}
