use std::cell::RefCell;
use std::cmp::Ordering;
use std::iter;
use std::rc::Rc;

use log::{debug, error, info};
use remus::dev::Null;
use remus::mem::{Memory, Ram, Rom};
use remus::{Block, Device};
use thiserror::Error;

use self::header::Header;
use self::mbc::{Mbc, Mbc1, NoMbc};
use crate::cart::header::CartridgeType;

mod header;
mod mbc;

#[derive(Debug)]
pub struct Cartridge {
    header: Header,
    mbc: Box<dyn Mbc>,
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Result<Cartridge, Error> {
        // Parse cartridge header
        let header = Header::try_from(&*rom)?;
        info!("Cartridge:\n{header}");

        // Construct null device (for reuse where needed)
        let null = Rc::new(RefCell::new(Null::<0>::new()));

        // Prepare external ROM
        let rom = {
            // Calculate buffer stats
            let read = rom.len();
            match read.cmp(&header.romsz) {
                Ordering::Less => {
                    let diff = header.romsz - read;
                    error!("Read {read} bytes; remaining {diff} byte(s) uninitialized.")
                }
                Ordering::Equal => info!("Read {read} bytes."),
                Ordering::Greater => {
                    let diff = read - header.romsz;
                    error!("Read {read} bytes; remaining {diff} byte(s) truncated.")
                }
            }
            rom.iter()
                .cloned()
                .chain(iter::repeat(0u8))
                .take(header.romsz)
                .collect::<Vec<_>>()
        };
        debug!("ROM:\n{}", &rom as &dyn Memory);

        // Construct external ROM
        let erom: Rc<RefCell<dyn Device>> = {
            let rom: Box<[_]> = rom.into();
            match header.romsz {
                0x8000 => Rc::new(RefCell::new(Rom::<0x8000>::from(
                    &*Box::<[_; 0x8000]>::try_from(rom).unwrap(),
                ))),
                0x10000 => Rc::new(RefCell::new(Rom::<0x10000>::from(
                    &*Box::<[_; 0x10000]>::try_from(rom).unwrap(),
                ))),
                0x20000 => Rc::new(RefCell::new(Rom::<0x20000>::from(
                    &*Box::<[_; 0x20000]>::try_from(rom).unwrap(),
                ))),
                0x40000 => Rc::new(RefCell::new(Rom::<0x40000>::from(
                    &*Box::<[_; 0x40000]>::try_from(rom).unwrap(),
                ))),
                0x80000 => Rc::new(RefCell::new(Rom::<0x80000>::from(
                    &*Box::<[_; 0x80000]>::try_from(rom).unwrap(),
                ))),
                0x100000 => Rc::new(RefCell::new(Rom::<0x100000>::from(
                    &*Box::<[_; 0x100000]>::try_from(rom).unwrap(),
                ))),
                0x200000 => Rc::new(RefCell::new(Rom::<0x200000>::from(
                    &*Box::<[_; 0x200000]>::try_from(rom).unwrap(),
                ))),
                0x400000 => Rc::new(RefCell::new(Rom::<0x400000>::from(
                    &*Box::<[_; 0x400000]>::try_from(rom).unwrap(),
                ))),
                0x800000 => Rc::new(RefCell::new(Rom::<0x800000>::from(
                    &*Box::<[_; 0x800000]>::try_from(rom).unwrap(),
                ))),
                _ => unreachable!(),
            }
        };

        // Construct external RAM
        let eram: Rc<RefCell<dyn Device>> = match header.ramsz {
            0x0 => null.clone(),
            0x2000 => Rc::new(RefCell::new(Ram::<0x2000>::new())),
            0x8000 => Rc::new(RefCell::new(Ram::<0x8000>::new())),
            0x20000 => Rc::new(RefCell::new(Ram::<0x20000>::new())),
            0x10000 => Rc::new(RefCell::new(Ram::<0x10000>::new())),
            _ => unreachable!(),
        };

        // Construct a cartridge
        let mbc: Box<dyn Mbc> = match header.cart {
            CartridgeType::Rom { ram, .. } => {
                let eram = [null, eram][ram as usize].clone();
                Box::new(NoMbc::with(erom, eram))
            }
            CartridgeType::Mbc1 { ram, battery } => {
                let eram = [null, eram][ram as usize].clone();
                Box::new(Mbc1::with(erom, eram, battery))
            }
            cart => unimplemented!("{cart:?}"),
        };

        Ok(Self { header, mbc })
    }

    /// Get a reference to the cartridge's header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn rom(&self) -> Rc<RefCell<dyn Device>> {
        self.mbc.rom()
    }

    pub fn ram(&self) -> Rc<RefCell<dyn Device>> {
        self.mbc.ram()
    }
}

impl Block for Cartridge {
    fn reset(&mut self) {
        self.mbc.reset();
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not parse ROM header")]
    Header(#[from] header::Error),
}
