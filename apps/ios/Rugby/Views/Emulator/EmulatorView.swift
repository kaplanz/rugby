//
//  EmulatorView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct EmulatorView: View {
    @Environment(GameBoy.self) private var emu

    /// Emulator paused.
    @State private var paused = false
    /// About this game.
    @State private var detail = false
    /// Manage this game.
    @State private var manage = false

    var body: some View {
        GeometryReader { geo in
            if geo.size.height > geo.size.width {
                VStack {
                    Screen()
                    Spacer()
                    Joypad()
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
        .onChange(of: paused) {
            if paused {
                emu.pause()
            } else {
                emu.resume()
            }
        }
        .toolbar {
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
                Button("Get Info", systemImage: "info.circle") {
                    detail.toggle()
                }
                Button("Manage", systemImage: "gearshape") {
                    manage.toggle()
                }
                Divider()
                Button("Exit", systemImage: "xmark.circle", role: .destructive) {
                    emu.stop()
                }
            }
            .labelStyle(.iconOnly)
        }
        .sheet(isPresented: $detail) {
            NavigationStack {
                GameInfo(game: emu.game!)
                    .navigationTitle("Info")
                    .navigationBarTitleDisplayMode(.inline)
                    .toolbar {
                        Button("Done") {
                            detail.toggle()
                        }
                        .bold()
                    }
                    .onAppear {
                        emu.pause()
                    }
                    .onDisappear {
                        emu.pause(paused)
                    }
            }
        }
        .sheet(isPresented: $manage) {
            NavigationStack {
                SettingsView()
                    .navigationTitle(emu.game!.name)
                    .navigationBarTitleDisplayMode(.inline)
                    .toolbar {
                        Button("Done") {
                            manage.toggle()
                        }
                        .bold()
                    }
                    .onAppear {
                        emu.pause()
                    }
                    .onDisappear {
                        emu.pause(paused)
                    }
            }
        }
    }
}

#Preview {
    EmulatorView()
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
