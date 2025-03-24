use std::cmp::Ordering;
use std::iter;

use log::{debug, warn};
use rugby_arch::Byte;

use super::{Data, Header};

/// Constructs a new ROM.
pub fn rom(head: &Header, rom: &[Byte]) -> Data {
    let read = rom.len();
    match read.cmp(&head.romsz) {
        Ordering::Less => {
            warn!(
                "loaded {init}; remaining {diff} uninitialized",
                init = bfmt::Size::from(read),
                diff = bfmt::Size::from(head.romsz - read),
            );
        }
        Ordering::Equal => debug!("loaded {read}", read = bfmt::Size::from(read)),
        Ordering::Greater => {
            warn!(
                "loaded {init}; remaining {diff} truncated",
                init = bfmt::Size::from(head.romsz),
                diff = bfmt::Size::from(read - head.romsz),
            );
        }
    }
    rom.iter()
        .copied()
        // pad missing values with open bus value
        .chain(iter::repeat(0xff))
        // truncate based on recorded cartridge size
        .take(head.romsz)
        // collect as a heap-allocated slice
        .collect::<Box<_>>()
}

/// Constructs a new RAM.
pub fn ram(head: &Header) -> Data {
    if head.info.has_ram() && head.ramsz == 0 {
        warn!("cartridge supports RAM, but specified size is zero");
    }
    if !head.info.has_ram() && head.ramsz > 0 {
        warn!(
            "cartridge does not support RAM, but specified size is non-zero (found: {})",
            head.ramsz
        );
    }
    vec![0; head.ramsz].into_boxed_slice()
}
