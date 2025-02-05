//! Audio model.

use rugby_arch::mem::Ram;
use rugby_arch::mio::{Bus, Mmio};
use rugby_arch::reg::{Port, Register};
use rugby_arch::{Block, Byte, Shared};

use super::timer;
use crate::api::part::audio::{Audio as Api, Sample};

mod reg;

pub mod ch1;
pub mod ch2;
pub mod ch3;
pub mod ch4;

pub use self::reg::*;

/// Wave RAM.
///
/// 16 byte RAM used to store audio samples. See more details [here][wave].
///
/// [wave]: https://gbdev.io/pandocs/Audio_Registers.html#ff30ff3f--wave-pattern-ram
pub type Wave = Ram<[Byte; 0x0010]>;

/// Audio sequencer.
///
/// # Note
///
/// Also referred to as the frame sequencer, this determines how often each
/// channel's features should be clocked. It is driven by [timer], specifically
/// the falling edge of the [divider register](timer::reg::Div)'s bit 4.
#[derive(Debug)]
pub struct Sequencer {
    /// `DIV[4]` last measured value.
    ///
    /// Used to detect the falling edge.
    pub bit: bool,
    /// `DIV-APU` sequence value.
    ///
    /// This will tick at approx. 512 Hz, with extra ticks potentially
    /// occurring when the timer's divider is reset.
    pub clk: u8,
    /// Clock driver.
    ///
    /// Uses `DIV[4]` from the [timer].
    pub div: Shared<timer::reg::Div>,
}

impl Sequencer {
    /// Clocking bit to test.
    ///
    /// APU will clock a cycle whenever the masked bit falls.
    const MASK: u8 = 0b0001_0000;
}

impl Block for Sequencer {
    fn ready(&self) -> bool {
        // Check for falling edge
        let next = self.div.load() & Self::MASK != 0;
        !self.bit && next
    }

    fn cycle(&mut self) {
        // Fetch current clock bit
        let next = self.div.load() & Self::MASK != 0;
        // Check for falling edge
        if !self.bit && next {
            // Only tick the internal clock value on the falling edge of the
            // timer's divider.
            self.clk = self.clk.wrapping_add(1);
        }
        // Store current clock bit
        self.bit = next;
    }
}

/// Audio register select.
///
/// See more details [here][regs].
///
/// [regs]: https://gbdev.io/pandocs/Audio_Registers.html
#[derive(Clone, Copy, Debug)]
pub enum Select {
    /// `[$FF26]`: Audio master control.
    Nr52,
    /// `[$FF25]`: Sound panning.
    Nr51,
    /// `[$FF24]`: Master volume & VIN panning.
    Nr50,
    /// `[$FF10]`: CH1 period sweep.
    Nr10,
    /// `[$FF11]`: CH1 length timer & duty cycle.
    Nr11,
    /// `[$FF12]`: CH1 volume & envelope.
    Nr12,
    /// `[$FF13]`: CH1 period low.
    Nr13,
    /// `[$FF14]`: CH1 period high & control.
    Nr14,
    /// `[$FF16]`: CH2 length timer & duty cycle.
    Nr21,
    /// `[$FF17]`: CH2 volume & envelope.
    Nr22,
    /// `[$FF18]`: CH2 period low.
    Nr23,
    /// `[$FF19]`: CH2 period high & control.
    Nr24,
    /// `[$FF1A]`: CH3 DAC enable.
    Nr30,
    /// `[$FF1B]`: CH3 length timer.
    Nr31,
    /// `[$FF1C]`: CH3 output level.
    Nr32,
    /// `[$FF1D]`: CH3 period low.
    Nr33,
    /// `[$FF1E]`: CH3 period high & control.
    Nr34,
    /// `[$FF20]`: CH4 length timer.
    Nr41,
    /// `[$FF21]`: CH4 volume & envelope.
    Nr42,
    /// `[$FF22]`: CH4 frequency & randomness.
    Nr43,
    /// `[$FF23]`: CH4 control.
    Nr44,
}

/// Audio processing unit.
#[derive(Debug)]
pub struct Apu {
    /// Audio registers.
    pub reg: Control,
    /// Audio memory.
    pub mem: Bank,
    /// Channel 1.
    pub ch1: ch1::Channel,
    /// Channel 2.
    pub ch2: ch2::Channel,
    /// Channel 3.
    pub ch3: ch3::Channel,
    /// Channel 4.
    pub ch4: ch4::Channel,
    /// Audio sequencer.
    pub seq: Sequencer,
    /// Audio internals.
    pub etc: Internal,
}

