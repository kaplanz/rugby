use std::sync::RwLock;

use rugby::arch::Block;
use rugby::core::dmg;
use rugby::prelude::*;
use uniffi::Object;

uniffi::setup_scaffolding!();

pub mod cart;
pub mod keys;

/// Game Boy model.
#[derive(Debug, Default, Object)]
pub struct GameBoy(RwLock<inner::Model>);

#[uniffi::export]
impl GameBoy {
    /// Constructs a new `GameBoy`.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self(RwLock::new(inner::Model(dmg::GameBoy::new())))
    }

    /// Checks if enabled.
    #[uniffi::method]
    pub fn ready(&self) -> bool {
        self.0.read().unwrap().ready()
    }

    /// Emulates a single cycle.
    #[uniffi::method]
    pub fn cycle(&self) {
        self.0.write().unwrap().cycle();
    }

    /// Performs a reset.
    #[uniffi::method]
    pub fn reset(&self) {
        self.0.write().unwrap().reset();
    }
}

#[uniffi::export]
impl GameBoy {
    /// Checks if the frame is ready to be rendered.
    pub fn vsync(&self) -> bool {
        self.0.read().unwrap().inside().video().vsync()
    }

    /// Retrieves the current frame state.
    pub fn frame(&self) -> Vec<u8> {
        self.0
            .read()
            .unwrap()
            .inside()
            .video()
            .frame()
            .iter()
            .map(|&pix| pix as u8)
            .collect::<Vec<u8>>()
    }
}

/// Private model implementation.
mod inner {
    use std::ops::{Deref, DerefMut};

    use rugby::core::dmg::GameBoy;

    /// Inner emulator model.
    #[derive(Debug, Default)]
    pub struct Model(pub(crate) GameBoy);

    impl Deref for Model {
        type Target = GameBoy;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Model {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    // SAFETY: `GameBoy` does not expose its thread-unsafe internals.
    unsafe impl Send for Model {}

    // SAFETY: `GameBoy` does not expose its thread-unsafe internals.
    unsafe impl Sync for Model {}
}
