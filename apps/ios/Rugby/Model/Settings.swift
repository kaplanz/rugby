//
//  Settings.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import Foundation

@Observable
class Settings {
    /// Configuration file.
    let path: URL
    /// Configuration data.
    let data: Config

    init(path: URL) {
        self.path = path
        self.data = Config()
    }
}

@Observable
class Config {
    /// Palette selection.
    var pal = Palette.mono
    /// Emulation speed.
    var spd = Speed.actual
}

/// Emulation speed.
enum Speed: Float, CaseIterable {
    case half = 0.5
    case actual = 1.0
    case double = 2.0
    case turbo = 0.0
}

extension Speed: CustomStringConvertible {
    var description: String {
        switch self {
        case .turbo:
            "Turbo"
        default:
            rawValue.formatted(.percent)
        }
    }
}

extension Speed: Identifiable {
    var id: Self { self }
}
