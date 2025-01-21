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

    @State private var pal = Palette.mono
    @State private var spd = Speed.actual

    var body: some View {
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
                Picker("Palette", selection: $pal) {
                    ForEach(Palette.allCases) { pal in
                        Text(pal.description)
                    }
                }
                .pickerStyle(.navigationLink)
                // Speed
                Picker("Speed", selection: $spd) {
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

enum Palette: String, CaseIterable, CustomStringConvertible, Identifiable {
    case mono

    // impl CustomStringConvertible
    var description: String {
        rawValue.capitalized
    }

    // impl Identifiable
    var id: Self { self }
}

enum Speed: Float, CaseIterable, CustomStringConvertible, Identifiable {
    case half = 0.5
    case actual = 1.0
    case double = 2.0
    case turbo = 0.0
    // impl CustomStringConvertible
    var description: String {
        switch self {
        case .turbo:
            "Turbo"
        default:
            rawValue.formatted(.percent)
        }
    }

    // impl Identifiable
    var id: Self { self }
}
