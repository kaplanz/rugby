//
//  SettingsView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct SettingsView: View {
    @Environment(GameBoy.self) private var emu
    @Environment(\.openURL) private var openURL

    /// Open the danger zone.
    @State private var dangerZone = false
    /// Show alert for reset.
    @State private var alertReset = false

    var body: some View {
        @Bindable var cfg = emu.cfg.data

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
                Picker(selection: $cfg.spd) {
                    ForEach(Speed.allCases) { spd in
                        Text(spd.description)
                    }
                } label: {
                    Label("Speed", systemImage: "stopwatch")
                }
                .onChange(of: cfg.spd) { _, speed in
                    emu.clock(speed: speed.rawValue)
                }
            }
            // Danger Zone
            Section(isExpanded: $dangerZone) {
                // Reset All
                Button(role: .destructive) {
                    alertReset = true
                } label: {
                    Label {
                        VStack(alignment: .leading) {
                            Text("Reset Settings")
                                .foregroundStyle(.tint)
                            Text("This will restore all settings to their defaults.")
                                .font(.caption)
                                .foregroundStyle(Color.secondary)
                        }
                    } icon: {
                        Image(systemName: "gearshape.arrow.trianglehead.2.clockwise.rotate.90")
                            .foregroundStyle(.tint)
                    }
                }
                .confirmationDialog(
                    "Are you sure?",
                    isPresented: $alertReset,
                ) {
                    Button("Reset", role: .destructive) {
                        withAnimation {
                            // TODO: reset settings
                        }
                    }
                } message: {
                    Text("This action cannot be undone.")
                }
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
    }
}

#Preview {
    NavigationStack {
        SettingsView()
            .environment(GameBoy())
    }
}
