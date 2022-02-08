use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use remus::{Block, Device};

pub use self::none::NoMbc;

mod none;

pub trait Mbc: Block + Debug {
    fn load(&mut self, rom: &[u8]);

    fn rom(&self) -> Rc<RefCell<dyn Device>>;

    fn ram(&self) -> Rc<RefCell<dyn Device>>;
}
