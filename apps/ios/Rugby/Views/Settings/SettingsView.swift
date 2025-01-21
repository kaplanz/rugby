//
//  SettingsView.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-20.
//

import SwiftUI

struct SettingsView: View {
    @Environment(\.openURL) private var openURL

    let game: Game?

    init(game: Game? = nil) {
        self.game = game
    }

    var body: some View {
        @Bindable var cfg = cfg

        Form {
            // Header
            if let game = game {
                GameHeader(game: game)
            } else {
                AppHeader()
            }
            // Application
            Section("Application") {
                // Palette
                Picker("Palette", selection: $cfg.pal) {
                    ForEach(Palette.allCases) { pal in
                        PaletteView(pal: pal)
                    }
                }
                .pickerStyle(.navigationLink)
                // Speed
                Picker("Speed", selection: $cfg.spd) {
                    ForEach(Speed.allCases) { spd in
                        Text(spd.description)
                    }
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
    }
}
