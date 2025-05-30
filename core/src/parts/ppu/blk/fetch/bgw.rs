use rugby_arch::Block;

use super::meta::{Layer, Meta, Row};
use super::{Fifo, Lcdc, Ppu, Step};

/// Background fetcher.
#[derive(Clone, Debug)]
pub struct Fetcher {
    /// Background FIFO.
    pub fifo: Fifo,
    /// Fetcher step.
    pub step: Step,
    /// X position counter.
    pub xpos: u8,
    /// Graphics layer.
    pub layer: Layer,
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            fifo: Fifo::default(),
            step: Step::Fetch,
            xpos: u8::default(),
            layer: Layer::Background,
        }
    }
}

impl Fetcher {
    /// Executes a cycle of the fetcher.
    pub fn exec(&mut self, ppu: &mut Ppu) {
        self.step = match self.step {
            Step::Fetch => exec::fetch(ppu, self),
            Step::Read0 { tdat } => exec::read0(ppu, tdat),
            Step::Read1 { tdat, data } => exec::read1(ppu, tdat, data),
            Step::Push { data } => exec::push(self, data),
        }
    }
}

impl Block for Fetcher {
    fn reset(&mut self) {
        std::mem::take(&mut self.fifo);
        std::mem::take(&mut self.step);
        std::mem::take(&mut self.xpos);
    }
}

/// Execution steps.
pub(super) mod exec {
    use log::trace;
    use rugby_arch::mem::Memory;
    use rugby_arch::reg::Register;

    use super::{Fetcher, Layer, Lcdc, Meta, Ppu, Row, Step};

    /// Executes fetch tile step.
    pub fn fetch(ppu: &Ppu, fetch: &mut Fetcher) -> Step {
        // Determine which tile map to use
        let tmap = [0x1800, 0x1c00][usize::from(ppu.lcdc(match fetch.layer {
            Layer::Background => Lcdc::BgMap,
            Layer::Window => Lcdc::WinMap,
            Layer::Sprite => unreachable!(),
        }))];
        // Calculate the offset from the pixel coordinates
        let toff = {
            let row: u8;
            let col: u8;
            match fetch.layer {
                Layer::Background => {
                    row = ppu.reg.ly.load().wrapping_add(ppu.reg.scy.load()) / 8;
                    col = (fetch.xpos + ppu.reg.scx.load() / 8) & 0x1f;
                }
                Layer::Window => {
                    row = ppu.etc.ywin / 8;
                    col = fetch.xpos;
                }
                Layer::Sprite => unreachable!(),
            }
            (32 * (u16::from(row)) + u16::from(col)) & 0x03ff
        };
        // Read the tile number from the tile map
        let addr = tmap + toff;
        let tnum = ppu.mem.vram.read(addr).unwrap();
        trace!("read tile index: VRAM[${addr:04x}] -> #{tnum}");
        // Calculate the tile data address
        let tdat = ppu.tdat(fetch.layer, tnum, ppu.reg.scy.load());

        // Progress to next step
        Step::Read0 { tdat }
    }

    /// Executes read tile data low.
    pub fn read0(ppu: &Ppu, tdat: u16) -> Step {
        // Fetch the low byte of the tile
        let data = ppu.mem.vram.read(tdat).unwrap();
        trace!("read lower byte: VRAM[${tdat:04x}] -> {data:#04x}");

        // Progress to next step
        let tdat = tdat + 1;
        Step::Read1 { tdat, data }
    }

    /// Executes read tile data high.
    pub fn read1(ppu: &Ppu, tdat: u16, data0: u8) -> Step {
        // Fetch the high byte of the tile
        let data1 = ppu.mem.vram.read(tdat).unwrap();
        trace!("read upper byte: VRAM[${tdat:04x}] -> {data1:#04x}");

        // Progress to next step
        Step::Push {
            data: [data0, data1],
        }
    }

    /// Executes push to FIFO.
    pub fn push(fetch: &mut Fetcher, data: [u8; 2]) -> Step {
        // Decode pixel row from bytes
        let row = Row::from(data);
        let meta = Meta::Bgw;

        // Only push when the FIFO is empty
        if fetch.fifo.is_empty() {
            // Push to the FIFO
            trace!("pushed row of pixels: {row:?}");
            fetch.fifo.push(row, meta);
            // Advance x position
            fetch.xpos += 1;
            // Restart from beginning
            Step::Fetch
        } else {
            trace!("stalled; background FIFO non-empty");
            // Try again next cycle
            Step::Push { data }
        }
    }
}
