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

/// Channel 1 sweep.
///
/// See more details [here][nr10].
///
/// [nr10]: https://gbdev.io/pandocs/Audio_Registers.html#ff10--nr10-channel-1-sweep
#[bitfield(u8, order = msb)]
pub struct Nr10 {
    /// `NR10[7]`: Padding.
    #[bits(1)]
    __: u8,
    /// `NR10[6:4]`: Pace of sweep iterations.
    #[bits(3)]
    pub pace: u8,
    /// `NR10[3]`: Direction of sweep period.
    ///
    /// - `0`: increase period
    /// - `1`: decrease period
    #[bits(1)]
    pub sign: bool,
    /// `NR10[2:0]`: Individual step modifier.
    #[bits(3)]
    pub step: u8,
}

impl Nr10 {
    /// Unusable bit mask.
    const UNUSABLE: Byte = 0b1_000_0_000;
}

impl Memory for Nr10 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr10 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | Self::UNUSABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= !Self::UNUSABLE;
        self.0 = value;
    }
}

/// Channel 1 length timer & duty cycle.
///
/// See more details [here][nr11].
///
/// [nr11]: https://gbdev.io/pandocs/Audio_Registers.html#ff11--nr11-channel-1-length-timer--duty-cycle
#[bitfield(u8, order = msb)]
pub struct Nr11 {
    /// `NR11[7:6]`: Waveform duty cycle.
    #[bits(2)]
    pub duty: usize,
    /// `NR11[5:0]`: Initial length timer. (Write-only)
    #[bits(6)]
    pub step: u8,
}

impl Nr11 {
    /// Readable bit mask.
    const READABLE: Byte = 0b11_000000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b11_111111;
}

impl Memory for Nr11 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr11 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Channel 1 volume & envelope
///
/// See more details [here][nr12].
///
/// [nr12]: https://gbdev.io/pandocs/Audio_Registers.html#ff12--nr12-channel-1-volume--envelope
#[bitfield(u8, order = msb)]
pub struct Nr12 {
    /// `NR12[7:4]`: Initial volume.
    #[bits(4)]
    pub ivol: u8,
    /// `NR12[3]`: Envelope direction.
    ///
    /// - `0`: decrease volume
    /// - `1`: increase volume
    #[bits(1)]
    pub sign: bool,
    /// `NR12[2:0]`: Envelope pace.
    #[bits(3)]
    pub pace: u8,
}

impl Memory for Nr12 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr12 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = value;
    }
}

/// Channel 1 period low.
///
/// See more details [here][nr13].
///
/// [nr13]: https://gbdev.io/pandocs/Audio_Registers.html#ff13--nr13-channel-1-period-low-write-only
#[bitfield(u8, order = msb)]
pub struct Nr13 {
    /// `NR13[7:0]`: Period low. (Write-only)
    #[bits(8)]
    pub clk_lo: u8,
}

impl Nr13 {
    /// Readable bit mask.
    const READABLE: Byte = 0b00000000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b11111111;
}

impl Memory for Nr13 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr13 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Channel 1 period high & control.
///
/// See more details [here][nr14].
///
/// [nr14]: https://gbdev.io/pandocs/Audio_Registers.html#ff14--nr14-channel-1-period-high--control
#[bitfield(u8, order = msb)]
pub struct Nr14 {
    /// `NR14[7]`: Channel trigger. (Write-only)
    #[bits(1)]
    pub trigger: bool,
    /// `NR14[6]`: Length enable.
    #[bits(1)]
    pub length: bool,
    /// `NR14[5:3]`: Padding.
    #[bits(3)]
    __: u8,
    /// `NR14[2:0]`: Period high. (Write-only)
    #[bits(3)]
    pub clk_hi: u8,
}

impl Nr14 {
    /// Readable bit mask.
    const READABLE: Byte = 0b0_1_000_000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b1_1_000_111;
}

impl Memory for Nr14 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr14 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Channel 2 length timer & duty cycle.
///
/// See more details [here][nr11].
///
/// [nr11]: https://gbdev.io/pandocs/Audio_Registers.html#ff11--nr11-channel-1-length-timer--duty-cycle
pub type Nr21 = Nr11;

/// Channel 2 volume & envelope
///
/// See more details [here][nr12].
///
/// [nr12]: https://gbdev.io/pandocs/Audio_Registers.html#ff12--nr12-channel-1-volume--envelope
pub type Nr22 = Nr12;

/// Channel 2 period low.
///
/// See more details [here][nr13].
///
/// [nr13]: https://gbdev.io/pandocs/Audio_Registers.html#ff13--nr13-channel-1-period-low-write-only
pub type Nr23 = Nr13;

/// Channel 2 period high & control.
///
/// See more details [here][nr14].
///
/// [nr14]: https://gbdev.io/pandocs/Audio_Registers.html#ff14--nr14-channel-1-period-high--control
pub type Nr24 = Nr14;

/// Channel 3 DAC enable.
///
/// See more details [here][nr30].
///
/// [nr30]: https://gbdev.io/pandocs/Audio_Registers.html#ff1a--nr30-channel-3-dac-enable
#[bitfield(u8, order = msb)]
pub struct Nr30 {
    /// `NR30[7]`: DAC enable.
    #[bits(1)]
    pub dac: bool,
    /// `NR30[6:0]`: Padding.
    #[bits(7)]
    __: u8,
}

impl Nr30 {
    /// Unusable bit mask.
    const UNUSABLE: Byte = 0b0_1111111;
}

impl Memory for Nr30 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr30 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | Self::UNUSABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= !Self::UNUSABLE;
        self.0 = value;
    }
}

