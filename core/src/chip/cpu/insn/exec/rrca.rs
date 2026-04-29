use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Rrca(Rrca::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Rrca {
    #[default]
    Execute,
}

impl Execute for Rrca {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Rrca> for Operation {
    fn from(value: Rrca) -> Self {
        Self::Rrca(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x0f {
        return Err(Error::Opcode(code));
    }

    // Execute RRCA
    let acc = cpu.reg.a.load();
    let carry = acc & 0x01 != 0;
    let res = ((carry as u8) << 7) | (acc >> 1);
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(false);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(carry);

    // Finish
    Ok(None)
}
