use std::cell::RefCell;
use std::rc::Rc;

use remus::bus::adapters::{Bank, View};
use remus::bus::Bus;
use remus::dev::Null;
use remus::{Block, Device};

use super::Mbc;

#[derive(Debug)]
pub struct Mbc1 {
    rom: Rc<RefCell<Rom>>,
    ram: Rc<RefCell<Ram>>,
}

impl Mbc1 {
    pub fn with(
        rom: Rc<RefCell<dyn Device>>,
        ram: Rc<RefCell<dyn Device>>,
        _battery: bool,
    ) -> Self {
        // Prepare RAM
        #[allow(clippy::vec_init_then_push)]
        let ram = {
            // Determine how many banks to create
            let ramsz = ram.borrow().len();
            let nbanks = ramsz / 0x4000;
            // Create banks as `View`s of the RAM
            let mut banks: Vec<Rc<RefCell<dyn Device>>> = Default::default();
            banks.push(Rc::new(RefCell::new(Null::<0>::new()))); // disable RAM at index 0
            for i in 0..nbanks {
                let range = (0x4000 * i)..(0x4000 * (i + 1));
                banks.push(Rc::new(RefCell::new(View::new(ram.clone(), range))));
            }
            // Create the RAM bank object
            let bank = Bank { active: 0, banks };
            let bank = Rc::new(RefCell::new(bank));

            Ram(bank)
        };
        // Prepare ROM
        let rom = {
            // Determine how many banks to create
            let romsz = rom.borrow().len();
            let nbanks = romsz / 0x4000;
            // Create banks as `View`s of the ROM
            let mut banks: Vec<Rc<RefCell<dyn Device>>> = Default::default();
            for i in 0..nbanks {
                let range = (0x4000 * i)..(0x4000 * (i + 1));
                banks.push(Rc::new(RefCell::new(View::new(rom.clone(), range))));
            }
            // Create the ROM bank object
            let rom0 = banks.remove(0);
            let bank = Bank { active: 0, banks };
            let rom = Rc::new(RefCell::new(bank));
            // Use a bus to join ROM banks together
            let mut bus = Bus::new();
            bus.map(0x0000, rom0);
            bus.map(0x4000, rom.clone());
            let bus = Rc::new(RefCell::new(bus));

            Rom {
                bus,
                rom,
                ram: ram.0.clone(),
            }
        };

        Self {
            rom: Rc::new(RefCell::new(rom)),
            ram: Rc::new(RefCell::new(ram)),
        }
    }
}

impl Block for Mbc1 {
    fn reset(&mut self) {
        // Reset ROM bank
        self.rom.borrow_mut().reset();
        // Reset RAM
        self.ram.borrow_mut().reset();
    }
}

impl Mbc for Mbc1 {
    fn rom(&self) -> Rc<RefCell<dyn Device>> {
        self.rom.clone()
    }

    fn ram(&self) -> Rc<RefCell<dyn Device>> {
        self.ram.clone()
    }
}

#[derive(Debug)]
struct Ram(Rc<RefCell<Bank>>);

impl Block for Ram {}

impl Device for Ram {
    fn contains(&self, index: usize) -> bool {
        self.0.borrow().contains(index)
    }

    fn len(&self) -> usize {
        self.0.borrow().len()
    }

    fn read(&self, index: usize) -> u8 {
        self.0.borrow().read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        self.0.borrow_mut().write(index, value);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rom {
    bus: Rc<RefCell<Bus>>,
    rom: Rc<RefCell<Bank>>,
    ram: Rc<RefCell<Bank>>,
}

impl Block for Rom {}

impl Device for Rom {
    fn contains(&self, index: usize) -> bool {
        self.bus.borrow().contains(index)
    }

    fn len(&self) -> usize {
        self.bus.borrow().len()
    }

    fn read(&self, index: usize) -> u8 {
        self.bus.borrow().read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        match index {
            // RAM Enable
            0x0000..=0x1fff => {
                // TODO: RAM Enable
            }
            // ROM Bank Number
            0x2000..=0x3fff => {
                // FIXME: Mask depending on ROM size
                self.rom.borrow_mut().active = match value & 0x1f {
                    0x00 => 0x00,
                    bank => bank - 1,
                } as usize;
            }
            0x4000..=0x5fff => {
                // TODO: RAM Bank Number - or - Upper Bits of ROM Bank Number
            }
            0x6000..=0x7fff => {
                // TODO: Banking Mode Select
            }
            _ => panic!(), // TODO: some error here
        }
    }
}
