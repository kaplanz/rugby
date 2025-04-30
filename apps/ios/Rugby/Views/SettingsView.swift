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

    @State private var icon = UIApplication.shared.alternateIconName.flatMap(
        AppIcon.from(appIcon:)
    ) ?? .gamePak

    let game: Game?

    init(game: Game? = nil) {
        self.game = game
    }

    var body: some View {
        @Bindable var cfg = emu.cfg.data

        Form {
            // Header
            if let game = game {
                GameHeader(game: game)
            } else {
                AppHeader(icon: icon)
                    .id(icon)
            }
            // General
            Section("General") {
                // App Icon
                NavigationLink {
                    List(AppIcon.allCases, id: \.self) { icon in
                        Button {
                            self.icon = icon
                        } label: {
                            HStack {
                                Image(icon.preview)
                                    .resizable()
                                    .aspectRatio(1, contentMode: .fill)
                                    .frame(width: 60, height: 60)
                                    .clipShape(.rect(cornerRadius: 13.5, style: .continuous))
                                Text(icon.description)
                                Spacer()
                                Image(systemName: self.icon == icon ? "checkmark.circle.fill" : "circle")
                                    .foregroundStyle(.tint)
                            }
                            .contentShape(.rect)
                        }
                        .buttonStyle(.plain)
                    }
                } label: {
                    HStack {
                        Text("App Icon")
                        Spacer()
                        Text(icon.description)
                            .foregroundStyle(.secondary)
                    }
                }
                .onChange(of: icon) {
                    UIApplication.shared.setAlternateIconName(icon.appIcon)
                }
            }
            // Application
            Section("Application") {
                // Palette
                Picker("Palette", selection: $cfg.pal) {
                    ForEach(Palette.allCases) { pal in
                        HStack {
                            PaletteIcon(pal: pal)
                            Text(pal.description)
                        }
                    }
                }
                .pickerStyle(.navigationLink)
                // Speed
                Picker("Speed", selection: $cfg.spd) {
                    ForEach(Speed.allCases) { spd in
                        Text(spd.description)
                    }
                }
                .onChange(of: cfg.spd) { _, speed in
                    emu.clock(speed: speed.rawValue)
                }
            }
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
