//! Channel 2: Square.

use std::ops::BitAnd;

use log::{debug, trace};
use rugby_arch::{Block, Shared};

pub use super::ch1::WAVE;
pub use super::reg::{Nr21, Nr22, Nr23, Nr24};

/// Sound Channel 2 - Pulse.
#[derive(Debug)]
pub struct Channel {
    /// Channel output.
    pub out: f32,
    /// Channel registers.
    pub reg: Control,
    /// Channel internals.
    pub etc: Internal,
}

/// Channel 2 internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// Channel enabled.
    ena: bool,
    /// Frequency timer. (11-bit)
    clk: u16,
    /// Length timer. (6-bit)
    len: u8,
    /// Volume envelope.
    env: Envelope,
    /// Waveform position pointer.
    pos: u8,
}

/// Channel 2 envelope.
pub type Envelope = super::ch1::Envelope;

impl Channel {
    /// Triggers this channel.
    pub fn trigger(&mut self) {
        log::debug!("trigger channel");

        // Extract control values
        let nr21 = *self.reg.nr21.borrow();
        let nr22 = *self.reg.nr22.borrow();
        let nr23 = *self.reg.nr23.borrow();
        let nr24 = *self.reg.nr24.borrow();

        // Clear flag
        self.reg.nr24.borrow_mut().set_trigger(false);

        // Trigger channel
        //
        // - Enable channel
        self.etc.ena = true;
        // - Reload frequency
        self.etc.clk = u16::from_le_bytes([nr23.clk_lo(), nr24.clk_hi()]);
        // - Reload length timer (if expired)
        self.etc.len = 0x40 - nr21.step();
        // - Reload envelope timer
        self.etc.env.len = nr22.pace();
        // - Reload initial volume
        self.etc.env.vol = nr22.ivol();
        // - Validate position pointer
        if self.etc.pos == 0 {
            // After a reset, the position pointer must be initialized to the
            // most-significant bit of the waveform pattern.
            self.etc.pos = 0b10000000;
        }
    }

    /// Tick the channel's length timer.
    pub fn length(&mut self) {
        // If length timer is disabled, do nothing
        if !self.reg.nr24.borrow().length() {
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

    /// Tick the channel's envelope.
    pub fn volume(&mut self) {
        // If volume envelope is disabled, do nothing
        if self.reg.nr22.borrow().pace() == 0 {
            return;
        }

        // A envelope iteration only happens when the envelope timer reaches
        // zero, otherwise, the volume is not modified.
        #[expect(clippy::redundant_else)]
        if let Some(len @ 1..) = self.etc.env.len.checked_sub(1) {
            // While still non-zero, simply decrement the sweep timer without
            // re-calculating or updating the frequency.
            trace!("tick envelope timer: {len}");
            self.etc.env.len = len;
            return;
        } else {
            // When zero is reached, re-initialize the envelope timer.
            self.etc.env.len = self.reg.nr22.borrow_mut().pace();
        }

        // Determine envelope operation
        //
        // Either increment or decrement depending on the direction bit.
        let update = if self.reg.nr22.borrow().sign() {
            |vol| u8::saturating_add(vol, 1)
        } else {
            |vol| u8::saturating_sub(vol, 1)
        };

        // Compute updated volume
        let vol = update(self.etc.env.vol)
            // retain saturated 4-bit value
            .clamp(0b0000, 0b1111);

        // Write updated volume
        self.etc.env.vol = vol;
        trace!("update volume: {vol}");
    }
}

impl Block for Channel {
    fn ready(&self) -> bool {
        self.etc.ena || self.reg.nr24.borrow().trigger()
    }

    fn cycle(&mut self) {
        // Extract control values
        let nr21 = *self.reg.nr21.borrow();
        let nr22 = *self.reg.nr22.borrow();
        let nr23 = *self.reg.nr23.borrow();
        let nr24 = *self.reg.nr24.borrow();

        // Check for trigger
        if nr24.trigger() {
            self.trigger();
        }

        // Tick frequency timer
        self.etc.clk = match self.etc.clk.saturating_add(1) {
            // On overflow, properly handle reload
            0x800.. => {
                // Update waveform position
                self.etc.pos = self.etc.pos.rotate_right(1);
                // Reload from control values
                u16::from_le_bytes([nr23.clk_lo(), nr24.clk_hi()])
            }
            // Otherwise, increment as usual
            clk => clk,
        };

        // Compute output signal
        self.out = true
            // Only produce an output when:
            //
            // - DAC is enabled, and;
            .bitand(nr22.ivol() > 0 || nr22.sign())
            // - Channel is enabled.
            .bitand(self.etc.ena)
            //
            // If enabled, select the waveform level...
            .then(|| (WAVE[nr21.duty()] & self.etc.pos) != 0)
            // ... then convert it into a volume level.
            .map(|lvl| u8::from(lvl) * self.etc.env.vol)
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

/// Channel 2 registers.
#[derive(Debug)]
pub struct Control {
    /// `[$FF16]` - NR21: Channel 2 length timer & duty cycle.
    pub nr21: Shared<Nr21>,
    /// `[$FF17]` - NR22: Channel 2 volume & envelope.
    pub nr22: Shared<Nr22>,
    /// `[$FF18]` - NR23: Channel 2 period low.
    pub nr23: Shared<Nr23>,
    /// `[$FF19]` - NR24: Channel 2 period high & control.
    pub nr24: Shared<Nr24>,
}

impl Control {
    /// Instantiates a `Channel` with the provided control registers.
    #[must_use]
    pub fn with(reg: &super::Control) -> Self {
        Self {
            nr21: reg.nr21.clone(),
            nr22: reg.nr22.clone(),
            nr23: reg.nr23.clone(),
            nr24: reg.nr24.clone(),
        }
    }
}
