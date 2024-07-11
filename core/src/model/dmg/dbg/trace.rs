//! Introspective tracing.

use std::fmt::Write;

use rugby_arch::reg::Register;

use super::GameBoy;
use crate::api::core::Core;

/// Collect a trace formatted as specified by [Gameboy Doctor][gbdoc].
///
/// [gbdoc]: https://robertheaton.com/gameboy-doctor
#[must_use]
pub fn doctor(emu: &GameBoy) -> String {
    let mut repr = String::new();
    let cpu = emu.inside().proc();
    write!(&mut repr, "A:{:02X} ", cpu.reg.a.load()).unwrap();
    write!(&mut repr, "F:{:02X} ", cpu.reg.f.load()).unwrap();
    write!(&mut repr, "B:{:02X} ", cpu.reg.b.load()).unwrap();
    write!(&mut repr, "C:{:02X} ", cpu.reg.c.load()).unwrap();
    write!(&mut repr, "D:{:02X} ", cpu.reg.d.load()).unwrap();
    write!(&mut repr, "E:{:02X} ", cpu.reg.e.load()).unwrap();
    write!(&mut repr, "H:{:02X} ", cpu.reg.h.load()).unwrap();
    write!(&mut repr, "L:{:02X} ", cpu.reg.l.load()).unwrap();
    write!(&mut repr, "SP:{:04X} ", cpu.reg.sp.load()).unwrap();
    write!(&mut repr, "PC:{:04X} ", cpu.reg.pc.load()).unwrap();
    let pcmem = [0, 1, 2, 3]
        .map(|i| cpu.reg.pc.load() + i)
        .map(|addr| cpu.read(addr));
    write!(
        &mut repr,
        "PCMEM:{:02X},{:02X},{:02X},{:02X}",
        pcmem[0], pcmem[1], pcmem[2], pcmem[3],
    )
    .unwrap();
    repr
}
