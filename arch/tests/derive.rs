use rugby_arch::Shared;
use rugby_arch::mem::{Error, Memory, Ram};

/// Sample memory map.
#[derive(Debug)]
#[derive(Memory)]
struct Board {
    /// Single mapped address.
    #[mmap(0x0010)]
    reg: u8,
    /// Mapped address range.
    #[mmap(0x0100..=0x01ff)]
    ram: Ram<[u8; 0x200]>,
    /// Masked address ranges.
    #[mmap(0x8000..=0x9fff, mask = 0x1fff)]
    #[mmap(0xe000..=0xffff, mask = 0x1fff)]
    wram: Shared<Ram<[u8; 0x2000]>>,
}

fn setup() -> Board {
    Board {
        reg: 0xaa,
        ram: Ram::from([0xbb; 0x200]),
        wram: Shared::new(Ram::from([0xcc; 0x2000])),
    }
}

#[test]
fn single_read_works() {
    let dev = setup();
    assert_eq!(dev.read(0x0010), Ok(0xaa));
    assert!(matches!(dev.read(0x0011), Err(Error::Range)));
}

#[test]
fn single_write_works() {
    let mut dev = setup();
    dev.write(0x0010, 0x55).unwrap();
    assert_eq!(dev.reg, 0x55);
    assert!(matches!(dev.write(0x0011, 0x55), Err(Error::Range)));
}

#[test]
fn range_read_works() {
    let dev = setup();
    assert_eq!(dev.read(0x0100), Ok(0xbb));
    assert_eq!(dev.read(0x01ff), Ok(0xbb));
    assert!(matches!(dev.read(0x0200), Err(Error::Range)));
}

#[test]
fn range_write_works() {
    let mut dev = setup();
    dev.write(0x0123, 0x55).unwrap();
    // Unmasked arms delegate the address unchanged.
    assert_eq!(dev.ram.read(0x0123), Ok(0x55));
    assert_eq!(dev.ram.read(0x0023), Ok(0xbb));
}

#[test]
fn mask_read_works() {
    let dev = setup();
    dev.wram.borrow_mut().write(0x0123, 0x55).unwrap();
    // Masked arms fold onto the device's own address space.
    assert_eq!(dev.read(0x8123), Ok(0x55));
    assert_eq!(dev.read(0x8124), Ok(0xcc));
}

#[test]
fn mask_write_works() {
    let mut dev = setup();
    dev.write(0x9fff, 0x55).unwrap();
    assert_eq!(dev.wram.borrow().read(0x1fff), Ok(0x55));
}

#[test]
fn ranges_alias_works() {
    let mut dev = setup();
    // Both ranges map onto the same device (echo).
    dev.write(0x8123, 0x55).unwrap();
    assert_eq!(dev.read(0xe123), Ok(0x55));
    dev.write(0xffff, 0x66).unwrap();
    assert_eq!(dev.read(0x9fff), Ok(0x66));
}

#[test]
#[should_panic = "address is unmapped"]
fn unmapped_read_panics() {
    let dev = setup();
    dev.read(0x4000).expect("address is unmapped");
}

#[test]
#[should_panic = "address is unmapped"]
fn unmapped_write_panics() {
    let mut dev = setup();
    dev.write(0x4000, 0x55).expect("address is unmapped");
}

/// Gated overlay device.
#[derive(Debug)]
struct Overlay {
    ena: bool,
    mem: Ram<[u8; 0x100]>,
}

impl Overlay {
    fn enabled(&self) -> bool {
        self.ena
    }
}

impl Memory for Overlay {
    fn read(&self, addr: u16) -> rugby_arch::mem::Result<u8> {
        self.mem.read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.mem.write(addr, data)
    }
}

/// Gated memory map.
#[derive(Debug)]
#[derive(Memory)]
struct Gated {
    /// Gated overlay.
    #[mmap(0x0000..=0x00ff, gate = enabled)]
    boot: Overlay,
    /// Ungated fallback.
    #[mmap(0x0000..=0x00ff)]
    back: Ram<[u8; 0x100]>,
}

impl Gated {
    fn new() -> Self {
        Self {
            boot: Overlay {
                ena: false,
                mem: Ram::from([0x00; 0x100]),
            },
            back: Ram::from([0x00; 0x100]),
        }
    }
}

#[test]
fn gate_read_works() {
    let mut dev = Gated::new();
    dev.boot.mem.write(0x0050, 0x55).unwrap();
    dev.back.write(0x0050, 0x66).unwrap();
    // While the gate holds, the overlay is selected...
    dev.boot.ena = true;
    assert_eq!(dev.read(0x0050), Ok(0x55));
    // ... otherwise, the arm is skipped.
    dev.boot.ena = false;
    assert_eq!(dev.read(0x0050), Ok(0x66));
}

#[test]
fn gate_write_works() {
    let mut dev = Gated::new();
    dev.boot.ena = true;
    dev.write(0x0050, 0x55).unwrap();
    dev.boot.ena = false;
    dev.write(0x0050, 0x66).unwrap();
    assert_eq!(dev.boot.mem.read(0x0050), Ok(0x55));
    assert_eq!(dev.back.read(0x0050), Ok(0x66));
}

/// Overlapping memory map.
#[derive(Debug)]
#[derive(Memory)]
struct Layered {
    /// Front layer.
    #[mmap(0x0000..=0x00ff)]
    fore: Ram<[u8; 0x100]>,
    /// Back layer.
    #[mmap(0x0000..=0x01ff)]
    back: Ram<[u8; 0x200]>,
}

impl Layered {
    fn new() -> Self {
        Self {
            fore: Ram::from([0x00; 0x100]),
            back: Ram::from([0x00; 0x200]),
        }
    }
}

#[test]
fn priority_read_works() {
    let mut dev = Layered::new();
    dev.fore.write(0x0050, 0x55).unwrap();
    dev.back.write(0x0050, 0x66).unwrap();
    dev.back.write(0x0150, 0x77).unwrap();
    // Declaration order is priority order: the first match wins.
    assert_eq!(dev.read(0x0050), Ok(0x55));
    assert_eq!(dev.read(0x0150), Ok(0x77));
}

#[test]
fn priority_write_works() {
    let mut dev = Layered::new();
    dev.write(0x0050, 0x55).unwrap();
    dev.write(0x0150, 0x77).unwrap();
    assert_eq!(dev.fore.read(0x0050), Ok(0x55));
    assert_eq!(dev.back.read(0x0050), Ok(0x00));
    assert_eq!(dev.back.read(0x0150), Ok(0x77));
}
