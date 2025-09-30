//
//  SettingsView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct SettingsView: View {
    @Environment(Failure.self) private var err
    @Environment(Library.self) private var lib
    @Environment(Options.self) private var opt

    /// Open the danger zone.
    @State private var dangerZone = false
    /// Reset settings.
    @State private var defaultOpt = false
    /// Delete library.
    @State private var libraryDel = false
    /// Present welcome.
    @State private var welcomeUsr = false

    var body: some View {
        @Bindable var cfg = opt.data

        List {
            // Header
            HeaderView()

            Section {
                // Playback
                NavigationLink {
                    PlaybackSettings()
                } label: {
                    SettingsLabel("Playback", color: .gray, systemImage: "playpause.fill")
                }
                // Emulator
                NavigationLink {
                    EmulatorSettings()
                } label: {
                    SettingsLabel("Emulator", color: .green, systemImage: "cpu")
                }
                // Controls
                NavigationLink {
                    ControlsSettings()
                } label: {
                    SettingsLabel("Controls", color: .pink, systemImage: "gamecontroller.fill")
                }
            } header: {
                Label("General", systemImage: "gear")
            }

            Section {
                // Audio
                NavigationLink {
                    AudioSettings()
                } label: {
                    SettingsLabel("Audio", color: .orange, systemImage: "hifispeaker")
                }
                // Video
                NavigationLink {
                    VideoSettings()
                } label: {
                    SettingsLabel("Video", color: .indigo, systemImage: "tv")
                }
            } header: {
                Label("Media", systemImage: "tv.and.hifispeaker.fill")
            }

            // Danger Zone
            Section(isExpanded: $dangerZone) {
                // Reset Settings
                Button(role: .destructive) {
                    defaultOpt = true
                } label: {
                    MultiLineLabel(
                        "Reset Settings",
                        about: "Reset all settings to their defaults.",
                        systemImage: "gear.badge.xmark",
                    )
                }
                .confirmationDialog(
                    "Are you sure?",
                    isPresented: $defaultOpt,
                ) {
                    Button("Reset", role: .destructive) {
                        withAnimation {
                            opt.reset()
                        }
                    }
                } message: {
                    Text("This action cannot be undone.")
                }
                // Delete library
                Button(role: .destructive) {
                    libraryDel = true
                } label: {
                    MultiLineLabel(
                        "Delete Library",
                        about: "Remove all games in your library.",
                        systemImage: "trash.fill",
                    )
                }
                .confirmationDialog(
                    "Are you sure?",
                    isPresented: $libraryDel,
                ) {
                    Button("Delete", role: .destructive) {
                        withAnimation {
                            lib.games.forEach { game in
                                do { try lib.delete(game: game) } catch { err.log(error) }
                            }
                        }
                    }
                } message: {
                    Text("This action cannot be undone.")
                }
                // Show welcome
                Button {
                    welcomeUsr = true
                } label: {
                    MultiLineLabel(
                        "Show Welcome",
                        about: "Present the welcome splash screen.",
                        systemImage: "app.gift",
                    )
                }
                .sheet(isPresented: $welcomeUsr) {
                    WelcomeView()
                }
                .tint(nil)
            } header: {
                Label("Danger Zone", systemImage: "exclamationmark.triangle")
                    .foregroundStyle(.tint)
            }
            .tint(.red)
        }
        .listStyle(.sidebar)
        .navigationTitle("Settings")
        .navigationBarTitleDisplayMode(.inline)
        .id(opt.uuid)
    }
}

#Preview {
    NavigationStack {
        SettingsView()
    }
    .environment(Failure())
    .environment(Library())
    .environment(Options())
}

private struct SettingsLabel: View {
    @Environment(\.colorScheme) var colorScheme

    @State private var title: String
    @State private var color: Color
    @State private var systemImage: String

    init(_ titleKey: String, color: Color, systemImage: String) {
        self.title = titleKey
        self.color = color
        self.systemImage = systemImage
    }

    /// Border shape.
    private var shape: some Shape {
        RoundedRectangle(cornerRadius: 6, style: .continuous)
    }

    var body: some View {
        Label {
            Text(title)
        } icon: {
            Image(systemName: systemImage)
                .resizable()
                .foregroundStyle((colorScheme == .dark ? color : .white).gradient)
                .scaledToFit()
                .padding(4)
                .frame(width: 28, height: 28)
                .background {
                    shape
                        .fill((colorScheme == .dark ? .black : color).gradient)
                }
                .glassEffect(
                    colorScheme == .dark ? .regular : .identity,
                    in: shape
                )
        }
    }
}

private struct MultiLineLabel: View {
    @State private var title: String
    @State private var about: String
    @State private var systemImage: String

    init(_ titleKey: String, about: String, systemImage: String) {
        self.title = titleKey
        self.about = about
        self.systemImage = systemImage
    }

    var body: some View {
        Label {
            VStack(alignment: .leading) {
                Text(title)
                    .foregroundStyle(.tint)
                Text(about)
                    .font(.caption)
                    .foregroundStyle(Color.secondary)
            }
        } icon: {
            Image(systemName: systemImage)
                .foregroundStyle(.tint)
        }
    }
}
