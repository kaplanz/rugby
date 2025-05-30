use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Decw(Decw::Fetch)
}

#[derive(Clone, Debug, Default)]
pub enum Decw {
    #[default]
    Fetch,
    Execute(u16),
}

impl Execute for Decw {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Fetch        => fetch(code, cpu),
            Self::Execute(op1) => execute(code, cpu, op1),
        }
    }
}

impl From<Decw> for Operation {
    fn from(value: Decw) -> Self {
        Self::Decw(value)
    }
}

fn fetch(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    let op1 = match code {
        0x0b => cpu.reg.bc().load(),
        0x1b => cpu.reg.de().load(),
        0x2b => cpu.reg.hl().load(),
        0x3b => cpu.reg.sp.load(),
        code => return Err(Error::Opcode(code)),
    };

    // Proceed
    Ok(Some(Decw::Execute(op1).into()))
}

fn execute(code: u8, cpu: &mut Cpu, op1: u16) -> Return {
    // Execute DECW
    let res = op1.wrapping_sub(1);
    match code {
        0x0b => {
            cpu.reg.bc_mut().store(res);
        }
        0x1b => {
            cpu.reg.de_mut().store(res);
        }
        0x2b => {
            cpu.reg.hl_mut().store(res);
        }
        0x3b => cpu.reg.sp.store(res),
        code => return Err(Error::Opcode(code)),
    }

    // Finish
    Ok(None)
}
