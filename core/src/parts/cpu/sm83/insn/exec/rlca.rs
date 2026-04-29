use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Rlca(Rlca::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Rlca {
    #[default]
    Execute,
}

impl Execute for Rlca {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Rlca> for Operation {
    fn from(value: Rlca) -> Self {
        Self::Rlca(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x07 {
        return Err(Error::Opcode(code));
    }

    // Execute RLCA
    let acc = cpu.reg.a.load();
    let carry = acc & 0x80 != 0;
    let res = (acc << 1) | (carry as u8);
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(false);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(carry);

    // Finish
    Ok(None)
}
