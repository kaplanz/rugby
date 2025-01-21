//
//  Settings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import Foundation

@Observable
class Settings {
    /// Palette selection.
    var pal = Palette.mono
    /// Emulation speed.
    var spd = Speed.actual
}

/// Emulation speed.
enum Speed: Float, CaseIterable, CustomStringConvertible, Identifiable {
    case half = 0.5
    case actual = 1.0
    case double = 2.0
    case turbo = 0.0

    // impl CustomStringConvertible
    var description: String {
        switch self {
        case .turbo:
            "Turbo"
        default:
            rawValue.formatted(.percent)
        }
    }

    // impl Identifiable
    var id: Self { self }
}
