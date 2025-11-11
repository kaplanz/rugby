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
    /// Gamepad handle.
    @State private var pad: Gamepad = .init()

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
        .environment(pad)
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
            queue: nil,
        ) { notification in
            guard let controller = notification.object as? GCController else { return }
            log.notice("controller connected: \(controller)")
        }

        // Observe gamepad disconnects
        NotificationCenter.default.addObserver(
            forName: .GCControllerDidDisconnect,
            object: nil,
            queue: nil,
        ) { notification in
            guard let controller = notification.object as? GCController else { return }
            log.notice("controller disconnected: \(controller)")
        }
    }
}
