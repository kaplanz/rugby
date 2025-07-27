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

    var shape: some Shape {
        .rect(cornerRadius: 12)
    }

    var body: some View {
        ZStack {
            // Image
            image
                .resizable()
                .aspectRatio(contentMode: .fit)
            // Shape
            shape
                .foregroundStyle(.clear)
                .blur(radius: 4)
        }
        .aspectRatio(10 / 9, contentMode: .fit)
        .glassEffect(in: shape)
        .clipShape(shape)
        .frame(minWidth: 80, maxWidth: .infinity)
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
