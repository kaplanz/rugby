use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Ccf(Ccf::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Ccf {
    #[default]
    Execute,
}

impl Execute for Ccf {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Ccf> for Operation {
    fn from(value: Ccf) -> Self {
        Self::Ccf(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x3f {
        return Err(Error::Opcode(code));
    }

    // Execute CCF
    let c = cpu.reg.f.c();
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(!c);

    // Finish
    Ok(None)
}
