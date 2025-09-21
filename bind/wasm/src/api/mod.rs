//! Emulator API.

use rugby::arch::Block;
use rugby::core::dmg;
use wasm_bindgen::prelude::*;

pub mod audio;
pub mod cart;
pub mod joypad;
pub mod video;

/// Game Boy (DMG) emulator model.
#[derive(Debug, Default)]
#[wasm_bindgen(inspectable)]
pub struct GameBoy(dmg::GameBoy);

#[wasm_bindgen]
impl GameBoy {
    /// Constructs a new `GameBoy`.
    ///
    /// New instances will behave deterministically, and can be considered as
    /// being a hard reset.
    ///
    /// Use [`Self::reset`] for a soft reset.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        Self(dmg::GameBoy::new())
    }
}

#[wasm_bindgen]
impl GameBoy {
    /// Checks if the emulator is ready to be cycled.
    ///
    /// In practice, this will (almost) always return `true`. However, if the
    /// CPU is idle, this will be `false`. This can be used to indicate that no
    /// further emulation is required.
    ///
    /// This should generally be checked before calling [`Self::cycle`].
    #[must_use]
    pub fn ready(&self) -> bool {
        self.0.ready()
    }

    /// Emulates a single cycle.
    ///
    /// A single cycle refers to the smallest period of time modelled by the
    /// emulator. In this implementation, this is one quarter of the CPU's
    /// clock speed, also referred to as a T-cycle.
    ///
    /// To accurately synchronize emulated time to wall clock time, this
    /// function should be called 4,194,304 times per second (or at a frequency
    /// of 4 MiHz).
    pub fn cycle(&mut self) {
        self.0.cycle();
    }

    /// Performs a soft reset.
    ///
    /// Analogous to a reset on real hardware _without toggling power_.
    /// Critically, only some hardware registers are reset, and memory is
    /// typically left in its previous state. This can lead to changes in some
    /// undefined behaviours which are dependant on the internal hardware state.
    ///
    /// To perform a hard reset, construct a new emulator instance with
    /// [`Self::new`].
    pub fn reset(&mut self) {
        self.0.reset();
    }
}
