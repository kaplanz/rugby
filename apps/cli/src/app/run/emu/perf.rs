//! Emulator profiler.

use std::time::Instant;

/// Emulator performance measurements.
///
/// Calculates the running frame rate of the emulator thread.
#[derive(Clone, Debug)]
pub struct Profiler {
    /// Clock instant
    pub clk: Instant,
    /// Clock counter.
    pub idx: u32,
}

impl Default for Profiler {
    fn default() -> Self {
        Self {
            clk: Instant::now(),
            idx: u32::default(),
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
    pub fn tick(&mut self) {
        // Increment counter
        self.idx += 1;
    }

    /// Ticks the profiler by the specified number of cycles.
    #[allow(unused)]
    pub fn tick_by(&mut self, count: u32) {
        // Increment counter
        self.idx += count;
    }

    /// Reports the profiled frequency.
    ///
    /// # Returns
    ///
    /// Extrapolates the average frequency over the recorded time period.
    pub fn report(&mut self) -> f64 {
        // Compute rate
        let rate = f64::from(self.idx) / self.clk.elapsed().as_secs_f64();
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
        if self.clk.elapsed().as_secs() >= 1 {
            Some(self.report())
        } else {
            None
        }
    }
}
