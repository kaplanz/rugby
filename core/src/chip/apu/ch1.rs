//! Channel 1: Square.

use std::ops::BitAnd;

use log::{debug, trace};
use rugby_arch::{Block, Shared};

pub use super::{Nr10, Nr11, Nr12, Nr13, Nr14};

/// Output waveform duty patterns.
///
/// Specifies a pattern for the output waveform's duty cycle.
pub const WAVE: [u8; 4] = [
    0b00000001, // 12.5% (active 1/8)
    0b10000001, // 25.0% (active 2/8)
    0b10000111, // 50.0% (active 4/8)
    0b01111110, // 75.0% (active 6/8)
];

/// Sound Channel 1 - Pulse with period sweep.
#[derive(Debug)]
pub struct Channel {
    /// Channel output.
    pub out: f32,
    /// Channel registers.
    pub reg: Control,
    /// Channel internals.
    pub etc: Internal,
}

/// Channel 1 internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// Channel enabled.
    ena: bool,
    /// Frequency timer. (11-bit)
    clk: u16,
    /// Frequency sweep.
    swp: Sweep,
    /// Length timer. (6-bit)
    len: u8,
    /// Volume envelope.
    env: Envelope,
    /// Waveform position pointer.
    pos: u8,
}

/// Channel 1 sweep.
#[derive(Debug, Default)]
pub struct Sweep {
    /// Sweep enabled.
    ///
    /// Compute enabled if sweep period or sweep shift is non-zero.
    ena: bool,
    /// Shadow frequency. (11-bit)
    ///
    /// Reloaded from `{NR14[2:0], NR13[7:0]}`
    frq: u16,
    /// Sweep timer. (3-bit)
    ///
    /// Reloaded from `NR10[6:4]`
    len: u8,
}

/// Channel 1 envelope.
#[derive(Debug, Default)]
pub struct Envelope {
    /// Envelope timer. (4-bit)
    pub(super) len: u8,
    /// Current volume. (4-bit)
    pub(super) vol: u8,
}

impl Channel {
    /// Triggers this channel.
    pub fn trigger(&mut self) {
        log::debug!("trigger channel");

        // Extract control values
        let nr10 = *self.reg.nr10.borrow();
        let nr11 = *self.reg.nr11.borrow();
        let nr12 = *self.reg.nr12.borrow();
        let nr13 = *self.reg.nr13.borrow();
        let nr14 = *self.reg.nr14.borrow();

        // Clear flag
        self.reg.nr14.borrow_mut().set_trigger(false);

        // Trigger channel
        //
        // - Enable channel
        self.etc.ena = true;
        // - Reload sweep enable
        let (has_pace, has_step) = (nr10.pace() != 0, nr10.step() != 0);
        self.etc.swp.ena = has_pace || has_step;
        // - Reload sweep timer
        self.etc.swp.len = match nr10.pace() {
            0 => 8, // if zero, reload with 8
            x => x, // else, use stored value
        };
        // - Reload sweep frequency
        self.etc.swp.frq = u16::from_le_bytes([nr13.clk_lo(), nr14.clk_hi()]);
        // - Reload frequency
        self.etc.clk = u16::from_le_bytes([nr13.clk_lo(), nr14.clk_hi()]);
        // - Reload length timer (if expired)
        self.etc.len = 0x40 - nr11.step();
        // - Reload envelope timer
        self.etc.env.len = nr12.pace();
        // - Reload initial volume
        self.etc.env.vol = nr12.ivol();
        // - Validate position pointer
        if self.etc.pos == 0 {
            // After a reset, the position pointer must be initialized to the
            // most-significant bit of the waveform pattern.
            self.etc.pos = 0b10000000;
        }
        // - Perform overflow check
        if nr10.step() != 0 {
            // If the individual step is non-zero, frequency calculation and
            // overflow check are performed immediately.
            self.overflow_check();
        }
    }

