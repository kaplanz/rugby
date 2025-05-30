//
//  RugbyApp.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import GameController
import OSLog
import RugbyKit
import SwiftUI

/// Global logger.
let log = Logger()

/// Define core logger.
@_cdecl("log")
func rugby_log(level: UInt64, target: UnsafePointer<CChar>, message: UnsafePointer<CChar>) {
    // Decode log level
    let level: OSLogType = [
        // error
        .error,
        // warn
        .error,
        // info
        .info,
        // debug
        .debug,
        // trace
        .debug,
    ][Int(level)]
    // Write with global logger
    log.log(level: level, "[\(String(cString: target))]: \(String(cString: message))")
}

struct Build {
    /// Application name.
    static let NAME = Bundle.main.infoDictionary?["CFBundleName"] as! String
    /// Version number.
    static let VERSION = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as! String
    /// Compilation date.
    static let DATE = Date.now
}

@main
struct RugbyApp: App {
    /// Global emulator instance.
    @State private var emu = GameBoy()
    /// Global game library.
    @State private var lib = Library()

    init() {
        // Initialize core
        RugbyKit.initialize()
        // Initialize game
        initGameController()
    }

    var body: some Scene {
        WindowGroup {
            MainView()
                .onOpenURL { url in
                    // Ensure valid ROM
                    guard let valid = try? lib.precheck(url: url), valid else {
                        return
                    }
                    // Add to library
                    lib.insert(src: url)
                    // Play in emulator
                    let name = url.deletingPathExtension().lastPathComponent
                    if let game = lib.games.first(where: { $0.name == name }) {
                        emu.play(game)
                    }
                }
        }
        .environment(emu)
        .environment(lib)
    }
}

extension RugbyApp {
    func initGameController() {
        // Start looking for wireless controllers
        GCController.startWirelessControllerDiscovery {
            log.debug("discovering wireless controllers")
        }

        // Observe controller connections
        NotificationCenter.default.addObserver(
            forName: .GCControllerDidConnect,
            object: nil,
            queue: nil
        ) { note in
            guard let pad = note.object as? GCController else {
                return
            }
            log.info("controller connected: \(pad)")

            // Handle controller button input
            initGameControllerHandlers(pad: pad)
        }

        // Observe controller connections
        NotificationCenter.default.addObserver(
            forName: .GCControllerDidDisconnect,
            object: nil,
            queue: nil
        ) { note in
            guard let pad = note.object as? GCController else {
                return
            }
            log.info("controller disconnected: \(pad)")
        }
    }

    nonisolated func initGameControllerHandlers(pad: GCController) {
        pad.extendedGamepad?.buttonA.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                emu.input(.a, pressed: pressed)
            }
        }
        pad.extendedGamepad?.buttonB.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                emu.input(.b, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.right.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                emu.input(.right, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.left.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                emu.input(.left, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.up.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                emu.input(.up, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.down.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                emu.input(.down, pressed: pressed)
            }
        }
    }
}