/// Audio internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// Master clock divider.
    ///
    /// While the APU is externally clocked at 4 MiHz, channels are clocked at
    /// either 1 MiHz (CH1, CH2, CH4) or 2 MiHz (CH3).
    div: u8,
}

impl Api for Apu {
    fn sample(&self) -> Sample {
        // Extract control values
        let nr50 = *self.reg.nr50.borrow();
        let nr51 = *self.reg.nr51.borrow();

        // Mix each channel's sample
        let ch1 = Sample {
            lt: nr51.ch1_l().then_some(self.ch1.out).unwrap_or_default(),
            rt: nr51.ch1_r().then_some(self.ch1.out).unwrap_or_default(),
        };
        let ch2 = Sample {
            lt: nr51.ch2_l().then_some(self.ch2.out).unwrap_or_default(),
            rt: nr51.ch2_r().then_some(self.ch2.out).unwrap_or_default(),
        };
        let ch3 = Sample {
            lt: nr51.ch3_l().then_some(self.ch3.out).unwrap_or_default(),
            rt: nr51.ch3_r().then_some(self.ch3.out).unwrap_or_default(),
        };
        let ch4 = Sample {
            lt: nr51.ch4_l().then_some(self.ch4.out).unwrap_or_default(),
            rt: nr51.ch4_r().then_some(self.ch4.out).unwrap_or_default(),
        };

        // Combine mixed channels
        let mix = [ch1, ch2, ch3, ch4].into_iter().sum::<Sample>() / 4.;

        // Scale channel volumes
        Sample {
            lt: mix.lt * (f32::from(nr50.vol_l()) / 7.),
            rt: mix.rt * (f32::from(nr50.vol_r()) / 7.),
        }
    }
}

impl Block for Apu {
    fn cycle(&mut self) {
        // Check if the APU is enabled before clocking.
        //
        // Most of the APU's functionality isn't powered when disabled. However,
        // some components (namely the NR52 register and the sequencer) are
        // always active.
        if self.reg.nr52.borrow().enable() {
            // Frame sequencer
            //
            // Ticks at around 512 Hz. Used to sequence channel features.
            if self.seq.ready() {
                // Length: 256 Hz
                //
                // Ticks every even cycle.
                if self.seq.clk & 0b001 == 0b000 {
                    // Tick length timers
                    self.ch1.length();
                    self.ch2.length();
                    self.ch3.length();
                    self.ch4.length();
                }
                // Envelope: 64 Hz
                //
                // Ticks every 8 cycles, offset by 7.
                if self.seq.clk & 0b111 == 0b111 {
                    // Tick volume envelope
                    self.ch1.volume();
                    self.ch2.volume();
                    self.ch4.volume();
                }
                // CH1 Freq: 128 Hz
                //
                // Ticks every 4 cycles, offset by 2.
                if self.seq.clk & 0b011 == 0b010 {
                    // Tick frequency sweep
                    self.ch1.sweep();
                }
            }

            // Cycle channels
            //
            // Channel 1: 1 MiHz
            if self.ch1.ready() && self.etc.div % 4 == 0 {
                self.ch1.cycle();
            }
            // Channel 2: 1 MiHz
            if self.ch2.ready() && self.etc.div % 4 == 0 {
                self.ch2.cycle();
            }
            // Channel 3: 2 MiHz
            if self.ch3.ready() && self.etc.div % 2 == 0 {
                self.ch3.cycle();
            }
            // Channel 4: 1 MiHz
            if self.ch4.ready() && self.etc.div % 4 == 0 {
                self.ch4.cycle();
            }
        } else {
            // When disabled, all registers are reset and in read-only mode. We
            // can emulate this by constantly resetting these components.
            self.reset();
        }

        // Cycle frame sequencer
        //
        // This ensures we can detect falling edges as they occur. The frame
        // sequencer is always cycled, even while the APU is disabled.
        self.seq.cycle();

        // Update channel status
        let mut nr52 = self.reg.nr52.borrow_mut();
        nr52.set_ch1_on(self.ch1.ready());
        nr52.set_ch2_on(self.ch2.ready());
        nr52.set_ch3_on(self.ch3.ready());
        nr52.set_ch4_on(self.ch4.ready());

        // Cycle internal clock divider
        self.etc.div = self.etc.div.wrapping_add(1);
    }

    fn reset(&mut self) {
        self.ch1.reset();
        self.ch2.reset();
        self.ch3.reset();
        self.ch4.reset();
        self.reg.reset();
    }
}

impl Mmio for Apu {
    fn attach(&self, bus: &mut Bus) {
        self.reg.attach(bus);
    }
}

impl Port<Byte> for Apu {
    type Select = Select;

