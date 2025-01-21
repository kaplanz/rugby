//
//  GameIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-23.
//

import SwiftUI

struct GameIcon: View {
    @State var game: Game

    private var image: Image? {
        game.icon.flatMap(Image.init(uiImage:))
    }

    var body: some View {
        (image ?? Image(systemName: "questionmark.app"))
            .frame(width: 160, height: 144)
            .background(.ultraThinMaterial)
            .clipShape(.rect(cornerRadius: 15))
            .contentShape(.contextMenuPreview, .rect(cornerRadius: 15))
            .overlay {
                RoundedRectangle(cornerRadius: 15)
                    .stroke(.foreground.secondary)
            }
    }
}

#Preview {
    GameIcon(
        game: try! Game(
            path: Bundle.main.url(
                forResource: "roms/games/porklike/porklike",
                withExtension: "gb"
            )!
        )
    )
}
