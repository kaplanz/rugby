use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Scf(Scf::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Scf {
    #[default]
    Execute,
}

impl Execute for Scf {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Scf> for Operation {
    fn from(value: Scf) -> Self {
        Self::Scf(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x37 {
        return Err(Error::Opcode(code));
    }

    // Execute SCF
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(true);

    // Finish
    Ok(None)
}
