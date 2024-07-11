//! Introspective tracing.

use std::fmt::Write;

use rugby_arch::reg::{Port, Register};
use rugby_arch::{Byte, Word};

use super::cpu::{Flag, Select16, Select8};
use super::GameBoy;
use crate::api::core::Core;

/// Collect a trace with formatting matching [binjgb].
///
/// [binjgb]: https://github.com/binji/binjgb
#[must_use]
pub fn binjgb(emu: &GameBoy) -> String {
    let mut repr = String::new();
    let cpu = emu.inside().proc();
    write!(&mut repr, "A:{:02X} ", Port::<Byte>::load(cpu, Select8::A)).unwrap();
    write!(&mut repr, "F:{} ", {
        let flags = Port::<Byte>::load(cpu, Select8::F);
        format!(
            "{}{}{}{}",
            if Flag::Z.get(&flags) { "Z" } else { "-" },
            if Flag::N.get(&flags) { "N" } else { "-" },
            if Flag::H.get(&flags) { "H" } else { "-" },
            if Flag::C.get(&flags) { "C" } else { "-" },
        )
    })
    .unwrap();
    write!(
        &mut repr,
        "BC:{:04X} ",
        Port::<Word>::load(cpu, Select16::BC)
    )
    .unwrap();
    write!(
        &mut repr,
        "DE:{:04x} ",
        Port::<Word>::load(cpu, Select16::DE)
    )
    .unwrap();
    write!(
        &mut repr,
        "HL:{:04x} ",
        Port::<Word>::load(cpu, Select16::HL)
    )
    .unwrap();
    write!(
        &mut repr,
        "SP:{:04x} ",
        Port::<Word>::load(cpu, Select16::SP)
    )
    .unwrap();
    write!(
        &mut repr,
        "PC:{:04x}",
        Port::<Word>::load(cpu, Select16::PC)
    )
    .unwrap();
    repr
}

/// Collect a trace with formatting as specified by [Gameboy Doctor][gbdoc].
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
