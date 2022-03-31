use std::ops::Deref;

pub const SCREEN: (usize, usize) = (160, 144);
const DEPTH: usize = SCREEN.0 * SCREEN.1;

pub trait Emulator {
    fn send(&mut self, btn: Button);

    fn redraw<F>(&self, draw: F)
    where
        F: FnMut(&[u32]);
}

#[derive(Debug)]
pub enum Button {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Screen([u32; DEPTH]);

impl Deref for Screen {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u32; DEPTH]> for Screen {
    fn from(buf: [u32; DEPTH]) -> Self {
        Self(buf)
    }
}
