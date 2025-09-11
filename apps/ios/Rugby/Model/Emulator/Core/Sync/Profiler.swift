//
//  Profiler.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import Foundation

/// Emulator performance measurements.
///
/// Calculates the running frame rate of the emulator thread.
struct Profiler {
    /// Clock instant.
    var clk: Date = .now
    /// Clock counter.
    var idx: UInt32 = 0

    /// Performs a reset on the profiler.
    mutating func reset() {
        self = .init()
    }

    /// Ticks the profiler by the specified number of cycles.
    mutating func tick(by count: UInt32 = 1) {
        // Increment counter
        idx += count
    }

    /// Reports the profiled frequency.
    ///
    /// # Returns
    ///
    /// Extrapolates the average frequency over the recorded time period.
    mutating func report() -> Double {
        // Compute rate
        let rate = Double(idx) / Date.now.timeIntervalSince(clk)
        // Reset profiler
        self.reset()
        // Return rate
        return rate
    }

    /// Reports the profiled frequency after a second has passed.
    ///
    /// # Returns
    ///
    /// Every second, the profiler will return the period-adjusted cycle count.
    mutating func reportDelay(seconds delay: TimeInterval = 1) -> Double? {
        // Report after delay
        if Date.now.timeIntervalSince(clk) >= delay {
            return self.report()
        } else {
            return nil
        }
    }
}
