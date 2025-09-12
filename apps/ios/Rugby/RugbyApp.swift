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
        // Initialize gamepads
        initGamepad()
        // Initialize playback
        enableAudio()
    }

    var body: some Scene {
        WindowGroup {
            MainView()
                .onOpenURL { file in
                    do {
                        // Ensure valid ROM
                        try lib.check(url: file)
                        // Add to library
                        try lib.add(url: file)
                    } catch { err.log(error) }
                    // Play new import
                    let name = file.deletingPathExtension().lastPathComponent
                    if let game = lib.games.first(where: { $0.name == name }) {
                        do { try app.play(game) } catch { err.log(error) }
                    }
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
        } catch { err.log(error) }
    }
}

extension RugbyApp {
    func initGamepad() {
        // Start looking wireless gamepads
        GCController.startWirelessControllerDiscovery {
            log.debug("discovering wireless controllers")
        }

        // Observe gamepad connections
        NotificationCenter.default.addObserver(
            forName: .GCControllerDidConnect,
            object: nil,
            queue: nil
        ) { note in
            guard let pad = note.object as? GCController else {
                return
            }
            log.info("controller connected: \(pad)")

            // Handle button input
            initGamepadHandlers(pad: pad)
        }

        // Observe gamepad connections
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

    nonisolated func initGamepadHandlers(pad: GCController) {
        pad.extendedGamepad?.buttonA.valueChangedHandler = { _, _, state in
            DispatchQueue.main.async {
                app.emu?.input(.a, state: state)
            }
        }
        pad.extendedGamepad?.buttonB.valueChangedHandler = { _, _, state in
            DispatchQueue.main.async {
                app.emu?.input(.b, state: state)
            }
        }
        pad.extendedGamepad?.dpad.right.valueChangedHandler = { _, _, state in
            DispatchQueue.main.async {
                app.emu?.input(.right, state: state)
            }
        }
        pad.extendedGamepad?.dpad.left.valueChangedHandler = { _, _, state in
            DispatchQueue.main.async {
                app.emu?.input(.left, state: state)
            }
        }
        pad.extendedGamepad?.dpad.up.valueChangedHandler = { _, _, state in
            DispatchQueue.main.async {
                app.emu?.input(.up, state: state)
            }
        }
        pad.extendedGamepad?.dpad.down.valueChangedHandler = { _, _, state in
            DispatchQueue.main.async {
                app.emu?.input(.down, state: state)
            }
        }
    }
}
