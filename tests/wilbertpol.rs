#![allow(non_snake_case)]

use std::fmt::{Debug, Display};

use rugby::arch::Block;
use rugby::arch::reg::Port;
use rugby::core::Revision;
use rugby::core::cart::Cartridge;
use rugby::core::dmg::soc::cpu::Cpu;
use rugby::core::dmg::{GameBoy, rev};

/// Number of cycles after which the test is considered to have failed due to a
/// timeout error.
const TIMEOUT: usize = 10_000_000;

/// Perform integration test emulation.
fn emulate<R: Revision>(mut emu: GameBoy<R>, rom: &[u8]) -> Result<()>
where
    GameBoy<R>: Block,
{
    // Instantiate a cartridge
    let cart = Cartridge::new(rom).unwrap();
    // Load the cartridge
    emu.insert(cart);

    // Loop until completion or timeout
    for _ in 0..TIMEOUT {
        emu.cycle();

        // Check for success or failure
        match check(&emu) {
            Err(Error::Running) => continue,
            res => return res,
        }
    }

    // Fail with timeout if reached
    Err(Error::Timeout)
}

/// Check for test results.
fn check<R: Revision>(emu: &GameBoy<R>) -> Result<()> {
    type Select = <Cpu as Port<u16>>::Select;
    // Extract register values
    let cpu = &emu.inner().soc.cpu;
    let bc: u16 = cpu.load(Select::BC);
    let de: u16 = cpu.load(Select::DE);
    let hl: u16 = cpu.load(Select::HL);
    // Calculate pass/fail conditions
    let pass = (bc == 0x0305) && (de == 0x080d) && (hl == 0x1522);
    let fail = (bc == 0x4242) && (de == 0x4242) && (hl == 0x4242);
    // Report results
    if fail {
        Err(Error::Failed)
    } else if pass {
        Ok(())
    } else {
        Err(Error::Running)
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
type Result<T, E = Error> = std::result::Result<T, E>;

/// Failure conditions caused by a test.
#[derive(thiserror::Error)]
enum Error {
    /// Test has explicitly failed.
    #[error("test failed")]
    Failed,
    /// Test is still in progress.
    #[error("test in progress")]
    Running,
    /// Test has reached timeout.
    #[error("timeout reached")]
    Timeout,
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

macro_rules! test {
    ($($test:ident = $path:tt;)*) => {
        $(
            #[test]
            fn $test() -> Result<()> {
                emulate(GameBoy::<rev::A>::new(), include_bytes!($path))
            }
        )*
    };
    ($rev:ty; $($test:ident = $path:tt;)*) => {
        $(
            #[test]
            fn $test() -> Result<()> {
                emulate(GameBoy::<$rev>::new(), include_bytes!($path))
            }
        )*
    };
}

test! {
    acceptance_gpu_hblank_ly_scx_timing_GS               = "../roms/test/wilbertpol/acceptance/gpu/hblank_ly_scx_timing-GS.gb";
    acceptance_gpu_hblank_ly_scx_timing_nops             = "../roms/test/wilbertpol/acceptance/gpu/hblank_ly_scx_timing_nops.gb";
    acceptance_gpu_hblank_ly_scx_timing_variant_nops     = "../roms/test/wilbertpol/acceptance/gpu/hblank_ly_scx_timing_variant_nops.gb";
    acceptance_gpu_intr_0_timing                         = "../roms/test/wilbertpol/acceptance/gpu/intr_0_timing.gb";
    acceptance_gpu_intr_1_2_timing_GS                    = "../roms/test/wilbertpol/acceptance/gpu/intr_1_2_timing-GS.gb";
    acceptance_gpu_intr_1_timing                         = "../roms/test/wilbertpol/acceptance/gpu/intr_1_timing.gb";
    acceptance_gpu_intr_2_0_timing                       = "../roms/test/wilbertpol/acceptance/gpu/intr_2_0_timing.gb";
    acceptance_gpu_intr_2_mode0_scx1_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx1_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_scx2_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx2_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_scx3_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx3_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_scx4_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx4_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_scx5_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx5_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_scx6_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx6_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_scx7_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx7_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_scx8_timing_nops         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_scx8_timing_nops.gb";
    acceptance_gpu_intr_2_mode0_timing                   = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_timing.gb";
    acceptance_gpu_intr_2_mode0_timing_sprites           = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_timing_sprites.gb";
    acceptance_gpu_intr_2_mode0_timing_sprites_nops      = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_timing_sprites_nops.gb";
    acceptance_gpu_intr_2_mode0_timing_sprites_scx1_nops = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_timing_sprites_scx1_nops.gb";
    acceptance_gpu_intr_2_mode0_timing_sprites_scx2_nops = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_timing_sprites_scx2_nops.gb";
    acceptance_gpu_intr_2_mode0_timing_sprites_scx3_nops = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_timing_sprites_scx3_nops.gb";
    acceptance_gpu_intr_2_mode0_timing_sprites_scx4_nops = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode0_timing_sprites_scx4_nops.gb";
    acceptance_gpu_intr_2_mode3_timing                   = "../roms/test/wilbertpol/acceptance/gpu/intr_2_mode3_timing.gb";
    acceptance_gpu_intr_2_oam_ok_timing                  = "../roms/test/wilbertpol/acceptance/gpu/intr_2_oam_ok_timing.gb";
    acceptance_gpu_intr_2_timing                         = "../roms/test/wilbertpol/acceptance/gpu/intr_2_timing.gb";
    acceptance_gpu_lcdon_mode_timing                     = "../roms/test/wilbertpol/acceptance/gpu/lcdon_mode_timing.gb";
    acceptance_gpu_ly00_01_mode0_2                       = "../roms/test/wilbertpol/acceptance/gpu/ly00_01_mode0_2.gb";
    acceptance_gpu_ly00_mode0_2_GS                       = "../roms/test/wilbertpol/acceptance/gpu/ly00_mode0_2-GS.gb";
    acceptance_gpu_ly00_mode1_0_GS                       = "../roms/test/wilbertpol/acceptance/gpu/ly00_mode1_0-GS.gb";
    acceptance_gpu_ly00_mode2_3                          = "../roms/test/wilbertpol/acceptance/gpu/ly00_mode2_3.gb";
    acceptance_gpu_ly00_mode3_0                          = "../roms/test/wilbertpol/acceptance/gpu/ly00_mode3_0.gb";
    acceptance_gpu_ly143_144_145                         = "../roms/test/wilbertpol/acceptance/gpu/ly143_144_145.gb";
    acceptance_gpu_ly143_144_152_153                     = "../roms/test/wilbertpol/acceptance/gpu/ly143_144_152_153.gb";
    acceptance_gpu_ly143_144_mode0_1                     = "../roms/test/wilbertpol/acceptance/gpu/ly143_144_mode0_1.gb";
    acceptance_gpu_ly143_144_mode3_0                     = "../roms/test/wilbertpol/acceptance/gpu/ly143_144_mode3_0.gb";
    acceptance_gpu_ly_lyc_GS                             = "../roms/test/wilbertpol/acceptance/gpu/ly_lyc-GS.gb";
    acceptance_gpu_ly_lyc_0_GS                           = "../roms/test/wilbertpol/acceptance/gpu/ly_lyc_0-GS.gb";
    acceptance_gpu_ly_lyc_0_write_GS                     = "../roms/test/wilbertpol/acceptance/gpu/ly_lyc_0_write-GS.gb";
    acceptance_gpu_ly_lyc_144_GS                         = "../roms/test/wilbertpol/acceptance/gpu/ly_lyc_144-GS.gb";
    acceptance_gpu_ly_lyc_153_GS                         = "../roms/test/wilbertpol/acceptance/gpu/ly_lyc_153-GS.gb";
    acceptance_gpu_ly_lyc_153_write_GS                   = "../roms/test/wilbertpol/acceptance/gpu/ly_lyc_153_write-GS.gb";
    acceptance_gpu_ly_lyc_write_GS                       = "../roms/test/wilbertpol/acceptance/gpu/ly_lyc_write-GS.gb";
    acceptance_gpu_ly_new_frame_GS                       = "../roms/test/wilbertpol/acceptance/gpu/ly_new_frame-GS.gb";
    acceptance_gpu_stat_irq_blocking                     = "../roms/test/wilbertpol/acceptance/gpu/stat_irq_blocking.gb";
    acceptance_gpu_stat_write_if_GS                      = "../roms/test/wilbertpol/acceptance/gpu/stat_write_if-GS.gb";
    acceptance_gpu_vblank_if_timing                      = "../roms/test/wilbertpol/acceptance/gpu/vblank_if_timing.gb";
    acceptance_gpu_vblank_stat_intr_GS                   = "../roms/test/wilbertpol/acceptance/gpu/vblank_stat_intr-GS.gb";
}
