//! Emulator profiler.

use std::time::Instant;

/// Emulator performance measurements.
///
/// Calculates the running frame rate of an emulator task,
#[derive(Clone, Debug)]
pub struct Profiler {
    /// Elapsed cycles count.
    pub cycle: u32,
    /// Statistics timestamp.
    pub timer: Instant,
}

impl Default for Profiler {
    fn default() -> Self {
        Self {
            cycle: u32::default(),
            timer: Instant::now(),
        }
    }
}

impl Profiler {
    /// Constructs a new `Counter`.
    #[allow(unused)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Performs a reset on the profiler.
    pub fn reset(&mut self) {
        std::mem::take(self);
    }

    /// Ticks the profiler.
    ///
    /// # Returns
    ///
    /// Every second, the profiler will return the wall-clock cycle count.
    #[expect(unused)]
    pub fn tick(&mut self) {
        // Increment counter
        self.cycle += 1;
    }

    /// Ticks the profiler by the specified amount.
    pub fn tick_by(&mut self, count: u32) {
        // Increment counter
        self.cycle += count;
    }

    /// Reports the profiled frequency.
    ///
    /// # Returns
    ///
    /// Extrapolates the average frequency over the recorded time period.
    pub fn report(&mut self) -> f64 {
        // Compute rate
        let rate = self
            .timer
            .elapsed()
            .checked_div(self.cycle)
            .map_or(f64::NAN, |iter| iter.as_secs_f64().recip());
        // Reset profiler
        self.reset();
        // Return rate
        rate
    }

    /// Reports the profiled frequency after a second has passed.
    ///
    /// # Returns
    ///
    /// Every second, the profiler will return the period-adjusted cycle count.
    pub fn report_delay(&mut self) -> Option<f64> {
        // Report after delay
        if self.timer.elapsed().as_secs() >= 1 {
            Some(self.report())
        } else {
            None
        }
    }
}
