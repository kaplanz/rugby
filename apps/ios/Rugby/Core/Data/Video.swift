//
//  Video.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import CoreImage
import Foundation

extension Video {
    /// A single video frame.
    typealias Frame = Data
}

/// Video publisher.
@Observable
final class Video: @unchecked Sendable {
    /// Rendered frame.
    private(set) var image: CGImage?

    /// Processing queue.
    private let queue = DispatchQueue(label: "dev.zakhary.rugby.core.video")

    /// Publish a video frame.
    func push(frame: Frame) {
        queue.async { [weak self] in
            guard let self else { return }
            // Draw frame to image
            let image = self.draw(frame: frame)
            // Publish on main thread
            Task { @MainActor in
                self.image = image
            }
        }
    }

    /// Draw a video frame.
    private func draw(frame: Frame) -> CGImage? {
        // Colour frame to buffer
        let pal = Options().data.pal
        let buf = frame.map { pal.data[Int($0)].bigEndian }

        // Cast buffer to data
        let data = buf.withUnsafeBufferPointer { ptr in
            return ptr.baseAddress!.withMemoryRebound(
                to: UInt8.self, capacity: buf.count * MemoryLayout<UInt32>.size
            ) { ptr in
                return Data(bytes: ptr, count: buf.count * MemoryLayout<UInt32>.size)
            }
        }

        // Define image parameters
        let (wd, ht) = (160, 144)
        let bpp = 4
        let bpc = 8
        let bpr = wd * bpp

        // Render data as image
        return CGImage(
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
        )
    }
}
