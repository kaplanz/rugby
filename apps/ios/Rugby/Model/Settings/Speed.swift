//
//  Speed.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-12.
//

import Foundation

/// Simulated clock frequency.
enum Speed {
    /// Actual hardware speed.
    ///
    /// The real clock frequency used by the actual hardware. Equal to 4 MiHz
    /// (approx. 59.7 FPS).
    case actual
    /// Speedup ratio.
    ///
    /// Multiple of the actual hardware speed. May be a floating point.
    case ratio(Double)
    /// Clock frequency.
    ///
    /// Precise frequency (Hz) to clock the emulator. Must be an integer.
    case clock(UInt32)
    /// Frame rate.
    ///
    /// Frequency that targets the supplied frame rate (FPS). Must be an
    /// integer.
    case frame(UInt8)
    /// Maximum possible speed.
    ///
    /// Unconstrained, limited only by the host system's capabilities.
    case turbo
}

extension Speed {
    /// Converts the `Speed` to its corresponding frequency.
    var freq: UInt32? {
        switch self {
        case .actual:
            CLOCK
        case .ratio(let mult):
            UInt32(Double(CLOCK) * mult)
        case .clock(let freq):
            freq
        case .frame(let rate):
            UInt32(rate) * VIDEO
        case .turbo:
            nil
        }
    }
}

extension Speed: Codable {}

extension Speed: CustomStringConvertible {
    var description: String {
        switch self {
        case .actual:
            "Actual"
        case .ratio(let mult):
            mult.formatted(.percent.precision(.fractionLength(0)))
        case .clock(let freq):
            Measurement(value: Double(freq), unit: UnitFrequency.hertz).converted(to: .megahertz)
                .formatted(
                    .measurement(
                        width: .abbreviated,
                        numberFormatStyle: .number.precision(.fractionLength(3))))
        case .frame(let rate):
            Measurement(value: Double(rate), unit: UnitFrequency.framesPerSecond).formatted()
        case .turbo:
            "Turbo"
        }
    }
}

extension Speed: Hashable {}
