//
//  GameIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-23.
//

import SwiftUI

struct GameIcon: View {
    @State var game: Game

    private var image: Image {
        game.icon.flatMap(Image.init(uiImage:)) ?? Image("Missing")
    }

    var body: some View {
        image
            .resizable()
            .aspectRatio(10 / 9, contentMode: .fit)
            .clipShape(.containerRelative)
            .glassEffect(in: .containerRelative)
    }
}

#Preview {
    if let game = Bundle
        .main
        .url(forResource: "porklike", withExtension: "gb")
        .flatMap({ try? Game(path: $0) })
    {
        GameIcon(game: game)
    }
}
