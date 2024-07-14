//! Introspective tracing.

use rugby_arch::reg::Register;
use rugby_arch::Block;

use super::cpu::Flag;
use super::GameBoy;

/// Collect a trace with formatting matching [binjgb].
///
/// [binjgb]: https://github.com/binji/binjgb
#[must_use]
pub fn binjgb(emu: &GameBoy) -> String {
    let cpu = &emu.main.soc.cpu;
    let ppu = &emu.main.soc.ppu;
    [
        format!("A:{:02X}", cpu.reg.a),
        format!("F:{}", {
            [Flag::Z, Flag::N, Flag::H, Flag::C]
                .map(|flag| {
                    if flag.get(&cpu.reg.f) {
                        format!("{flag:?}")
                    } else {
                        "-".to_string()
                    }
                })
                .join("")
        }),
        format!("BC:{:04X}", cpu.reg.bc().load()),
        format!("DE:{:04x}", cpu.reg.de().load()),
        format!("HL:{:04x}", cpu.reg.hl().load()),
        format!("SP:{:04x}", cpu.reg.sp),
        format!("PC:{:04x}", cpu.reg.pc),
        format!("(cy: {})", emu.main.clk),
        format!(
            "ppu:{}{}",
            ['-', '+'][usize::from(ppu.ready())],
            ppu.mode().value()
        ),
    ]
    .join(" ")
}

/// Collect a trace with formatting as specified by [Gameboy Doctor][gbdoc].
///
/// [gbdoc]: https://robertheaton.com/gameboy-doctor
#[must_use]
pub fn doctor(emu: &GameBoy) -> String {
    let cpu = &emu.main.soc.cpu;
    [
        format!("A:{:02X}", cpu.reg.a.load()),
        format!("F:{:02X}", cpu.reg.f.load()),
        format!("B:{:02X}", cpu.reg.b.load()),
        format!("C:{:02X}", cpu.reg.c.load()),
        format!("D:{:02X}", cpu.reg.d.load()),
        format!("E:{:02X}", cpu.reg.e.load()),
        format!("H:{:02X}", cpu.reg.h.load()),
        format!("L:{:02X}", cpu.reg.l.load()),
        format!("SP:{:04X}", cpu.reg.sp.load()),
        format!("PC:{:04X}", cpu.reg.pc.load()),
        format!(
            "PCMEM:{}",
            [0, 1, 2, 3]
                .map(|offs| cpu.reg.pc.load() + offs)
                .map(|addr| cpu.read(addr))
                .map(|data| format!("{data:02X}"))
                .join(","),
        ),
    ]
    .join(" ")
}
