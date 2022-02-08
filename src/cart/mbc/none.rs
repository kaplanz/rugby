use std::cell::RefCell;
use std::cmp::Ordering;
use std::iter;
use std::rc::Rc;

use log::{error, info, warn};
use remus::mem::{self, Ram};
use remus::{Block, Device};

use super::Mbc;

#[derive(Debug, Default)]
pub struct NoMbc {
    rom: Rc<RefCell<Rom<0x8000>>>,
    ram: Rc<RefCell<Ram<0x2000>>>,
}

impl Block for NoMbc {
    fn reset(&mut self) {
        self.ram.borrow_mut().reset();
    }
}

impl Mbc for NoMbc {
    fn load(&mut self, rom: &[u8]) {
        // Calculate buffer stats
        let read = rom.len();
        let size = self.rom.borrow().0.len();
        let diff = size - read;
        match read.cmp(&size) {
            Ordering::Less => {
                error!("Read {read} bytes; remaining {diff} bytes uninitialized.")
            }
            Ordering::Equal => info!("Read {read} bytes."),
            Ordering::Greater => {
                error!("Read {read} bytes; remaining {diff} bytes uninitialized.")
            }
        }
        let rom: [u8; 0x8000] = rom
            .iter()
            .cloned()
            .chain(iter::repeat(0u8))
            .take(0x8000)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        // Initialize from ROM
        self.rom.replace(Rom::from(&rom));
    }

    fn rom(&self) -> Rc<RefCell<dyn Device>> {
        self.rom.clone()
    }

    fn ram(&self) -> Rc<RefCell<dyn Device>> {
        self.ram.clone()
    }
}

#[derive(Debug, Default)]
struct Rom<const N: usize>(mem::Rom<N>);

impl<const N: usize> Device for Rom<N> {
    fn contains(&self, index: usize) -> bool {
        self.0.contains(index)
    }

    fn read(&self, index: usize) -> u8 {
        self.0.read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        warn!("called `Device::write({index:#06x}, {value:#04x})` on a `Rom`");
    }
}

impl<const N: usize> From<&[u8; N]> for Rom<N> {
    fn from(arr: &[u8; N]) -> Self {
        Self(arr.into())
    }
}
