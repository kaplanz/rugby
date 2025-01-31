//! Audio registers.

use bitfield_struct::bitfield;
use rugby_arch::mem::Memory;
use rugby_arch::reg::Register;
use rugby_arch::{Byte, Word};

/// Audio master control.
///
/// See more details [here][nr52].
///
/// [nr52]: https://gbdev.io/pandocs/Audio_Registers.html#ff26--nr52-audio-master-control
#[bitfield(u8, order = msb)]
pub struct Nr52 {
    /// `NR52[7]`: Audio enable.
    ///
    /// This controls whether the APU is powered on at all (akin to [LCDC bit
    /// 7][lcdc.7]). Turning the APU off drains less power (around 16%), but
    /// clears all APU registers and makes them read-only until turned back on,
    /// except NR521. Turning the APU off, however, does not affect [Wave
    /// RAM](Wave), which can always be read/written, nor the [DIV-APU] counter.
    ///
    /// [lcdc.7]: super::ppu::Lcdc::Enable
    #[bits(1)]
    pub enable: bool,
    /// `NR52[6:4]`: Padding.
    #[bits(3)]
    __: u8,
    /// `NR52[3]`: Channel 4 enabled. (Read-only)
    ///
    /// Allows checking whether this channel is active. Writing to this bit does
    /// **not** enable or disable the channel.
    #[bits(1)]
    pub ch4_on: bool,
    /// `NR52[2]`: Channel 3 enabled. (Read-only)
    ///
    /// Allows checking whether this channel is active. Writing to this bit does
    /// **not** enable or disable the channel.
    #[bits(1)]
    pub ch3_on: bool,
    /// `NR52[1]`: Channel 2 enabled. (Read-only)
    ///
    /// Allows checking whether this channel is active. Writing to this bit does
    /// **not** enable or disable the channel.
    #[bits(1)]
    pub ch2_on: bool,
    /// `NR52[0]`: Channel 1 enabled. (Read-only)
    ///
    /// Allows checking whether this channel is active. Writing to this bit does
    /// **not** enable or disable the channel.
    #[bits(1)]
    pub ch1_on: bool,
}

impl Nr52 {
    /// Readable bit mask.
    const READABLE: Byte = 0b1_000_1111;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b1_000_0000;
}

impl Memory for Nr52 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr52 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Sound panning.
///
/// Each channel can be panned hard left, center, hard right, or ignored
/// entirely.
///
/// Setting a bit to 1 enables the channel to go into the selected output.
///
/// # Note
///
/// Selecting or de-selecting a channel whose [DAC] is enabled will [cause an audio
/// pop][pop].
///
/// See more details [here][nr51].
///
/// [dac]:  https://gbdev.io/pandocs/Audio_details.html#dacs
/// [pop]:  https://gbdev.io/pandocs/Audio_details.html#mixer
/// [nr51]: https://gbdev.io/pandocs/Audio_Registers.html#ff25--nr51-sound-panning
#[bitfield(u8, order = msb)]
pub struct Nr51 {
    /// `NR51[7]`: Channel 4 left.
    #[bits(1)]
    pub ch4_l: bool,
    /// `NR51[6]`: Channel 3 left.
    #[bits(1)]
    pub ch3_l: bool,
    /// `NR51[5]`: Channel 2 left.
    #[bits(1)]
    pub ch2_l: bool,
    /// `NR51[4]`: Channel 1 left.
    #[bits(1)]
    pub ch1_l: bool,
    /// `NR51[3]`: Channel 4 right.
    #[bits(1)]
    pub ch4_r: bool,
    /// `NR51[2]`: Channel 3 right.
    #[bits(1)]
    pub ch3_r: bool,
    /// `NR51[1]`: Channel 2 right.
    #[bits(1)]
    pub ch2_r: bool,
    /// `NR51[0]`: Channel 1 right.
    #[bits(1)]
    pub ch1_r: bool,
}

impl Memory for Nr51 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr51 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = value;
    }
}

/// Master volume & VIN panning.
///
/// See more details [here][nr50].
///
/// [nr50]: https://gbdev.io/pandocs/Audio_Registers.html#ff24--nr50-master-volume--vin-panning
#[bitfield(u8, order = msb)]
pub struct Nr50 {
    /// `NR50[7]`: VIN left.
    ///
    /// Work exactly like the bits in [NR51](Nr51). Should be set at 0 if
    /// external sound hardware is not being used.
    #[bits(1)]
    pub vin_l: bool,
    /// `NR50[6:4]`: Left volume.
    ///
    /// Specifies the master volume for the left output.
    ///
    /// # Note
    ///
    /// A value of 0 is treated as a volume of 1 (very quiet), and a value of 7
    /// is treated as a volume of 8 (no volume reduction). Importantly, the
    /// amplifier **never mutes** a non-silent input.
    #[bits(3)]
    pub vol_l: u8,
    /// `NR50[5]`: VIN right.
    ///
    /// Work exactly like the bits in [NR51](Nr51). Should be set at 0 if
    /// external sound hardware is not being used.
    #[bits(1)]
    pub vin_r: bool,
    /// `NR50[2:0]`: Right volume.
    ///
    /// Specifies the master volume for the right output.
    ///
    /// # Note
    ///
    /// A value of 0 is treated as a volume of 1 (very quiet), and a value of 7
    /// is treated as a volume of 8 (no volume reduction). Importantly, the
    /// amplifier **never mutes** a non-silent input.
    #[bits(3)]
    pub vol_r: u8,
}

impl Memory for Nr50 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr50 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = value;
    }
}
