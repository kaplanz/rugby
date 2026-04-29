use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Daa(Daa::Execute)
}

#[derive(Clone, Debug, Default)]
pub enum Daa {
    #[default]
    Execute,
}

impl Execute for Daa {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Execute => execute(code, cpu),
        }
    }
}

impl From<Daa> for Operation {
    fn from(value: Daa) -> Self {
        Self::Daa(value)
    }
}

fn execute(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    if code != 0x27 {
        return Err(Error::Opcode(code));
    }

    // Execute DAA
    let didsub = cpu.reg.f.n();
    let hcarry = cpu.reg.f.h();
    let mut carry = cpu.reg.f.c();
    let mut adj = 0i8;
    let acc = cpu.reg.a.load();
    if hcarry || (!didsub && (acc & 0x0f) > 0x09) {
        adj |= 0x06;
    }
    if carry || (!didsub && acc > 0x99) {
        adj |= 0x60;
        carry = true;
    }
    adj = if didsub { -adj } else { adj };
    let res = (acc as i8).wrapping_add(adj) as u8;
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(res == 0);
    cpu.reg.f.set_h(false);
    cpu.reg.f.set_c(carry);

    // Finish
    Ok(None)
}
