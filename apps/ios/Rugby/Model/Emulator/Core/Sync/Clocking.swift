//
//  Clocking.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import Foundation

/// Clock synchronizer.
struct Clocking {
    /// Target frequency.
    var frq: UInt32? = Speed.actual.freq {
        willSet {
            self.reset()
        }
    }
    /// Clock instant.
    var clk: Date = .now
    /// Cycle counter.
    var idx: UInt32 = 0

    /// Constructs a profiler from the synchronizer.
    func perf() -> Profiler {
        return Profiler(clk: clk, idx: idx)
    }

    /// Synchronizes this thread.
    ///
    /// # Returns
    ///
    /// Returns an indicator on whether a sync occurred.
    func sync() -> Bool {
        // Only synchronize if a target frequency is provided.
        guard let frq else { return false }

        // Take current timestamp
        let time = Date.now

        // Compute sync timestamp
        //
        // Note the order of operations here is important, namely that the
        // duration is multiplied by the cycle count *before* dividing by
        // frequency. Otherwise, rounding within the duration leads to a
        // precision loss, amounting to an overall skew in the emulated clock.
        let sync = clk + TimeInterval(idx) / Double(frq)

        // Compare current time against schedule
        if time > sync {
            // If running behind, no sync needed
            return false
        } else {
            // If running ahead, sleep this thread until scheduled wakeup. This
            // causes the operating system to reschedule us, allowing the
            // wall-clock to catch up.
            Thread
                .sleep(
                    forTimeInterval: sync.timeIntervalSince(time)
                )
            // Indicate to the caller that a sync occurred.
            return true
        }
    }

    /// Ticks the synchronizer.
    mutating func tick() {
        // Check for overflow in increment
        let sum = idx.addingReportingOverflow(1)
        if !sum.overflow {
            // Increment cycle counter
            idx = sum.partialValue
        } else {
            // Restart synchronizer on overflow
            self.reset()
        }
    }

    /// Resets synchronization clock.
    mutating func reset() {
        self = .init(frq: frq)
    }
}