    fn load(&self, reg: Self::Select) -> Byte {
        match reg {
            Select::Nr52 => self.reg.nr52.load(),
            Select::Nr51 => self.reg.nr51.load(),
            Select::Nr50 => self.reg.nr50.load(),
            Select::Nr10 => self.reg.nr10.load(),
            Select::Nr11 => self.reg.nr11.load(),
            Select::Nr12 => self.reg.nr12.load(),
            Select::Nr13 => self.reg.nr13.load(),
            Select::Nr14 => self.reg.nr14.load(),
            Select::Nr21 => self.reg.nr21.load(),
            Select::Nr22 => self.reg.nr22.load(),
            Select::Nr23 => self.reg.nr23.load(),
            Select::Nr24 => self.reg.nr24.load(),
            Select::Nr30 => self.reg.nr30.load(),
            Select::Nr31 => self.reg.nr31.load(),
            Select::Nr32 => self.reg.nr32.load(),
            Select::Nr33 => self.reg.nr33.load(),
            Select::Nr34 => self.reg.nr34.load(),
            Select::Nr41 => self.reg.nr41.load(),
            Select::Nr42 => self.reg.nr42.load(),
            Select::Nr43 => self.reg.nr43.load(),
            Select::Nr44 => self.reg.nr44.load(),
        }
    }

    fn store(&mut self, reg: Self::Select, value: Byte) {
        match reg {
            Select::Nr52 => self.reg.nr52.store(value),
            Select::Nr51 => self.reg.nr51.store(value),
            Select::Nr50 => self.reg.nr50.store(value),
            Select::Nr10 => self.reg.nr10.store(value),
            Select::Nr11 => self.reg.nr11.store(value),
            Select::Nr12 => self.reg.nr12.store(value),
            Select::Nr13 => self.reg.nr13.store(value),
            Select::Nr14 => self.reg.nr14.store(value),
            Select::Nr21 => self.reg.nr21.store(value),
            Select::Nr22 => self.reg.nr22.store(value),
            Select::Nr23 => self.reg.nr23.store(value),
            Select::Nr24 => self.reg.nr24.store(value),
            Select::Nr30 => self.reg.nr30.store(value),
            Select::Nr31 => self.reg.nr31.store(value),
            Select::Nr32 => self.reg.nr32.store(value),
            Select::Nr33 => self.reg.nr33.store(value),
            Select::Nr34 => self.reg.nr34.store(value),
            Select::Nr41 => self.reg.nr41.store(value),
            Select::Nr42 => self.reg.nr42.store(value),
            Select::Nr43 => self.reg.nr43.store(value),
            Select::Nr44 => self.reg.nr44.store(value),
        }
    }
}

/// Audio memory.
///
/// |     Address     | Size | Name | Description |
/// |:---------------:|------|------|-------------|
/// | `$FF30..=$FF3F` | 16 B | WAVE | Wave RAM    |
#[derive(Clone, Debug, Default)]
pub struct Bank {
    /// Wave RAM.
    pub wave: Shared<Wave>,
}

