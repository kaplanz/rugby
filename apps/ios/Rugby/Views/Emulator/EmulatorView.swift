//
//  EmulatorView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct EmulatorView: View {
    @Environment(GameBoy.self) private var emu
    @Environment(\.scenePhase) var scenePhase

    /// Emulator paused.
    @State private var paused = false
    /// About this game.
    @State private var detail = false
    /// Manage this game.
    @State private var manage = false

    /// Emulation play status.
    @State private var status: Status? {
        didSet {
            if let status {
                switch status {
                case .sprint:
                    emu.clock(speed: nil)
                case .rewind:
                    print("unimplemented")
                }
            } else {
                emu.clock(speed: emu.cfg.data.spd.rawValue)
            }
        }
    }

    /// Emulation play status.
    enum Status {
        /// Fast forward.
        case sprint
        /// Rewind history.
        case rewind
    }

    /// Extra toolbar items.
    @State private var extras = Extras()

    /// Extra toolbar items.
    struct Extras {
        /// Quick controls.
        var ctrl = false
        /// Frame rate.
        var rate = false
    }

    /// Should the emulator be running?
    private var enable: Bool {
        !paused && !detail && !manage && scenePhase == .active
    }

    var body: some View {
        GeometryReader { geo in
            if geo.size.height > geo.size.width {
                VStack {
                    Screen()
                    Spacer()
                    Joypad()
                    Spacer()
                }
            } else {
                HStack {
                    Joypad(part: .left)
                    Spacer()
                    Screen()
                    Spacer()
                    Joypad(part: .right)
                }
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Background())
        .toolbar {
            ToolbarSpacer(placement: .bottomBar)
            ToolbarItemGroup(placement: .bottomBar) {
                Controls
            }
        }
        .alert(
            "Error",
            isPresented: Binding(
                get: { emu.error != nil },
                set: { if !$0 { emu.error = nil } }
            ), presenting: emu.error
        ) { _ in
            Button("OK", role: .cancel) {
                emu.stop()
            }
        } message: { error in
            Text(error.localizedDescription)
        }
        .sheet(isPresented: $detail) {
            NavigationStack {
                GameInfo(game: emu.game!)
                    .toolbar {
                        Button("Done", systemImage: "checkmark", role: .confirm) {
                            detail.toggle()
                        }
                        .bold()
                    }
            }
        }
        .sheet(isPresented: $manage) {
            NavigationStack {
                SettingsView()
                    .toolbar {
                        Button("Done", systemImage: "checkmark", role: .confirm) {
                            manage.toggle()
                        }
                        .bold()
                    }
            }
        }
        .onChange(of: enable) {
            emu.pause(!enable)
        }
    }

    var Controls: some View {
        Menu("Controls", systemImage: "ellipsis") {
            // Playback
            ControlGroup {
                Button {
                    status = (status == .rewind) ? nil : .rewind
                } label: {
                    Label("Rewind", systemImage: status == .rewind ? "backward.fill" : "backward")
                }
                .disabled(true)
                Button {
                    paused.toggle()
                } label: {
                    paused
                        ? Label("Play", systemImage: "play")
                        : Label("Pause", systemImage: "pause.fill")
                }
                Button {
                    status = (status == .sprint) ? nil : .sprint
                } label: {
                    Label("Forward", systemImage: status == .sprint ? "forward.fill" : "forward")
                }
            }
            .labelStyle(.iconOnly)
            // Emulator
            Button("Reset", systemImage: "arrow.counterclockwise") {
                emu.reset()
            }
            // Information
            Section {
                Button("Get Info", systemImage: "info.circle") {
                    detail.toggle()
                }
                Button("Settings", systemImage: "gearshape") {
                    manage.toggle()
                }
            }
            // Application
            Section {
                Button("Exit", systemImage: "xmark", role: .destructive) {
                    emu.stop()
                }
            }
        }
        .menuActionDismissBehavior(.disabled)
    }
}

#Preview {
    NavigationStack {
        EmulatorView()
    }
    .environment(GameBoy())
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
