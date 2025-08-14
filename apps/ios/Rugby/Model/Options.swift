//
//  Options.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import Foundation

/// Application options.
@Observable
class Options {
    /// Configuration data.
    private(set) var data: Config = .init()

    /// Application identity.
    private(set) var uuid: UUID = .init()

    /// Reset application settings.
    func reset() {
        // Clear defaults
        if let bundle = Bundle.main.bundleIdentifier {
            UserDefaults.standard.removePersistentDomain(forName: bundle)
        }
        // Reset settings
        data = .init()
        // Reset identity
        uuid = .init()
    }
}

/// Configuration data.
@Observable
class Config {
    /// Palette selection.
    var pal: Palette {
        get {
            access(keyPath: \.pal)
            return UserDefaults.standard.string(forKey: "dev.zakhary.rugby.pal").flatMap {
                Palette(rawValue: $0)
            }
                ?? .demichrome
        }
        set {
            withMutation(keyPath: \.pal) {
                UserDefaults.standard.setValue(newValue.rawValue, forKey: "dev.zakhary.rugby.pal")
            }
        }
    }
    /// Emulation speed.
    var spd: Speed {
        get {
            access(keyPath: \.spd)
            return UserDefaults.standard.data(forKey: "dev.zakhary.rugby.spd").flatMap {
                try? JSONDecoder().decode(Speed.self, from: $0)
            } ?? .actual
        }
        set {
            withMutation(keyPath: \.spd) {
                if let data = try? JSONEncoder().encode(newValue) {
                    UserDefaults.standard.set(data, forKey: "dev.zakhary.rugby.spd")
                }
            }
        }
    }
}
