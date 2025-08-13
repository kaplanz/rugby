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
    @State private var paused = false
    /// About this game.
    @State private var detail = false
    /// Manage this game.
    @State private var manage = false

    /// Emulator enabled.
    private var enable: Bool {
        !paused && !detail && !manage && scenePhase == .active
    }

    /// Video output frame.
    private var frame: UIImage? {
        emu.video.image.map { UIImage(cgImage: $0) }
    }

    var body: some View {
        let call = { (input, press) in
            emu.input.queue.withLock { queue in
                queue.append((input, press))
            }
        }

        GeometryReader { geo in
            if geo.size.height > geo.size.width {
                VStack {
                    ScreenView(frame: frame)
                        .id(frame)
                    Spacer()
                    JoypadView(call)
                    Spacer()
                }
            } else {
                HStack {
                    JoypadView(call, part: .left)
                    Spacer()
                    ScreenView()
                    Spacer()
                    JoypadView(call, part: .right)
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
        .sheet(isPresented: $detail) {
            NavigationStack {
                GameInfoView(game: app.game!)
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
        .onAppear {
            do { try emu.play(app.game!) } catch { err.or = error }
        }
        .onChange(of: enable) {
            emu.pause(!enable)
        }
        .onDisappear {
            do { try emu.stop() } catch { err.or = error }
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
                    paused.toggle()
                } label: {
                    paused
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
                    detail.toggle()
                }
                Button("Settings", systemImage: "gearshape") {
                    manage.toggle()
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
