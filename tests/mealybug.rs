#![allow(non_snake_case)]

use remus::Machine;
use rugby::core::dmg::cart::Cartridge;
use rugby::core::dmg::GameBoy;
use rugby::emu::cart::Support as _;

mod common;

use common::image::{self, Result};

const TIMEOUT: usize = 1_000_000;

fn emulate(rom: &[u8], img: &[u8]) -> Result<()> {
    // Instantiate a cartridge
    let cart = Cartridge::new(rom).unwrap();
    // Create an emulator instance
    let mut emu = GameBoy::new();
    // Load the cartridge
    emu.load(cart);

    // Loop until timeout
    for _ in 0..TIMEOUT {
        emu.cycle();
    }
    // Calculate difference
    let delta = image::cmp(emu.ppu().screen(), img);
    let total = img.len();

    // Check for success
    image::check(delta, total)
}

macro_rules! test {
    ($($test:ident,)*) => {
        $(
            #[test]
            fn $test() -> Result<()> {
                let rom = include_bytes!(concat!(
                    "../roms/test/mealybug/",
                    stringify!($test),
                    ".gb",
                ));
                let img = &image::png(include_bytes!(concat!(
                    "../roms/test/mealybug/expected/",
                    stringify!($test),
                    ".png",
                )))
                .unwrap();
                emulate(rom, img)
            }
        )*
    };
}

test! {
    m2_win_en_toggle,
    m3_bgp_change,
    m3_bgp_change_sprites,
    m3_lcdc_bg_en_change,
    m3_lcdc_bg_map_change,
    m3_lcdc_obj_en_change,
    m3_lcdc_obj_en_change_variant,
    m3_lcdc_obj_size_change,
    m3_lcdc_obj_size_change_scx,
    m3_lcdc_tile_sel_change,
    m3_lcdc_tile_sel_win_change,
    m3_lcdc_win_en_change_multiple,
    m3_lcdc_win_en_change_multiple_wx,
    m3_lcdc_win_map_change,
    m3_obp0_change,
    m3_scx_high_5_bits,
    m3_scx_low_3_bits,
    m3_scy_change,
    m3_window_timing,
    m3_window_timing_wx_0,
    m3_wx_4_change,
    m3_wx_4_change_sprites,
    m3_wx_5_change,
    m3_wx_6_change,
}