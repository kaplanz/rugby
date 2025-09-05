//
//  GameIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-23.
//

import ImageIO
import SwiftUI

struct GameIcon: View {
    @Environment(Options.self) private var opt

    @State var game: Game

    var body: some View {
        Image(uiImage: frame ?? empty)
            .resizable()
            .aspectRatio(10 / 9, contentMode: .fit)
            .clipShape(.containerRelative)
            .glassEffect(in: .containerRelative)
            .onChange(of: opt.data.pal) {
                // Makes this view subscribe to changes in the palette.
            }
    }

    private var frame: UIImage? {
        game.icon?.cgImage.flatMap(Self.redraw).map(UIImage.init(cgImage:))
    }

    private var empty: UIImage {
        // Load bundled unused image
        let url = Bundle.main.url(forResource: "unused", withExtension: "png")!
        let img = UIImage(named: url.path())!
        // Recolor with this palette
        return img.cgImage.flatMap(Self.redraw).map(UIImage.init(cgImage:)) ?? img
    }

    private static func redraw(image: CGImage) -> CGImage? {
        guard case .indexed = image.colorSpace?.model,
            let data = image.dataProvider?.data
        else {
            return nil
        }

        // Extract indexed frame data
        let frame = Data(
            UnsafeBufferPointer(start: CFDataGetBytePtr(data), count: CFDataGetLength(data)))
        // Draw using current palette
        return Video.draw(frame: frame)
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