/// Audio registers.
///
/// | Address | Size | Name | Description                   |
/// |:-------:|------|------|-------------------------------|
/// | `$FF26` | Byte | NR52 | Audio master control          |
/// | `$FF25` | Byte | NR51 | Sound panning                 |
/// | `$FF24` | Byte | NR50 | Master volume & VIN panning   |
/// | `$FF10` | Byte | NR10 | CH1 period sweep              |
/// | `$FF11` | Byte | NR11 | CH1 length timer & duty cycle |
/// | `$FF12` | Byte | NR12 | CH1 volume & envelope         |
/// | `$FF13` | Byte | NR13 | CH1 period low                |
/// | `$FF14` | Byte | NR14 | CH1 period high & control     |
/// | `$FF16` | Byte | NR21 | CH2 length timer & duty cycle |
/// | `$FF17` | Byte | NR22 | CH2 volume & envelope         |
/// | `$FF18` | Byte | NR23 | CH2 period low                |
/// | `$FF19` | Byte | NR24 | CH2 period high & control     |
/// | `$FF1A` | Byte | NR30 | CH3 DAC enable                |
/// | `$FF1B` | Byte | NR31 | CH3 length timer              |
/// | `$FF1C` | Byte | NR32 | CH3 output level              |
/// | `$FF1D` | Byte | NR33 | CH3 period low                |
/// | `$FF1E` | Byte | NR34 | CH3 period high & control     |
/// | `$FF20` | Byte | NR41 | CH4 length timer              |
/// | `$FF21` | Byte | NR42 | CH4 volume & envelope         |
/// | `$FF22` | Byte | NR43 | CH4 frequency & randomness    |
/// | `$FF23` | Byte | NR44 | CH4 control                   |
///
/// [regs]: https://gbdev.io/pandocs/Audio_Registers.html
#[derive(Debug, Default)]
pub struct Control {
    /// Audio master control.
    pub nr52: Shared<Nr52>,
    /// Sound panning.
    pub nr51: Shared<Nr51>,
    /// Master volume & VIN panning.
    pub nr50: Shared<Nr50>,
    /// CH1 period sweep.
    pub nr10: Shared<Nr10>,
    /// CH1 length timer & duty cycle.
    pub nr11: Shared<Nr11>,
    /// CH1 volume & envelope.
    pub nr12: Shared<Nr12>,
    /// CH1 period low.
    pub nr13: Shared<Nr13>,
    /// CH1 period high & control.
    pub nr14: Shared<Nr14>,
    /// CH2 length timer & duty cycle.
    pub nr21: Shared<Nr21>,
    /// CH2 volume & envelope.
    pub nr22: Shared<Nr22>,
    /// CH2 period low.
    pub nr23: Shared<Nr23>,
    /// CH2 period high & control.
    pub nr24: Shared<Nr24>,
    /// CH3 DAC enable.
    pub nr30: Shared<Nr30>,
    /// CH3 length timer.
    pub nr31: Shared<Nr31>,
    /// CH3 output level.
    pub nr32: Shared<Nr32>,
    /// CH3 period low.
    pub nr33: Shared<Nr33>,
    /// CH3 period high & control.
    pub nr34: Shared<Nr34>,
    /// CH4 length timer.
    pub nr41: Shared<Nr41>,
    /// CH4 volume & envelope.
    pub nr42: Shared<Nr42>,
    /// CH4 frequency & randomness.
    pub nr43: Shared<Nr43>,
    /// CH4 control.
    pub nr44: Shared<Nr44>,
}

impl Block for Control {
    fn reset(&mut self) {
        // Global Control Registers
        self.nr52.take();
        self.nr51.take();
        self.nr50.take();
        // Sound Channel 1 — Pulse with wavelength sweep
        self.nr10.take();
        self.nr11.take();
        self.nr12.take();
        self.nr13.take();
        self.nr14.take();
        // Sound Channel 2 — Pulse
        self.nr21.take();
        self.nr22.take();
        self.nr23.take();
        self.nr24.take();
        // Sound Channel 3 — Wave output
        self.nr30.take();
        self.nr31.take();
        self.nr32.take();
        self.nr33.take();
        self.nr34.take();
        // Sound Channel 4 — Noise
        self.nr41.take();
        self.nr42.take();
        self.nr43.take();
        self.nr44.take();
    }
}

impl Mmio for Control {
    fn attach(&self, bus: &mut Bus) {
        // Global Control Registers
        bus.map(0xff26..=0xff26, self.nr52.clone().into());
        bus.map(0xff25..=0xff25, self.nr51.clone().into());
        bus.map(0xff24..=0xff24, self.nr50.clone().into());
        // Sound Channel 1 — Pulse with wavelength sweep
        bus.map(0xff10..=0xff10, self.nr10.clone().into());
        bus.map(0xff11..=0xff11, self.nr11.clone().into());
        bus.map(0xff12..=0xff12, self.nr12.clone().into());
        bus.map(0xff13..=0xff13, self.nr13.clone().into());
        bus.map(0xff14..=0xff14, self.nr14.clone().into());
        // Sound Channel 2 — Pulse
        bus.map(0xff16..=0xff16, self.nr21.clone().into());
        bus.map(0xff17..=0xff17, self.nr22.clone().into());
        bus.map(0xff18..=0xff18, self.nr23.clone().into());
        bus.map(0xff19..=0xff19, self.nr24.clone().into());
        // Sound Channel 3 — Wave output
        bus.map(0xff1a..=0xff1a, self.nr30.clone().into());
        bus.map(0xff1b..=0xff1b, self.nr31.clone().into());
        bus.map(0xff1c..=0xff1c, self.nr32.clone().into());
        bus.map(0xff1d..=0xff1d, self.nr33.clone().into());
        bus.map(0xff1e..=0xff1e, self.nr34.clone().into());
        // Sound Channel 4 — Noise
        bus.map(0xff20..=0xff20, self.nr41.clone().into());
        bus.map(0xff21..=0xff21, self.nr42.clone().into());
        bus.map(0xff22..=0xff22, self.nr43.clone().into());
        bus.map(0xff23..=0xff23, self.nr44.clone().into());
    }
}
