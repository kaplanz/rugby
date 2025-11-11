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

    /// Boot ROM image.
    var img: URL? {
        get {
            access(keyPath: \.img)
            return UserDefaults.standard.url(forKey: "\(Self.root).img").flatMap { url in
                FileManager.default.fileExists(atPath: url.path(percentEncoded: false)) ? url : nil
            }
        }
        set {
            withMutation(keyPath: \.img) {
                UserDefaults.standard.set(newValue, forKey: "\(Self.root).img")
            }
        }
    }

    /// Audio speaker.
    var aud: Speaker = .init()

    @Observable
    class Speaker {
        fileprivate static let root = "\(Config.root).aud"

        /// Audio enabled.
        var enable: Bool {
            get {
                access(keyPath: \.enable)
                let value = UserDefaults.standard.value(forKey: "\(Self.root).enable") as? Bool
                return value ?? true
            }
            set {
                withMutation(keyPath: \.enable) {
                    UserDefaults.standard.set(
                        newValue,
                        forKey: "\(Self.root).enable"
                    )
                }
            }
        }

        /// Audio volume.
        var volume: Double {
            get {
                access(keyPath: \.volume)
                let value = UserDefaults.standard.value(forKey: "\(Self.root).volume") as? Double
                return value ?? 0.8
            }
            set {
                withMutation(keyPath: \.enable) {
                    UserDefaults.standard.set(
                        newValue,
                        forKey: "\(Self.root).volume"
                    )
                }
            }
        }

        /// Audio sample rate.
        var sample: Measurement<UnitFrequency> {
            get {
                access(keyPath: \.sample)
                let value = UserDefaults.standard.value(forKey: "\(Self.root).sample") as? Double
                return .init(value: value ?? 48000, unit: .hertz)
            }
            set {
                withMutation(keyPath: \.sample) {
                    UserDefaults.standard
                        .set(
                            newValue.converted(to: .hertz).value,
                            forKey: "\(Self.root).sample",
                        )
                }
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
                    UserDefaults.standard.set(
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

    /// Heads-up display.
    var hud: Bool {
        get {
            access(keyPath: \.hud)
            return UserDefaults.standard.bool(forKey: "\(Self.root).hud")
        }
        set {
            withMutation(keyPath: \.hud) {
                UserDefaults.standard.set(newValue, forKey: "\(Self.root).hud")
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
