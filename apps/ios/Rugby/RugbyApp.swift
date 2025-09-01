//
//  RugbyApp.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import AVFoundation
import GameController
import RugbyKit
import SwiftUI

@main
struct RugbyApp: App {
    /// Runtime data.
    @State private var app: Runtime = .init()
    /// Emulator error.
    @State private var err: Failure = .init()
    /// Game library.
    @State private var lib: Library = .init()
    /// App settings.
    @State private var opt: Options = .init()

    init() {
        // Initialize emulator
        RugbyKit.initialize()
        // Connect controllers
        initGameController()
        // Enable audio playback
        enableAudio()
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
                    do { try lib.add(url: url) } catch { err.or = error }
                    // Play new import
                    let name = url.deletingPathExtension().lastPathComponent
                    if let game = lib.games.first(where: { $0.name == name }) {
                        do { try app.play(game) } catch { err.or = error }
                    }
                }
                .alert(
                    "Error",
                    isPresented: Binding(
                        get: { err.or != nil },
                        set: { if !$0 { err.or = nil } },
                    ),
                    presenting: err.or,
                ) { _ in
                    Button("OK", role: .cancel) {}
                } message: { error in
                    Text(error.localizedDescription)
                }
        }
        .onChange(of: opt.data.pal.tint, initial: true) { _, tint in
            UIApplication.shared.connectedScenes.compactMap { $0 as? UIWindowScene }.flatMap {
                $0.windows
            }.forEach { window in
                window.tintColor = UIColor(tint)
            }
        }
        .environment(app)
        .environment(err)
        .environment(lib)
        .environment(opt)
    }
}

extension RugbyApp {
    func enableAudio() {
        let session = AVAudioSession.sharedInstance()
        do {
            try session.setCategory(.playback)
            try session.setActive(true)
        } catch {
            err.or = error
        }
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
