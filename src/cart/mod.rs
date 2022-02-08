use anyhow::Result;
use remus::Block;

use self::header::Header;
pub use self::mbc::{Mbc, NoMbc};

mod header;
mod mbc;

#[derive(Debug)]
pub struct Cartridge {
    pub header: Header,
    pub mbc: Box<dyn Mbc>,
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Result<Cartridge> {
        // Parse cartridge header
        let header = Header::try_from(&*rom)?;

        // Construct a cartridge
        let mut mbc = match header.cart {
            0x00 => Box::new(NoMbc::default()),
            _ => unreachable!(),
        };

        // Load the ROM
        mbc.load(rom);

        Ok(Self { header, mbc })
    }
}

impl Block for Cartridge {
    fn reset(&mut self) {
        self.mbc.reset();
    }
}
