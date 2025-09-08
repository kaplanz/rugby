//
//  EmulatorView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct EmulatorView: View {
    @Environment(Runtime.self) private var app
    @Environment(Failure.self) private var err
    @Environment(Options.self) private var opt
    @Environment(\.scenePhase) private var scenePhase

    /// Emulator instance.
    @State var emu: Emulator = .init()

    /// Emulator paused.
    @State private var isPaused = false
    /// Show game info.
    @State private var showInfo = false
    /// Show settings.
    @State private var showConf = false
    /// Play controls.
    @State private var playback: Playback?
    enum Playback {
        case forward, reverse
    }

    /// Emulator is active.
    private var active: Bool {
        !isPaused && !showInfo && !showConf && scenePhase == .active
    }

    /// Video output frame.
    private var frame: UIImage? {
        emu.frame.map { UIImage(cgImage: $0) }
    }

    var body: some View {
        GeometryReader { geo in
            if geo.size.height > geo.size.width {
                VStack {
                    ScreenView(frame: frame)
                        .id(frame)
                    Spacer()
                    JoypadView(emu.input(_:state:))
                    Spacer()
                }
            } else {
                HStack {
                    JoypadView(emu.input(_:state:), part: .left)
                    Spacer()
                    ScreenView()
                    Spacer()
                    JoypadView(emu.input(_:state:), part: .right)
                }
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(back)
        .toolbar {
            ToolbarSpacer(placement: .bottomBar)
            ToolbarItemGroup(placement: .bottomBar) {
                menu
            }
        }
        .sheet(isPresented: $showInfo) {
            NavigationStack {
                GameInfoView(game: app.game!)
                    .toolbar {
                        Button("Done", systemImage: "checkmark", role: .confirm) {
                            showInfo.toggle()
                        }
                        .bold()
                    }
            }
        }
        .sheet(isPresented: $showConf) {
            // Refresh configuration
            self.player(playback)
        } content: {
            NavigationStack {
                SettingsView()
                    .toolbar {
                        Button("Done", systemImage: "checkmark", role: .confirm) {
                            showConf.toggle()
                        }
                        .bold()
                    }
            }
        }
        .onChange(of: app.game, initial: true) {
            do {
                if let game = app.game {
                    // Insert cartridge
                    try emu.play(game)
                } else {
                    // Remove cartridge
                    try emu.stop()
                }
            } catch { err.log(error) }
        }
        .onChange(of: active, initial: true) {
            // Sync emulator
            emu.pause(!active)
            // Prevent idle
            UIApplication.shared.isIdleTimerDisabled = active
        }
        .onChange(of: playback) { _, newValue in
            self.player(playback)
        }
        .onDisappear {
            // Reallow idle
            UIApplication.shared.isIdleTimerDisabled = false
        }
    }

    /// Update emulation playback.
    func player(_ event: Playback?) {
        switch event {
        case .forward:
            emu.speed(opt.data.spd.fwd)
        case .reverse:
            // TODO: Implement reverse
            break
        default:
            emu.speed(.actual)
        }
    }

    var menu: some View {
        Menu("Controls", systemImage: "ellipsis") {
            // Playback
            ControlGroup {
                Button {
                    if playback.take() == nil {
                        playback = .reverse
                    }
                } label: {
                    Label(
                        "Reverse",
                        systemImage:
                            playback == .reverse ? "backward.fill" : "backward")
                }
                .disabled(true)
                Button {
                    isPaused.toggle()
                } label: {
                    isPaused
                        ? Label("Play", systemImage: "play")
                        : Label("Pause", systemImage: "pause.fill")
                }
                Button {
                    if playback.take() == nil {
                        playback = .forward
                    }
                } label: {
                    Label(
                        "Forward",
                        systemImage:
                            playback == .forward ? "forward.fill" : "forward"
                    )
                }
            }
            // Emulator
            Section {
                Button("Reset", systemImage: "arrow.counterclockwise") {
                    emu.reset(.soft)
                }
            }
            // Information
            Section {
                Button("Get Info", systemImage: "info.circle") {
                    showInfo.toggle()
                }
                Button("Settings", systemImage: "gearshape") {
                    showConf.toggle()
                }
            }
            // Application
            Section {
                Button("Exit", systemImage: "xmark", role: .destructive) {
                    app.stop()
                }
            }
        }
    }

    var back: some View {
        Color.shell
            .overlay {
                Image("Noise")
                    .resizable(resizingMode: .tile)
                    .opacity(0.15)
            }
            .ignoresSafeArea()
    }
}

#Preview {
    NavigationStack {
        EmulatorView()
    }
    .environment(Runtime())
    .environment(Failure())
}
