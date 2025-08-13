//! Emulator clocking.

use std::thread;
use std::time::{Duration, Instant};

use super::perf::Profiler;

/// Clock synchronizer.
#[derive(Debug)]
pub struct Clocking {
    /// Target frequency.
    pub frq: Option<u32>,
    /// Clock instant.
    pub clk: Instant,
    /// Cycle counter.
    pub idx: u32,
}

impl Default for Clocking {
    fn default() -> Self {
        Self {
            frq: None,
            clk: Instant::now(),
            idx: u32::default(),
        }
    }
}

impl Clocking {
    /// Constructs a new `Sync`
    #[expect(unused)]
    pub fn new(delay: u32) -> Self {
        Self {
            frq: Some(delay),
            clk: Instant::now(),
            idx: u32::default(),
        }
    }

    /// Constructs a profiler from the synchronizer.
    pub fn perf(self) -> Profiler {
        let Self { clk, idx, .. } = self;
        Profiler { clk, idx }
    }

    /// Synchronizes this thread.
    ///
    /// # Returns
    ///
    /// Returns an indicator on whether a sync occurred.
    pub fn sync(&self) -> bool {
        // Only synchronze if a target frequency is provided.
        let &Self {
            frq: Some(frq),
            clk,
            idx,
        } = self
        else {
            return false;
        };

        // Compute sync timestamp
        //
        // Note the order of operations here is important, namely that the
        // duration is multiplied by the cycle count *before* dividing by
        // frequency. Otherwise, rounding within the duration leads to a
        // precision loss, amounting to an overall skew in the emulated clock.
        let sync = clk + idx * Duration::from_secs(1) / frq;

        // Compare current time against schedule
        if sync.elapsed() > Duration::ZERO {
            // If running behind, no sync needed
            false
        } else {
            // If running ahead, simply yield this thread. This causes the
            // operating system to reschedule us, allowing the wall-clock to
            // catch up.
            thread::yield_now();
            // Indicate to the caller than a sync occurred.
            true
        }
    }

    /// Ticks the synchronizer.
    pub fn tick(&mut self) {
        // Check for overflow in increment
        if let Some(idx) = self.idx.checked_add(1) {
            // Increment cycle counter
            self.idx = idx;
        } else {
            // Restart synchronizer on overflow
            self.reset();
        }
    }

    /// Resets synchronization clock.
    pub fn reset(&mut self) {
        std::mem::take(self);
    }
}
