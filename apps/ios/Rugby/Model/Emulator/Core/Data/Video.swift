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
            let image = Self.draw(frame: frame)
            // Publish on main thread
            Task { @MainActor in
                self.image = image
            }
        }
    }

    /// Draw a video frame.
    static func draw(frame: Frame) -> CGImage? {
        // Use frame indices directly (no colour lookup)
        let pal = Options().data.pal.data
        let buf = Data(frame)

        // Convert palette to colour table data
        let data = Array(
            pal.flatMap { value in
                withUnsafeBytes(of: value.littleEndian) { bytes in
                    [
                        bytes[2],  // red
                        bytes[1],  // green
                        bytes[0],  // blue
                    ]
                }
            })

        // Create indexed colour space
        guard
            let space = CGColorSpace(
                indexedBaseSpace: CGColorSpaceCreateDeviceRGB(),
                last: pal.count - 1,
                colorTable: data.withUnsafeBytes { $0.bindMemory(to: UInt8.self).baseAddress! }
            )
        else {
            return nil
        }

        // Define image parameters
        let (wd, ht) = (160, 144)

        // Render data as image
        return CGImage(
            width: wd,
            height: ht,
            bitsPerComponent: 8,
            bitsPerPixel: 8,
            bytesPerRow: wd,
            space: space,
            bitmapInfo: CGBitmapInfo(rawValue: 0),
            provider: CGDataProvider(data: buf as CFData)!,
            decode: nil,
            shouldInterpolate: true,
            intent: .defaultIntent
        )
    }
}
