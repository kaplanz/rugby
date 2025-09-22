//! Emulator API.

use parking_lot::RwLock;
use rugby::arch::Block;
use rugby::core::dmg;

use crate::Result;

pub mod audio;
pub mod cart;
pub mod joypad;
pub mod video;

/// Game Boy (DMG) emulator model.
#[derive(Debug, Default, uniffi::Object)]
#[uniffi::export(Debug)]
pub struct GameBoy {
    /// Internal emulator model.
    inner: RwLock<dmg::GameBoy>,
}

#[uniffi::export]
impl GameBoy {
    /// Constructs a new `GameBoy`.
    ///
    /// New instances will behave deterministically, and can be considered as
    /// being a hard reset.
    ///
    /// Use [`Self::reset`] for a soft reset.
    #[uniffi::constructor]
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: dmg::GameBoy::new().into(),
        }
    }

    /// Constructs a new `GameBoy`, initialized with the provided boot ROM.
    ///
    /// New instances will behave deterministically, and can be considered as
    /// being a hard reset.
    ///
    /// Use [`Self::reset`] for a soft reset.
    #[uniffi::constructor]
    pub fn with(boot: &[u8]) -> Result<Self> {
        let boot = dmg::Boot::from(<[_; _]>::try_from(boot).unwrap());
        Ok(Self {
            inner: dmg::GameBoy::with(boot).into(),
        })
    }
}

#[uniffi::export]
impl GameBoy {
    /// Checks if the emulator is ready to be cycled.
    ///
    /// In practice, this will (almost) always return `true`. However, if the
    /// CPU is idle, this will be `false`. This can be used to indicate that no
    /// further emulation is required.
    ///
    /// This should generally be checked before calling [`Self::cycle`].
    #[uniffi::method]
    pub fn ready(&self) -> bool {
        self.inner.read().ready()
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
    #[uniffi::method]
    pub fn cycle(&self) {
        self.inner.write().cycle();
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
    #[uniffi::method]
    pub fn reset(&self) {
        self.inner.write().reset();
        self.inner.write().boot();
    }
}

// SAFETY: `GameBoy` does not expose its thread-unsafe internals.
//
// This invariant is respected as long as no part of this API exposes any
// internal reference-counting pointers.
unsafe impl Send for GameBoy {}

// SAFETY: `GameBoy` does not expose its thread-unsafe internals.
//
// This invariant is respected as long as no part of this API exposes any
// internal reference-counting pointers.
unsafe impl Sync for GameBoy {}