/// Channel 3 length timer.
///
/// See more details [here][nr31].
///
/// [nr31]: https://gbdev.io/pandocs/Audio_Registers.html#ff1b--nr31-channel-3-length-timer-write-only
#[bitfield(u8, order = msb)]
pub struct Nr31 {
    /// `NR11[7:0]`: Initial length timer. (Write-only)
    #[bits(8)]
    pub step: u8,
}

impl Nr31 {
    /// Readable bit mask.
    const READABLE: Byte = 0b00000000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b11111111;
}

impl Memory for Nr31 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr31 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Channel 3 output level.
///
/// See more details [here][nr32].
///
/// [nr32]: https://gbdev.io/pandocs/Audio_Registers.html#ff1c--nr32-channel-3-output-level
#[bitfield(u8, order = msb)]
pub struct Nr32 {
    /// `NR32[7]`: Padding.
    #[bits(1)]
    __: u8,
    /// `NR32[6:5]`: Output level.
    ///
    /// Controls the channel's volume as follows:
    ///
    /// | Bits | Output Level                           |
    /// |------|----------------------------------------|
    /// | `00` | Mute (No sound)                        |
    /// | `01` | 100% volume (read samples as-is)       |
    /// | `10` | 50% volume (shift samples right once)  |
    /// | `11` | 25% volume (shift samples right twice) |
    #[bits(2)]
    pub vol: u8,
    /// `NR32[4:0]`: Padding.
    #[bits(5)]
    __: u8,
}

impl Nr32 {
    /// Unusable bit mask.
    const UNUSABLE: Byte = 0b1_00_11111;
}

impl Memory for Nr32 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr32 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | Self::UNUSABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= !Self::UNUSABLE;
        self.0 = value;
    }
}

/// Channel 3 period low.
///
/// See more details [here][nr33].
///
/// [nr33]: https://gbdev.io/pandocs/Audio_Registers.html#ff1d--nr33-channel-3-period-low-write-only
#[bitfield(u8, order = msb)]
pub struct Nr33 {
    /// `NR33[7:0]`: Period low. (Write-only)
    #[bits(8)]
    pub clk_lo: u8,
}

impl Nr33 {
    /// Readable bit mask.
    const READABLE: Byte = 0b00000000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b11111111;
}

impl Memory for Nr33 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr33 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Channel 3 period high & control.
///
/// See more details [here][nr34].
///
/// [nr34]: https://gbdev.io/pandocs/Audio_Registers.html#ff1e--nr34-channel-3-period-high--control
#[bitfield(u8, order = msb)]
pub struct Nr34 {
    /// `NR34[7]`: Channel trigger. (Write-only)
    #[bits(1)]
    pub trigger: bool,
    /// `NR34[6]`: Length enable.
    #[bits(1)]
    pub length: bool,
    /// `NR34[5:3]`: Padding.
    #[bits(3)]
    __: u8,
    /// `NR34[2:0]`: Period high. (Write-only)
    #[bits(3)]
    pub clk_hi: u8,
}

impl Nr34 {
    /// Readable bit mask.
    const READABLE: Byte = 0b0_1_000_000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b1_1_000_111;
}

impl Memory for Nr34 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr34 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Channel 4 length timer.
///
/// See more details [here][nr41].
///
/// [nr41]: https://gbdev.io/pandocs/Audio_Registers.html#ff20--nr41-channel-4-length-timer-write-only
#[bitfield(u8, order = msb)]
pub struct Nr41 {
    /// `NR11[7:6]`: Padding.
    #[bits(2)]
    __: u8,
    /// `NR11[5:0]`: Initial length timer. (Write-only)
    #[bits(6)]
    pub step: u8,
}

impl Nr41 {
    /// Readable bit mask.
    const READABLE: Byte = 0b00_000000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b00_111111;
}

impl Memory for Nr41 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr41 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}

/// Channel 4 volume & envelope
///
/// See more details [here][nr42].
///
/// [nr42]: https://gbdev.io/pandocs/Audio_Registers.html#ff21--nr42-channel-4-volume--envelope
pub type Nr42 = Nr12;

/// Channel 4 frequency & randomness.
///
/// See more details [here][nr43].
///
/// [nr43]: https://gbdev.io/pandocs/Audio_Registers.html#ff22--nr43-channel-4-frequency--randomness
#[bitfield(u8, order = msb)]
pub struct Nr43 {
    /// `NR43[7:4]`: Clock shift.
    #[bits(4)]
    pub shift: u8,
    /// `NR43[3]`: LFSR width.
    #[bits(1)]
    pub width: bool,
    /// `NR43[2:0]`: Clock divider.
    #[bits(3)]
    pub divide: u8,
}

impl Memory for Nr43 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr43 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0
    }

    fn store(&mut self, value: Self::Value) {
        self.0 = value;
    }
}

/// Channel 4 control.
///
/// See more details [here][nr44].
///
/// [nr44]: https://gbdev.io/pandocs/Audio_Registers.html#ff23--nr44-channel-4-control
#[bitfield(u8, order = msb)]
pub struct Nr44 {
    /// `NR44[7]`: Channel trigger. (Write-only)
    #[bits(1)]
    pub trigger: bool,
    /// `NR44[6]`: Length enable.
    #[bits(1)]
    pub length: bool,
    /// `NR44[5:0]`: Padding.
    #[bits(6)]
    __: u8,
}

impl Nr44 {
    /// Readable bit mask.
    const READABLE: Byte = 0b0_1_000000;

    /// Writable bit mask.
    const WRITABLE: Byte = 0b1_1_000000;
}

impl Memory for Nr44 {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Nr44 {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.0 | !Self::READABLE
    }

    fn store(&mut self, mut value: Self::Value) {
        value &= Self::WRITABLE;
        self.0 = value;
    }
}
