use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Flag, Operation, Return, help};

pub const fn default() -> Operation {
    Operation::Sub(Sub::Fetch)
}

#[derive(Clone, Debug, Default)]
pub enum Sub {
    #[default]
    Fetch,
    Execute(u8),
}

impl Execute for Sub {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Fetch        => fetch(code, cpu),
            Self::Execute(op2) => execute(code, cpu, op2),
        }
    }
}

impl From<Sub> for Operation {
    fn from(value: Sub) -> Self {
        Self::Sub(value)
    }
}

fn fetch(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    match code {
        0x96 => {
            // Read [HL]
            let op2 = cpu.readbyte();
            // Proceed
            Ok(Some(Sub::Execute(op2).into()))
        }
        0xd6 => {
            // Fetch n8
            let op2 = cpu.fetchbyte();
            // Proceed
            Ok(Some(Sub::Execute(op2).into()))
        }
        0x90..=0x97 => {
            // Prepare op2
            let op2 = help::get_op8(cpu, code & 0x07);
            // Continue
            execute(code, cpu, op2)
        }
        code => Err(Error::Opcode(code)),
    }
}

fn execute(_: u8, cpu: &mut Cpu, op2: u8) -> Return {
    // Execute SUB
    let acc = cpu.reg.a.load();
    let (res, carry) = acc.overflowing_sub(op2);
    cpu.reg.a.store(res);

    // Set flags
    let flags = &mut cpu.reg.f.load();
    Flag::Z.set(flags, res == 0);
    Flag::N.set(flags, true);
    Flag::H.set(flags, (op2 & 0x0f) > (acc & 0x0f));
    Flag::C.set(flags, carry);
    cpu.reg.f.store(*flags);

    // Finish
    Ok(None)
}
