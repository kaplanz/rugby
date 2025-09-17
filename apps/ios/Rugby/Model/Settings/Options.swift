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
    fileprivate static let root = "dev.zakhary.rugby"

    /// Audio sample rate.
    var aud: Measurement<UnitFrequency> {
        get {
            access(keyPath: \.aud)
            let value = UserDefaults.standard.value(forKey: "\(Self.root).aud") as? Double
            return .init(value: value ?? 48000, unit: .hertz)
        }
        set {
            withMutation(keyPath: \.aud) {
                UserDefaults.standard
                    .setValue(
                        newValue.converted(to: .hertz).value,
                        forKey: "\(Self.root).aud",
                    )
            }
        }

    }

    /// Video shader.
    var tex: Shader? {
        get {
            access(keyPath: \.tex)
            return UserDefaults.standard.string(forKey: "\(Self.root).tex").flatMap {
                .init(rawValue: $0)
            }
        }
        set {
            withMutation(keyPath: \.tex) {
                if let newValue {
                    UserDefaults.standard.setValue(
                        newValue.rawValue, forKey: "\(Self.root).tex")
                } else {
                    UserDefaults.standard.removeObject(forKey: "\(Self.root).tex")
                }
            }
        }
    }

    /// Palette selection.
    var pal: Palette {
        get {
            access(keyPath: \.pal)
            return UserDefaults.standard.data(forKey: "\(Self.root).pal").flatMap {
                try? JSONDecoder().decode(Palette.self, from: $0)
            } ?? .default
        }
        set {
            withMutation(keyPath: \.pal) {
                if let data = try? JSONEncoder().encode(newValue) {
                    UserDefaults.standard.set(data, forKey: "\(Self.root).pal")
                }
            }
        }
    }

    /// Emulation speed.
    var spd: Speedup = .init()

    @Observable
    class Speedup {
        fileprivate static let root = "\(Config.root).spd"

        /// Forward speed.
        var fwd: Speed {
            get {
                access(keyPath: \.fwd)
                return UserDefaults.standard.data(forKey: "\(Self.root).fwd").flatMap {
                    try? JSONDecoder().decode(Speed.self, from: $0)
                } ?? .ratio(2.0)
            }
            set {
                withMutation(keyPath: \.fwd) {
                    if let data = try? JSONEncoder().encode(newValue) {
                        UserDefaults.standard.set(data, forKey: "\(Self.root).fwd")
                    }
                }
            }
        }
        /// Reverse speed.
        var rev: Speed {
            get {
                access(keyPath: \.rev)
                return UserDefaults.standard.data(forKey: "\(Self.root).rev").flatMap {
                    try? JSONDecoder().decode(Speed.self, from: $0)
                } ?? .ratio(0.5)
            }
            set {
                withMutation(keyPath: \.rev) {
                    if let data = try? JSONEncoder().encode(newValue) {
                        UserDefaults.standard.set(data, forKey: "\(Self.root).rev")
                    }
                }
            }
        }
    }
}
