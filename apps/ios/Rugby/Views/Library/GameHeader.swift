//
//  GameHeader.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-20.
//

import SwiftUI

struct GameHeader: View {
    @Environment(GameBoy.self) var emu
    @Environment(\.dismiss) private var dismiss

    let game: Game

    var body: some View {
        Section {
            HStack {
                Spacer()
                GameIcon(game: game)
                    .shadow(radius: 4)
                Spacer()
            }
            Text(game.name)
                .font(.title2)
                .fontWeight(.medium)
            Button("Play") {
                dismiss()
                // Only play if not playing anything
                if emu.game == nil {
                    emu.play(game)
                }
            }
            .bold()
            .buttonStyle(.borderedProminent)
            .clipShape(.rect(cornerRadius: .infinity))
        }
        .listRowBackground(Color.clear)
        .listRowSeparator(.hidden, edges: .top)
    }
}

#Preview {
    GameHeader(
        game: try! Game(
            path: Bundle.main.url(
                forResource: "roms/games/porklike/porklike",
                withExtension: "gb"
            )!
        )
    )
    .environment(GameBoy())
}
