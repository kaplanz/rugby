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

    var rect: RoundedRectangle {
        RoundedRectangle(cornerRadius: 10)
    }

    var body: some View {
        ZStack {
            rect
                .fill(.ultraThinMaterial)
                .overlay {
                    rect
                        .stroke(.foreground.secondary)
                }
            if let image = image {
                image
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .clipShape(rect)
            } else {
                Image(systemName: "questionmark.app")
                    .font(.system(size: 40))
                    .foregroundColor(.secondary)
            }
        }
        .aspectRatio(10 / 9, contentMode: .fit)
        .frame(minWidth: 80, maxWidth: .infinity)
        .contentShape(.contextMenuPreview, rect)
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
