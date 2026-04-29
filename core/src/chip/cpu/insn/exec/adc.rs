use rugby_arch::reg::Register;

use super::{Cpu, Error, Execute, Operation, Return, help};

pub const fn default() -> Operation {
    Operation::Adc(Adc::Fetch)
}

#[derive(Clone, Debug, Default)]
pub enum Adc {
    #[default]
    Fetch,
    Execute(u8),
}

impl Execute for Adc {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Fetch        => fetch(code, cpu),
            Self::Execute(op2) => execute(code, cpu, op2),
        }
    }
}

impl From<Adc> for Operation {
    fn from(value: Adc) -> Self {
        Self::Adc(value)
    }
}

fn fetch(code: u8, cpu: &mut Cpu) -> Return {
    // Check opcode
    match code {
        0x8e => {
            // Read [HL]
            let op2 = cpu.readbyte();
            // Proceed
            Ok(Some(Adc::Execute(op2).into()))
        }
        0xce => {
            // Fetch n8 <- [PC++]
            let op2 = cpu.fetchbyte();
            // Proceed
            Ok(Some(Adc::Execute(op2).into()))
        }
        0x88..=0x8f => {
            // Prepare op2
            let op2 = help::get_op8(cpu, code & 0x07);
            // Continue
            execute(code, cpu, op2)
        }
        code => Err(Error::Opcode(code)),
    }
}

fn execute(_: u8, cpu: &mut Cpu, op2: u8) -> Return {
    // Execute ADC
    let acc = cpu.reg.a.load();
    let cin = cpu.reg.f.c() as u8;
    let (res, carry0) = acc.overflowing_add(op2);
    let (res, carry1) = res.overflowing_add(cin);
    cpu.reg.a.store(res);

    // Set flags
    cpu.reg.f.set_z(res == 0);
    cpu.reg.f.set_n(false);
    cpu.reg.f.set_h(0x0f < (acc & 0x0f) + (op2 & 0x0f) + cin);
    cpu.reg.f.set_c(carry0 | carry1);

    // Finish
    Ok(None)
}
