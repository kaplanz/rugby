use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return};

pub const fn default() -> Operation {
    Operation::Pop(Pop::Pop1)
}

#[derive(Clone, Debug, Default)]
pub enum Pop {
    #[default]
    Pop1,
    Pop0,
    Delay,
}

impl Execute for Pop {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Pop1  => pop1(code, cpu),
            Self::Pop0  => pop0(code, cpu),
            Self::Delay => delay(code, cpu),
        }
    }
}

impl From<Pop> for Operation {
    fn from(value: Pop) -> Self {
        Self::Pop(value)
    }
}

fn pop1(code: u8, cpu: &mut Cpu) -> Return {
    // Pop LSB <- [SP++]
    let lsb = cpu.popbyte();

    // Store LSB
    match code {
        0xc1 => cpu.reg.c.store(lsb),
        0xd1 => cpu.reg.e.store(lsb),
        0xe1 => cpu.reg.l.store(lsb),
        0xf1 => cpu.reg.f.store(lsb),
        code => return Err(Error::Opcode(code)),
    }

    // Proceed
    Ok(Some(Pop::Pop0.into()))
}

fn pop0(code: u8, cpu: &mut Cpu) -> Return {
    // Pop MSB <- [SP++]
    let msb = cpu.popbyte();

    // Store MSB
    match code {
        0xc1 => &mut cpu.reg.b,
        0xd1 => &mut cpu.reg.d,
        0xe1 => &mut cpu.reg.h,
        0xf1 => &mut cpu.reg.a,
        code => return Err(Error::Opcode(code)),
    }
    .store(msb);

    // Finish
    Ok(Some(Pop::Delay.into()))
}

fn delay(_: u8, _: &mut Cpu) -> Return {
    // Delay by 1 cycle

    // Finish
    Ok(None)
}
