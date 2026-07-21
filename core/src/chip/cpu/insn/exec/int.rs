use rugby_arch::reg::Register;

use super::{Cpu, Execute, Ime, Operation, Return};
use crate::chip::irq::Interrupt;

#[derive(Clone, Debug, Default)]
pub enum Int {
    #[default]
    Fetch,
    Nop,
    Push0,
    Push1(u8),
    Jump(Option<Interrupt>),
}

impl Execute for Int {
    #[rustfmt::skip]
    fn exec(self, code: u8, cpu: &mut Cpu) -> Return {
        match self {
            Self::Fetch      => fetch(code, cpu),
            Self::Nop        => nop(code, cpu),
            Self::Push0      => push0(code, cpu),
            Self::Push1(ena) => push1(code, cpu, ena),
            Self::Jump(int)  => jump(code, cpu, int),
        }
    }
}

impl From<Int> for Operation {
    fn from(value: Int) -> Self {
        Self::Int(value)
    }
}

fn fetch(_: u8, cpu: &mut Cpu) -> Return {
    // Disable interrupts
    cpu.etc.ime = Ime::Disabled;

    // Proceed
    Ok(Some(Int::Nop.into()))
}

fn nop(_: u8, _: &mut Cpu) -> Return {
    // Execute NOP

    // Proceed
    Ok(Some(Int::Push0.into()))
}

fn push0(_: u8, cpu: &mut Cpu) -> Return {
    // Load MSB
    let msb = cpu.reg.pc.load().to_le_bytes()[1];

    // Push MSB -> [--SP]
    cpu.pushbyte(msb);

    // Sample IE after the high push
    let ena = cpu.irq.ena();

    // Proceed
    Ok(Some(Int::Push1(ena).into()))
}

fn push1(_: u8, cpu: &mut Cpu, ena: u8) -> Return {
    // Load LSB
    let lsb = cpu.reg.pc.load().to_le_bytes()[0];

    // Push LSB -> [--SP]
    cpu.pushbyte(lsb);

    // Re-derive the dispatched interrupt after the pushes
    let int = (ena & cpu.irq.flg()).try_into().ok();

    // Proceed
    Ok(Some(Int::Jump(int).into()))
}

fn jump(_: u8, cpu: &mut Cpu, int: Option<Interrupt>) -> Return {
    // Acknowledge the interrupt
    if let Some(int) = int {
        cpu.irq.clear(int);
    }

    // Jump to the handler
    cpu.reg.pc.store(u16::from_be_bytes([
        0x00,
        int
            // Resolve the interrupt vector
            .map(Interrupt::handler)
            // Resolve to zero if cancelled
            .unwrap_or_default(),
    ]));

    // Finish
    Ok(None)
}
