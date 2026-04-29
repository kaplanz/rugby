use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Rra(Rra::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Rra {
    #[default]
    Execute,
}

impl Execute for Rra {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Rra> for Operation {
    fn from(value: Rra) -> Self {
        Self::Rra(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x1f {
        return Err(Error::Opcode(code));
    }

    // Execute RRA
    let acc = cpu.reg.a.load();
    let cin = cpu.reg.f.c();
    let carry = acc & 0x01 != 0;
    let res = ((cin as u8) << 7) | (acc >> 1);
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(false);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(carry);

    // Finish
    Ok(None)
}
