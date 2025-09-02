//
//  GameIcon.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2024-06-23.
//

import PNG
import SwiftUI

struct GameIcon: View {
    @Environment(Options.self) private var opt

    @State var game: Game

    private var frame: Image? {
        game.icon.flatMap(Image.init(uiImage:))
    }

    private var empty: Image {
        let url = Bundle.main.url(forResource: "unused", withExtension: "png")!
        let img: PNG.Image = try! .decompress(path: url.path())!
        return recolor(img, pal: opt.data.pal)
    }

    var body: some View {
        (frame ?? empty)
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

private func recolor(_ img: PNG.Image, pal: Palette) -> Image {
    // Convert image to buffer
    let buf = img.storage.map { pixel in
        pal.data[Int(pixel)].bigEndian
    }

    // Cast buffer to data
    let data = buf.withUnsafeBufferPointer { ptr in
        return ptr.baseAddress!.withMemoryRebound(
            to: UInt8.self, capacity: buf.count * MemoryLayout<UInt32>.size
        ) { ptr in
            return Data(bytes: ptr, count: buf.count * MemoryLayout<UInt32>.size)
        }
    }

    // Define image parameters
    let (wd, ht) = img.size
    let bpp = 4
    let bpc = 8
    let bpr = wd * bpp

    // Render data as image
    return Image(
        uiImage: UIImage(
            cgImage: CGImage(
                width: wd,
                height: ht,
                bitsPerComponent: bpc,
                bitsPerPixel: bpc * bpp,
                bytesPerRow: bpr,
                space: CGColorSpaceCreateDeviceRGB(),
                bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipFirst.rawValue),
                provider: CGDataProvider(data: data as CFData)!,
                decode: nil,
                shouldInterpolate: true,
                intent: .defaultIntent
            )!))

}
