use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return, help};

pub const fn default() -> Operation {
    Operation::Cp(Cp::Fetch)
}

#[derive(Clone, Debug, Default)]
pub enum Cp {
    #[default]
    Fetch,
    Execute(u8),
}

impl Execute for Cp {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Fetch        => fetch(code, cpu),
            Self::Execute(op2) => execute(code, cpu, op2),
        }
    }
}

impl From<Cp> for Operation {
    fn from(value: Cp) -> Self {
        Self::Cp(value)
    }
}

fn fetch(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    match code {
        0xbe => {
            // Read [HL]
            let op2 = cpu.readbyte();
            // Proceed
            Ok(Some(Cp::Execute(op2).into()))
        }
        0xfe => {
            // Fetch n8 <- [PC++]
            let op2 = cpu.fetchbyte();
            // Proceed
            Ok(Some(Cp::Execute(op2).into()))
        }
        0xb8..=0xbf => {
            // Prepare op2
            let op2 = help::get_op8(cpu, code & 0x07);
            // Continue
            execute(code, cpu, op2)
        }
        code => Err(Error::Opcode(code)),
    }
}

fn execute(_: u8, cpu: &mut Cpu, op2: u8) -> Return {
    // Execute CP
    let acc = cpu.reg.a.load();
    let (res, carry) = acc.overflowing_sub(op2);

    // Set flags
    cpu.reg.f.set_z(res == 0);
    cpu.reg.f.set_n(true);
    cpu.reg.f.set_h((op2 & 0x0f) > (acc & 0x0f));
    cpu.reg.f.set_c(carry);

    // Finish
    Ok(None)
}
