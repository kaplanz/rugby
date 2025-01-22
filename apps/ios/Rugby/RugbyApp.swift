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
        initGameController()
    }

    var body: some Scene {
        WindowGroup {
            MainView()
                .onOpenURL { url in
                    // Acquire access permission
                    if !url.startAccessingSecurityScopedResource() {
                        fatalError("failed to securely access path: “\(url)”")
                    }
                    // Ensure valid ROM
                    do {
                        // Read the file data
                        let data = try Data(contentsOf: url)
                        // Try to construct a cartridge
                        let _ = try Cartridge(rom: data)
                    } catch let error as RugbyKit.Error {
                        // Retain cartridge errors
                        lib.error.append(error)
                        return
                    } catch let error {
                        // Crash on unknown errors
                        fatalError(error.localizedDescription)
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
