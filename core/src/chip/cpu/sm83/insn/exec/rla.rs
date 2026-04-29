use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Rla(Rla::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Rla {
    #[default]
    Execute,
}

impl Execute for Rla {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Rla> for Operation {
    fn from(value: Rla) -> Self {
        Self::Rla(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x17 {
        return Err(Error::Opcode(code));
    }

    // Execute RLA
    let acc = cpu.reg.a.load();
    let cin = cpu.reg.f.c();
    let carry = acc & 0x80 != 0;
    let res = (acc << 1) | (cin as u8);
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(false);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(carry);

    // Finish
    Ok(None)
}
