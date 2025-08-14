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
    @State private var delLibrary = false
    /// Present welcome.
    @State private var welcomeUsr = false

    var body: some View {
        @Bindable var cfg = opt.data

        Form {
            // Header
            AboutView()
            // Application
            Section("Application") {
                // Palette
                Picker(selection: $cfg.pal) {
                    ForEach(Palette.allCases) { pal in
                        HStack {
                            PaletteIcon(pal: pal)
                            Text(pal.description)
                        }
                    }
                } label: {
                    Label("Palette", systemImage: "swatchpalette")
                }
                .pickerStyle(.navigationLink)
                // Speed
                NavigationLink {
                    SpeedPicker(speed: $cfg.spd)
                } label: {
                    HStack {
                        Label("Speed", systemImage: "stopwatch")
                        Spacer()
                        Text(cfg.spd.description)
                            .foregroundStyle(.secondary)
                    }
                }
            }
            // Danger Zone
            Section(isExpanded: $dangerZone) {
                // Reset Settings
                Button(role: .destructive) {
                    defaultOpt = true
                } label: {
                    MultiLineLabel(
                        "Reset Settings",
                        about: "Restore all settings to their defaults.",
                        systemImage: "gearshape.arrow.trianglehead.2.clockwise.rotate.90",
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
                    delLibrary = true
                } label: {
                    MultiLineLabel(
                        "Delete Library",
                        about: "Remove all games in your library.",
                        systemImage: "trash.fill",
                    )
                }
                .confirmationDialog(
                    "Are you sure?",
                    isPresented: $delLibrary,
                ) {
                    Button("Delete", role: .destructive) {
                        withAnimation {
                            lib.games.forEach { game in
                                do { try lib.delete(game: game) } catch { err.or = error }
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
                Button {
                    withAnimation {
                        dangerZone.toggle()
                    }
                } label: {
                    Text("Danger Zone")
                        .bold()
                    Spacer()
                    Image(systemName: "chevron.right")
                        .imageScale(.small)
                        .rotationEffect(dangerZone ? .degrees(90) : .zero)
                }
            }
            .tint(.red)
        }
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
