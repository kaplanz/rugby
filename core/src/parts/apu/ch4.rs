//! Channel 4: Noise.

use std::ops::BitAnd;

use bitfield_struct::bitfield;
use log::{debug, trace};
use rugby_arch::{Block, Shared};

pub use super::reg::{Nr41, Nr42, Nr43, Nr44};

/// Sound Channel 4 - Noise.
#[derive(Debug)]
pub struct Channel {
    /// Channel output.
    pub out: f32,
    /// Channel registers.
    pub reg: Control,
    /// Channel internals.
    pub etc: Internal,
}

/// Channel 4 internals.
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
    /// Noise register.
    rng: Random,
}

/// Channel 4 envelope.
pub type Envelope = super::ch1::Envelope;

/// Linear feedback shift register.
#[bitfield(u16, order = msb)]
pub struct Random {
    /// Upper generator bit.
    #[bits(1)]
    gen_hi: bool,
    /// Upper noise bits.
    #[bits(7)]
    __: u8,
    /// Lower generator bit.
    #[bits(1)]
    gen_lo: bool,
    /// Lower noise bits.
    #[bits(5)]
    __: u8,
    /// XOR input bit (hi).
    #[bits(1)]
    xor_hi: bool,
    /// XOR input bit (lo).
    #[bits(1)]
    xor_lo: bool,
}

impl Block for Random {
    fn reset(&mut self) {
        self.0 = 0;
    }
}

impl Channel {
    /// Triggers this channel.
    pub fn trigger(&mut self) {
        log::debug!("trigger channel");

        // Extract control values
        let nr41 = *self.reg.nr41.borrow();
        let nr42 = *self.reg.nr42.borrow();

        // Clear flag
        self.reg.nr44.borrow_mut().set_trigger(false);

        // Trigger channel
        //
        // - Enable channel
        self.etc.ena = true;
        // - Reload frequency
        self.etc.clk = self.frequency_timer();
        // - Reload length timer (if expired)
        self.etc.len = 0x40 - nr41.step();
        // - Reload envelope timer
        self.etc.env.len = nr42.pace();
        // - Reload initial volume
        self.etc.env.vol = nr42.ivol();
        // - Reset LDSR
        self.etc.rng.reset();
    }

    /// Tick the channel's length timer.
    pub fn length(&mut self) {
        // If length timer is disabled, do nothing
        if !self.reg.nr44.borrow().length() {
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
        if self.reg.nr42.borrow().pace() == 0 {
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
            self.etc.env.len = self.reg.nr42.borrow_mut().pace();
        }

        // Determine envelope operation
        //
        // Either increment or decrement depending on the direction bit.
        let update = if self.reg.nr42.borrow().sign() {
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

    /// Tick the channel's randomness.
    ///
    /// Returns the bit that was shifted out.
    pub fn shift_random(&mut self) -> bool {
        let lfsr = &mut self.etc.rng;

        // Compute pseudo-random generator bit
        let rand = !(lfsr.xor_hi() ^ lfsr.xor_lo());

        // Writeback to `LFSR[15]`
        lfsr.set_gen_hi(rand);

        // Writeback to `LFSR[7]`
        //
        // NOTE: Writeback only occurs to `LFSR[7]` width is not set (configured
        //       to short mode.)
        if self.reg.nr43.borrow().width() {
            lfsr.set_gen_lo(rand);
        }

        // Shift out a bit
        lfsr.0 = lfsr.0.rotate_right(1);

        // Return shifted bit
        //
        // Due to shift performing a rotation, the shifted out bit will now be
        // the LFSR's most-significant bit.
        lfsr.gen_hi()
    }

    /// Reloads the frequency timer.
    pub fn frequency_timer(&mut self) -> u16 {
        u16::from(match self.reg.nr43.borrow().divide() {
            0 => 8,
            x => x << 4,
        }) << self.reg.nr43.borrow().shift()
    }
}

impl Block for Channel {
    fn ready(&self) -> bool {
        self.etc.ena || self.reg.nr44.borrow().trigger()
    }

    fn cycle(&mut self) {
        // Extract control values
        let nr42 = *self.reg.nr42.borrow();
        let nr44 = *self.reg.nr44.borrow();

        // Check for trigger
        if nr44.trigger() {
            self.trigger();
        }

        // Tick frequency timer
        self.etc.clk = match self.etc.clk.saturating_sub(1) {
            // On overflow, properly handle reload
            0 => {
                // Shift random bit
                self.shift_random();
                // Reload according to control
                self.frequency_timer()
            }
            // Otherwise, decrement as usual
            clk => clk,
        };

        // Compute output signal
        self.out = true
            // Only produce an output when:
            //
            // - DAC is enabled, and;
            .bitand(nr42.ivol() > 0 || nr42.sign())
            // - Channel is enabled.
            .bitand(self.etc.ena)
            //
            // If enabled, select the waveform level...
            .then(|| self.etc.rng.gen_hi())
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

/// Channel 4 registers.
#[derive(Debug)]
pub struct Control {
    /// `[$FF20]` - NR41: Channel 4 length timer.
    pub nr41: Shared<Nr41>,
    /// `[$FF21]` - NR42: Channel 4 volume & envelope.
    pub nr42: Shared<Nr42>,
    /// `[$FF22]` - NR43: Channel 4 frequency & randomness.
    pub nr43: Shared<Nr43>,
    /// `[$FF23]` - NR44: Channel 4 control.
    pub nr44: Shared<Nr44>,
}

impl Control {
    /// Instantiates a `Channel` with the provided control registers.
    #[must_use]
    pub fn with(reg: &super::Control) -> Self {
        Self {
            nr41: reg.nr41.clone(),
            nr42: reg.nr42.clone(),
            nr43: reg.nr43.clone(),
            nr44: reg.nr44.clone(),
        }
    }
}