    /// Tick the channel's length timer.
    pub fn length(&mut self) {
        // If length timer is disabled, do nothing
        if !self.reg.nr14.borrow().length() {
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
        if self.reg.nr12.borrow().pace() == 0 {
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
            self.etc.env.len = self.reg.nr12.borrow_mut().pace();
        }

        // Determine envelope operation
        //
        // Either increment or decrement depending on the direction bit.
        let update = if self.reg.nr12.borrow().sign() {
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

    /// Tick the channel's frequency sweep.
    ///
    /// # Note
    ///
    /// This is also called a period sweep. If configured, channel 1 will
    /// linearly increase or decrease its frequency.
    pub fn sweep(&mut self) {
        // A sweep iteration only happens when the sweep timer reaches zero,
        // otherwise, the frequency is not modified.
        #[expect(clippy::redundant_else)]
        if let Some(len @ 1..) = self.etc.swp.len.checked_sub(1) {
            // While still counting, simply decrement the sweep timer without
            // re-calculating or updating the frequency.
            trace!("tick sweep timer: {len}");
            self.etc.swp.len = len;
            return;
        } else {
            // When zero is reached, re-initialize the sweep timer.
            self.etc.swp.len = match self.reg.nr10.borrow().pace() {
                0 => 8, // if zero, reload with 8
                x => x, // else, use stored value
            }
        }

        // If the sweep unit is disabled, the period sweep is disabled.
        if !self.etc.swp.ena {
            return;
        }
        // If the sweep pace is zero, the period sweep is disabled.
        if self.reg.nr10.borrow().pace() == 0 {
            return;
        }

        // Period sweep is enabled, so perform the frequency calculation.
        let Some(frq) = self.overflow_check() else {
            return;
        };

        // If the sweep shift is zero, don't perform an update.
        if self.reg.nr10.borrow().step() == 0 {
            trace!("sweep ignored");
            return;
        }

        // Write shadow register
        self.etc.swp.frq = frq;
        // Write control registers
        let [lo, hi] = frq.to_le_bytes();
        self.reg.nr13.borrow_mut().set_clk_lo(lo);
        self.reg.nr14.borrow_mut().set_clk_hi(hi);

        // Perform final overflow check
        self.overflow_check();
    }

    /// Perform the frequency calculation and overflow check, potentially
    /// disabling the channel.
    pub fn overflow_check(&mut self) -> Option<u16> {
        // Extract control values
        let nr10 = *self.reg.nr10.borrow();

        // Calculate frequency step
        let step = self.etc.swp.frq >> nr10.step();
        // Determine frequency direction
        let compute = if nr10.sign() {
            u16::saturating_sub
        } else {
            u16::saturating_add
        };

        // Compute updated frequency
        let frq = compute(self.etc.swp.frq, step);
        trace!(
            "compute sweep: {old:#05x} -> {frq:#05x} ({sign}, shift: {size})",
            old = self.etc.swp.frq,
            sign = if nr10.sign() { "sub" } else { "add" },
            size = nr10.step(),
        );

        // Check if frequency has overflowed
        if frq >= 0x800 {
            // Disable the channel
            debug!("disable: sweep overflow");
            self.etc.ena = false;
            // Don't return frequency
            None
        } else {
            // Return new frequency
            Some(frq)
        }
    }
}

impl Block for Channel {
    fn ready(&self) -> bool {
        self.etc.ena || self.reg.nr14.borrow().trigger()
    }

    fn cycle(&mut self) {
        // Extract control values
        let nr11 = *self.reg.nr11.borrow();
        let nr12 = *self.reg.nr12.borrow();
        let nr13 = *self.reg.nr13.borrow();
        let nr14 = *self.reg.nr14.borrow();

        // Check for trigger
        if nr14.trigger() {
            self.trigger();
        }

        // Tick frequency timer
        self.etc.clk = match self.etc.clk.wrapping_add(1) {
            // On overflow, properly handle reload
            0x800.. => {
                // Update waveform position
                self.etc.pos = self.etc.pos.rotate_right(1);
                // Reload from control values
                u16::from_le_bytes([nr13.clk_lo(), nr14.clk_hi()])
            }
            // Otherwise, increment as usual
            x => x,
        };

        // Compute output signal
        self.out = true
            // Only produce an output when:
            //
            // - DAC is enabled, and;
            .bitand(nr12.ivol() > 0 || nr12.sign())
            // - Channel is enabled.
            .bitand(self.etc.ena)
            //
            // If enabled, select the waveform level...
            .then(|| (WAVE[nr11.duty()] & self.etc.pos) != 0)
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

/// Channel 1 registers.
#[derive(Debug)]
pub struct Control {
    /// `[$FF10]` - NR10: Channel 1 sweep.
    pub nr10: Shared<Nr10>,
    /// `[$FF11]` - NR11: Channel 1 length timer & duty cycle.
    pub nr11: Shared<Nr11>,
    /// `[$FF12]` - NR12: Channel 1 volume & envelope.
    pub nr12: Shared<Nr12>,
    /// `[$FF13]` - NR13: Channel 1 period low.
    pub nr13: Shared<Nr13>,
    /// `[$FF14]` - NR14: Channel 1 period high & control.
    pub nr14: Shared<Nr14>,
}

impl Control {
    /// Instantiates a `Channel` with the provided control registers.
    #[must_use]
    pub fn with(reg: &super::Control) -> Self {
        Self {
            nr10: reg.nr10.clone(),
            nr11: reg.nr11.clone(),
            nr12: reg.nr12.clone(),
            nr13: reg.nr13.clone(),
            nr14: reg.nr14.clone(),
        }
    }
}
