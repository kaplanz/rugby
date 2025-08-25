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
    @Environment(\.scenePhase) private var scenePhase

    /// Emulator instance.
    @State var emu: Emulator = .init()

    /// Emulator paused.
    @State private var isPaused = false
    /// Show game info.
    @State private var showInfo = false
    /// Show settings.
    @State private var showConf = false

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
        .background(Background())
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
            } catch {
                err.or = error
            }
        }
        .onChange(of: active, initial: true) {
            // Sync emulator
            emu.pause(!active)
        }
    }

    var menu: some View {
        Menu("Controls", systemImage: "ellipsis") {
            // Playback
            ControlGroup {
                Button {
                    // TODO: Implement rewind
                } label: {
                    Label("Rewind", systemImage: false ? "backward.fill" : "backward")
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
                    // TODO: Implement forward
                } label: {
                    Label("Forward", systemImage: false ? "forward.fill" : "forward")
                }
                .disabled(true)
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
}

#Preview {
    NavigationStack {
        EmulatorView()
    }
    .environment(Runtime())
    .environment(Failure())
}

private struct Background: View {
    var body: some View {
        Color.shell
            .overlay {
                Image("Noise")
                    .resizable(resizingMode: .tile)
                    .opacity(0.15)
            }
            .ignoresSafeArea()
    }
}
