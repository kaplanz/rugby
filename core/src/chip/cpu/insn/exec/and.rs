use std::ops::BitAnd;

use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return, help};

pub const fn default() -> Operation {
    Operation::And(And::Fetch)
}

#[derive(Clone, Debug, Default)]
pub enum And {
    #[default]
    Fetch,
    Execute(u8),
}

impl Execute for And {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Fetch        => fetch(code, cpu),
            Self::Execute(op2) => execute(code, cpu, op2),
        }
    }
}

impl From<And> for Operation {
    fn from(value: And) -> Self {
        Self::And(value)
    }
}

fn fetch(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    match code {
        0xa6 => {
            // Read [HL]
            let op2 = cpu.readbyte();
            // Proceed
            Ok(Some(And::Execute(op2).into()))
        }
        0xe6 => {
            // Fetch n8 <- [PC++]
            let op2 = cpu.fetchbyte();
            // Proceed
            Ok(Some(And::Execute(op2).into()))
        }
        0xa0..=0xa7 => {
            // Prepare op2
            let op2 = help::get_op8(cpu, code & 0x07);
            // Continue
            execute(code, cpu, op2)
        }
        code => Err(Error::Opcode(code)),
    }
}

fn execute(_: u8, cpu: &mut Cpu, op2: u8) -> Return {
    // Execute AND
    let acc = cpu.reg.a.load();
    let res = acc.bitand(op2);
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(res == 0);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(true);
    cpu.reg.f.set_c(false);

    // Finish
    Ok(None)
}
