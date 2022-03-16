use std::cell::RefCell;
use std::rc::Rc;

use remus::bus::adapters::{Bank, View};
use remus::bus::Bus;
use remus::dev::Null;
use remus::{Block, Device};

use super::Mbc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Mbc1 {
    rom: Rc<RefCell<dyn Device>>,
    rombank: RomBank,
    ram: Rc<RefCell<dyn Device>>,
    rambank: RamBank,
}

impl Mbc1 {
    pub fn with(
        rom: Rc<RefCell<dyn Device>>,
        ram: Rc<RefCell<dyn Device>>,
        _battery: bool,
    ) -> Self {
        Self {
            rombank: RomBank::from(rom.clone()),
            rom,
            rambank: RamBank::from(ram.clone()),
            ram,
        }
    }
}

impl Block for Mbc1 {
    fn reset(&mut self) {
        // Reset ROM bank
        self.rombank.reset();
        self.rombank.ram = self.rambank.0.clone();
        // Reset RAM
        self.ram.borrow_mut().reset();
        self.rambank.reset();
    }
}

impl Mbc for Mbc1 {
    fn rom(&self) -> Rc<RefCell<dyn Device>> {
        self.rombank.bus.clone()
    }

    fn ram(&self) -> Rc<RefCell<dyn Device>> {
        self.rambank.0.clone()
    }
}

#[derive(Debug)]
struct RomBank {
    bus: Rc<RefCell<Bus>>,
    rom: Rc<RefCell<Bank>>,
    ram: Rc<RefCell<Bank>>,
}

impl Block for RomBank {}

impl Device for RomBank {
    fn contains(&self, index: usize) -> bool {
        self.bus.borrow().contains(index)
    }

    fn read(&self, index: usize) -> u8 {
        self.bus.borrow().read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        match index {
            // RAM Enable
            0x0000..=0x1fff => {
                if value & 0x0f == 0x0a {
                    self.ram.borrow_mut().active = 0;
                }
            }
            // ROM Bank Number
            0x2000..=0x3fff => {
                self.rom.borrow_mut().active = match value & 0x1f {
                    0x00 => 0x01,
                    bank => bank,
                } as usize;
            }
            0x4000..=0x5fff => {}
            0x6000..=0x7fff => (),
            _ => panic!(), // TODO: some error here
        }
    }
}

impl From<Rc<RefCell<dyn Device>>> for RomBank {
    fn from(rom: Rc<RefCell<dyn Device>>) -> Self {
        // Determine how many banks to create
        let romsz = std::mem::size_of_val(&*rom.borrow());
        let nbanks = romsz / 0x4000;
        // Create banks as `View`s of the ROM
        let mut banks: Vec<Rc<RefCell<dyn Device>>> = Default::default();
        for i in 0..nbanks {
            let range = (0x4000 * i)..(0x4000 * (i + 1));
            banks.push(Rc::new(RefCell::new(View::new(rom.clone(), range))));
        }
        // Create the ROM bank object
        let rom0 = banks.remove(0);
        let bank = Bank { active: 1, banks };
        let rom = Rc::new(RefCell::new(bank));
        // Use a bus to join ROM banks together
        let mut bus = Bus::new();
        bus.map(0x0000, rom0);
        bus.map(0x4000, rom.clone());
        let bus = Rc::new(RefCell::new(bus));

        Self {
            bus,
            rom,
            ram: Default::default(),
        }
    }
}

#[derive(Debug)]
struct RamBank(Rc<RefCell<Bank>>);

impl Block for RamBank {}

impl Device for RamBank {
    fn contains(&self, index: usize) -> bool {
        self.0.borrow().contains(index)
    }

    fn read(&self, index: usize) -> u8 {
        self.0.borrow().read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        self.0.borrow_mut().write(index, value);
    }
}

#[allow(clippy::vec_init_then_push)]
impl From<Rc<RefCell<dyn Device>>> for RamBank {
    fn from(ram: Rc<RefCell<dyn Device>>) -> Self {
        // Determine how many banks to create
        let ramsz = std::mem::size_of_val(&*ram.borrow());
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

        Self(bank)
    }
}
