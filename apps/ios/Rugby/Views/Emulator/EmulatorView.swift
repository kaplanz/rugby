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

    /// Extra toolbar items.
    @State private var extras = Extras()

    /// Extra toolbar items.
    struct Extras {
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
            ToolbarItemGroup(placement: .topBarLeading) {
                if extras.rate, let rate = emu.stats.rate {
                    Text(String(format: "%.1f FPS", rate))
                        .bold()
                        .foregroundStyle(Color.accentColor)
                }
            }
            ToolbarItemGroup(placement: .topBarTrailing) {
                Menu("Help", systemImage: "ellipsis.circle") {
                    Button {
                        paused.toggle()
                    } label: {
                        paused
                        ? Label("Play", systemImage: "play") : Label("Pause", systemImage: "pause")
                    }
                    Button("Reset", systemImage: "arrow.clockwise") {
                        emu.reset()
                    }
                    Divider()
                    Menu("Toolbar", systemImage: "switch.2") {
                        Toggle("Frequency", systemImage: "stopwatch", isOn: $extras.rate)
                    }
                    Divider()
                    Button("Get Info", systemImage: "info.circle") {
                        detail.toggle()
                    }
                    Button("Settings", systemImage: "gearshape") {
                        manage.toggle()
                    }
                    Divider()
                    Button("Exit", systemImage: "xmark.circle", role: .destructive) {
                        emu.stop()
                    }
                }
                .labelStyle(.iconOnly)
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
                GameDetails(game: emu.game!)
                    .toolbar {
                        Button("Done") {
                            detail.toggle()
                        }
                        .bold()
                    }
            }
        }
        .sheet(isPresented: $manage) {
            NavigationStack {
                SettingsView(game: emu.game)
                    .toolbar {
                        Button("Done") {
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
