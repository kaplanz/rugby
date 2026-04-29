//! Channel 3: Wave.

use std::ops::BitAnd;

use log::{debug, trace};
use rugby_arch::mem::Memory;
use rugby_arch::{Block, Shared};

use super::Bank;
pub use super::reg::{Nr30, Nr31, Nr32, Nr33, Nr34};

/// Sound Channel 3 - Wave output.
#[derive(Debug)]
pub struct Channel {
    /// Channel output.
    pub out: f32,
    /// Channel registers.
    pub reg: Control,
    /// Channel memory.
    pub mem: Bank,
    /// Channel internals.
    pub etc: Internal,
}

/// Channel 3 internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// Channel enabled.
    ena: bool,
    /// Frequency timer. (11-bit)
    clk: u16,
    /// Length timer. (8-bit)
    len: u16,
    /// Sample index counter. (5-bit)
    idx: u8,
}

impl Channel {
    /// Triggers this channel.
    pub fn trigger(&mut self) {
        log::debug!("trigger channel");

        // Extract control values
        let nr31 = *self.reg.nr31.borrow();
        let nr33 = *self.reg.nr33.borrow();
        let nr34 = *self.reg.nr34.borrow();

        // Clear flag
        self.reg.nr34.borrow_mut().set_trigger(false);

        // Trigger channel
        //
        // - Enable channel
        self.etc.ena = true;
        // - Reload frequency
        self.etc.clk = u16::from_le_bytes([nr33.clk_lo(), nr34.clk_hi()]);
        // - Reload length timer (if expired)
        self.etc.len = 0x100 - u16::from(nr31.step());
    }

    /// Tick the channel's length timer.
    pub fn length(&mut self) {
        // If length timer is disabled, do nothing
        if !self.reg.nr34.borrow().length() {
            return;
        }

        // Decrement the length timer
        let Some(len) = self.etc.len.checked_sub(1) else {
            // If the length timer has already reached zero, the channel should
            // already be disabled, so return immediately.
            return;
        };

        // While still non-zero, simply decrement the length timer.
        trace!("tick length timer: {len}");
        self.etc.len = len;

        // If the length timer has just reached zero, disable the channel.
        if len == 0 {
            debug!("disable: length timeout");
            self.etc.ena = false;
        }
    }
}

impl Block for Channel {
    fn ready(&self) -> bool {
        self.etc.ena || self.reg.nr34.borrow().trigger()
    }

    fn cycle(&mut self) {
        // Extract control values
        let nr30 = *self.reg.nr30.borrow();
        let nr32 = *self.reg.nr32.borrow();
        let nr33 = *self.reg.nr33.borrow();
        let nr34 = *self.reg.nr34.borrow();

        // Check for trigger
        if nr34.trigger() {
            self.trigger();
        }

        // Tick frequency timer
        self.etc.clk = match self.etc.clk.saturating_add(1) {
            // On overflow, properly handle reload
            0x800.. => {
                // Update waveform position
                self.etc.idx = self.etc.idx.wrapping_add(1) & 0x1f;
                // Reload from control values
                u16::from_le_bytes([nr33.clk_lo(), nr34.clk_hi()])
            }
            // Otherwise, increment as usual
            clk => clk,
        };

        // Compute output signal
        self.out = true
            // Only produce an output when:
            //
            // - DAC is enabled, and;
            .bitand(nr30.dac())
            // - Channel is enabled.
            .bitand(self.etc.ena)
            //
            // If enabled, select the waveform pattern...
            .then(|| {
                // Extract waveform byte
                let byte = self.mem.wave.read(u16::from(self.etc.idx >> 1)).unwrap();
                // Extract waveform nibble (big-endian)
                match self.etc.idx & 0b1 {
                    0 => (byte & 0xf0) >> 4,
                    1 => (byte & 0x0f),
                    _ => unreachable!(),
                }
            })
            // ... then scale the output level.
            //
            // NOTE: This modulo operation is a hack to implement the output
            //       volume table, which defines a right-shift to the 4-bit
            //       waveform pattern according to the table below.
            //
            // | Level | Shift | Volume |
            // |-------|-------|--------|
            // | `00`  |   4   |     0% |
            // | `01`  |   0   |   100% |
            // | `10`  |   1   |    50% |
            // | `11`  |   2   |    25% |
            .map(|lvl| lvl >> ((4 + nr32.vol()) % 5))
            // Use DAC to convert the volume to a 32-bit pulse-code modulated
            // floating point value that is linearly scaled between -1 and 1.
            .map(|out| (f32::from(out) / 7.5) - 1.)
            // Otherwise (if disabled), the channel will produce a zero signal.
            .unwrap_or_default();
    }

    fn reset(&mut self) {
        std::mem::take(&mut self.etc);
    }
}

/// Channel 3 registers.
#[derive(Debug)]
pub struct Control {
    /// `[$FF1A]` - NR30: Channel 3 DAC enable.
    pub nr30: Shared<Nr30>,
    /// `[$FF1B]` - NR31: Channel 3 length timer.
    pub nr31: Shared<Nr31>,
    /// `[$FF1C]` - NR32: Channel 3 output level.
    pub nr32: Shared<Nr32>,
    /// `[$FF1D]` - NR33: Channel 3 period low.
    pub nr33: Shared<Nr33>,
    /// `[$FF1E]` - NR34: Channel 3 period high & control.
    pub nr34: Shared<Nr34>,
}

impl Control {
    /// Instantiates a `Channel` with the provided control registers.
    #[must_use]
    pub fn with(reg: &super::Control) -> Self {
        Self {
            nr30: reg.nr30.clone(),
            nr31: reg.nr31.clone(),
            nr32: reg.nr32.clone(),
            nr33: reg.nr33.clone(),
            nr34: reg.nr34.clone(),
        }
    }
}
