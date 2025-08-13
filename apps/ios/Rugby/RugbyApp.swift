//
//  RugbyApp.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import GameController
import RugbyKit
import SwiftUI

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
    /// Runtime data.
    @State private var app: Runtime = .init()
    /// Game library.
    @State private var lib: Library = .init()
    /// App settings.
    @State private var opt: Options = .init()

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
                    guard let valid = try? lib.check(url: url), valid else {
                        return
                    }
                    // Add to library
                    lib.add(url: url)
                    // Play new import
                    let name = url.deletingPathExtension().lastPathComponent
                    if let game = lib.games.first(where: { $0.name == name }) {
                        app.play(game)
                    }
                }
        }
        .environment(app)
        .environment(lib)
        .environment(opt)
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
                //                emu.input(.a, pressed: pressed)
            }
        }
        pad.extendedGamepad?.buttonB.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                //                emu.input(.b, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.right.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                //                emu.input(.right, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.left.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                //                emu.input(.left, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.up.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                //                emu.input(.up, pressed: pressed)
            }
        }
        pad.extendedGamepad?.dpad.down.valueChangedHandler = { _, _, pressed in
            DispatchQueue.main.async {
                //                emu.input(.down, pressed: pressed)
            }
        }
    }
}
