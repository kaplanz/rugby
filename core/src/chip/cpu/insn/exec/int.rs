use rugby_arch::reg::Register;

use super::{Cpu, Exec, Instruction};
use crate::chip::irq::Interrupt;

pub const fn default() -> Exec {
    cycle2
}

fn cycle2(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Proceed
    cpu.step(cycle3)
}

fn cycle3(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Load MSB
    let msb = cpu.reg.pc.load().to_le_bytes()[1];

    // Push MSB -> [--SP]
    cpu.pushbyte(msb);

    // Sample Z <- IE after the high push
    cpu.reg.z.store(cpu.irq.ena());

    // Proceed
    cpu.step(cycle4)
}

fn cycle4(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Load LSB
    let lsb = cpu.reg.pc.load().to_le_bytes()[0];

    // Push LSB -> [--SP]
    cpu.pushbyte(lsb);

    // Combine Z <- Z & IF after the low push
    let z = cpu.reg.z.load() & cpu.irq.flg();
    cpu.reg.z.store(z);

    // Proceed
    cpu.step(cycle5)
}

fn cycle5(_: u8, cpu: &mut Cpu) -> Option<Instruction> {
    // Derive the dispatched interrupt from Z
    let int: Option<Interrupt> = cpu.reg.z.load().try_into().ok();

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

    // Proceed
    cpu.step(cycle6)
}

fn cycle6(_: u8, _: &mut Cpu) -> Option<Instruction> {
    // Delay by 1 cycle

    // Finish
    None
}
