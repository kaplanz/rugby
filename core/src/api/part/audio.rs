//! Audio API.

use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Audio interface.
pub trait Audio {
    /// Samples audio output signals.
    ///
    /// Returns a rich audio sample with channel metadata.
    fn sample(&self) -> Chiptune;
}

/// Rich audio sample.
///
/// Contains individual samples generated by each audio channel. In a real
/// system, these are [mixed](Self::mix) together before being sent to the
/// speaker.
#[derive(Clone, Debug)]
pub struct Chiptune {
    /// Master volume.
    ///
    /// Generally used as a multiplier across all channels.
    pub vol: Sample,
    /// Channel 1 output.
    pub ch1: Sample,
    /// Channel 2 output.
    pub ch2: Sample,
    /// Channel 3 output.
    pub ch3: Sample,
    /// Channel 4 output.
    pub ch4: Sample,
}

impl Chiptune {
    /// Mixes all channels together to produce a single audio sample.
    #[must_use]
    pub fn mix(self) -> Sample {
        [self.ch1, self.ch2, self.ch3, self.ch4]
            .into_iter()
            // mix channel samples
            .sum::<Sample>()
            // normalize output
            .div(4.)
            // apply master volume
            .mul(self.vol)
    }
}

/// Audio sample.
///
/// Represents a pair of stereo channel outputs. Samples are 32-bit pulse-code
/// modulated floating point values that are linearly scaled between -1 and 1.
#[derive(Clone, Debug, Default)]
pub struct Sample {
    /// Left channel.
    pub lt: f32,
    /// Right channel.
    pub rt: f32,
}

impl From<(f32, f32)> for Sample {
    fn from((lt, rt): (f32, f32)) -> Self {
        Self { lt, rt }
    }
}

impl Add<Sample> for Sample {
    type Output = Self;

    fn add(self, rhs: Sample) -> Self::Output {
        Self::Output {
            lt: self.lt + rhs.lt,
            rt: self.rt + rhs.rt,
        }
    }
}

impl Sum for Sample {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), std::ops::Add::add)
    }
}

impl Sub<Sample> for Sample {
    type Output = Self;

    fn sub(self, rhs: Sample) -> Self::Output {
        self + -rhs
    }
}

impl Neg for Sample {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            lt: -self.lt,
            rt: -self.rt,
        }
    }
}

impl Mul<Sample> for Sample {
    type Output = Sample;

    fn mul(self, rhs: Sample) -> Self::Output {
        Self::Output {
            lt: self.lt * rhs.lt,
            rt: self.rt * rhs.rt,
        }
    }
}

impl Mul<f32> for Sample {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            lt: self.lt * rhs,
            rt: self.rt * rhs,
        }
    }
}

impl Div<f32> for Sample {
    type Output = Self;

    #[expect(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: f32) -> Self::Output {
        self * rhs.recip()
    }
}

impl Mul<Sample> for f32 {
    type Output = Sample;

    fn mul(self, rhs: Sample) -> Self::Output {
        Self::Output {
            lt: self * rhs.lt,
            rt: self * rhs.rt,
        }
    }
}
