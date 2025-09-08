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
    /// Video shader.
    var tex: Shader? {
        get {
            access(keyPath: \.tex)
            return UserDefaults.standard.string(forKey: "dev.zakhary.rugby.tex").flatMap {
                .init(rawValue: $0)
            }
        }
        set {
            withMutation(keyPath: \.tex) {
                if let newValue {
                    UserDefaults.standard.setValue(
                        newValue.rawValue, forKey: "dev.zakhary.rugby.tex")
                } else {
                    UserDefaults.standard.removeObject(forKey: "dev.zakhary.rugby.tex")
                }
            }
        }
    }

    /// Palette selection.
    var pal: Palette {
        get {
            access(keyPath: \.pal)
            return UserDefaults.standard.string(forKey: "dev.zakhary.rugby.pal").flatMap {
                .init(rawValue: $0)
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
    var spd: Speedup = .init()

    @Observable
    class Speedup {
        /// Forward speed.
        var fwd: Speed {
            get {
                access(keyPath: \.fwd)
                return UserDefaults.standard.data(forKey: "dev.zakhary.rugby.spd.fwd").flatMap {
                    try? JSONDecoder().decode(Speed.self, from: $0)
                } ?? .ratio(2.0)
            }
            set {
                withMutation(keyPath: \.fwd) {
                    if let data = try? JSONEncoder().encode(newValue) {
                        UserDefaults.standard.set(data, forKey: "dev.zakhary.rugby.spd.fwd")
                    }
                }
            }
        }
        /// Reverse speed.
        var rev: Speed {
            get {
                access(keyPath: \.rev)
                return UserDefaults.standard.data(forKey: "dev.zakhary.rugby.spd.rev").flatMap {
                    try? JSONDecoder().decode(Speed.self, from: $0)
                } ?? .ratio(0.5)
            }
            set {
                withMutation(keyPath: \.rev) {
                    if let data = try? JSONEncoder().encode(newValue) {
                        UserDefaults.standard.set(data, forKey: "dev.zakhary.rugby.spd.rev")
                    }
                }
            }
        }
    }
}
