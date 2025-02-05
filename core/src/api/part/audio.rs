//! Audio API.

use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Audio interface.
pub trait Audio {
    /// Samples audio output signals.
    ///
    /// Receives a pair of audio samples, corresponding to the left and right
    /// stereo outputs respectively.
    fn sample(&self) -> Sample;
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

    #[allow(clippy::suspicious_arithmetic_impl)]
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
