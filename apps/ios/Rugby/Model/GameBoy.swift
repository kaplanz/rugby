//
//  GameBoy.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-09.
//

import Combine
import Foundation
import RugbyKit
import SwiftUI

@MainActor
@Observable
class GameBoy {
    /// Selected game to play.
    private(set) var game: Game?

    /// Emulator enabled state.
    var show: Bool {
        get {
            game != nil
        }
        set {
            if !newValue {
                stop()
            }
        }
    }

    /// Most recent frame data.
    private(set) var frame: Data?

    /// Communication channel.
    private var talk: PassthroughSubject<Message, Never>

    /// Emulator control messages
    private enum Message {
        /// Start emulation with game.
        case play(Game)
        /// Pause emulator.
        case pause(Bool)
        /// Reset emulator.
        case reset
        /// Stop emulator.
        case stop
    }

    init() {
        // Initialize communication channel
        talk = PassthroughSubject()

        // Start emulator task
        Task.detached {
            // Construct emulator
            var emu = RugbyKit.GameBoy()

            // Initialize state
            var run = false

            // Create profiler
            var prof = Profiler()

            // Handle messages
            let sub = await self.talk.sink { msg in
                switch msg {
                case .play(let game):
                    // Hard reset
                    emu = RugbyKit.GameBoy()
                    // Insert cartridge
                    emu.insert(rom: game.data)
                    // Start emulation
                    run = true
                case .pause(let pause):
                    run = !pause
                    // Reset profiler
                    if run {
                        prof.reset()
                    }
                case .stop:
                    // Stop emulation
                    run = false
                    // Reset emulator
                    fallthrough
                case .reset:
                    // Soft reset
                    emu.reset()
                }
            }

            // Emulator loop
            prof.reset()
            while true {
                // Check if idle
                if !run {
                    // No work to be done... yield to other tasks
                    await Task.yield()
                    // When woken, check if ready to work
                    continue
                }
                // Tick emulator
                if emu.ready() {
                    emu.cycle()
                }
                // Tick display
                if emu.vsync() {
                    await self.redraw(frame: emu.frame())
                }
                // Tick profiler
                if let rate = prof.tick() {
                    log.trace("frame rate: \(rate)")
                }
            }

            // Hand up channel
            sub.cancel()
        }
    }

    /// Play emulator with game.
    func play(_ game: Game) {
        // Retain game
        self.game = game
        // Send to emulator
        talk.send(.play(game))
    }

    /// Pause emulation.
    func pause(_ state: Bool = true) {
        talk.send(.pause(state))
    }

    /// Resume emulation.
    func resume() {
        talk.send(.pause(false))
    }

    /// Reset emulator.
    func reset() {
        talk.send(.reset)
    }

    /// Stop emulation.
    func stop() {
        talk.send(.stop)
        // Save last frame
        game?.icon = frame.flatMap(Self.render(frame:))
        game = nil
    }

    /// Redraws the screen.
    func redraw(frame: Data) {
        self.frame = frame
    }

    static func render(frame: Data) -> UIImage? {
        let (wd, ht) = (160, 144)

        // Convert frame to data
        let pal: [UInt32] = [0xffff_ffff, 0xffaa_aaaa, 0xff55_5555, 0xff00_0000]
        let buf = frame.map { pal[Int($0)] }

        // Convert the buffer of UInt32 into raw bytes
        let bytes = buf.withUnsafeBufferPointer { bufferPointer in
            return bufferPointer.baseAddress!.withMemoryRebound(
                to: UInt8.self, capacity: buf.count * MemoryLayout<UInt32>.size
            ) { rawPointer in
                return Data(bytes: rawPointer, count: buf.count * MemoryLayout<UInt32>.size)
            }
        }

        // Create a CGDataProvider from the raw byte data
        let dataProvider = CGDataProvider(data: bytes as CFData)

        // Define the color space, bytes per row, and bits per component
        let colorSpace = CGColorSpaceCreateDeviceRGB()
        let bytesPerPixel = 4
        let bitsPerComponent = 8
        let bytesPerRow = wd * bytesPerPixel

        // Create the CGImage using the data provider
        guard
            let cgImage = CGImage(
                width: wd,
                height: ht,
                bitsPerComponent: bitsPerComponent,
                bitsPerPixel: bitsPerComponent * bytesPerPixel,
                bytesPerRow: bytesPerRow,
                space: colorSpace,
                bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue),
                provider: dataProvider!,
                decode: nil,
                shouldInterpolate: true,
                intent: .defaultIntent
            )
        else {
            return nil
        }

        // Convert image format
        return UIImage(cgImage: cgImage)
    }
}

/// Emulator profiler.
///
/// Calculates the running frame rate of an emulator task,
private struct Profiler {
    private var count = 0
    private var timer = DispatchTime.now()

    mutating func reset() {
        self = .init()
    }

    mutating func tick() -> Double? {
        // Calculate elapsed time
        let check = DispatchTime.now()
        let delta = Double(check.uptimeNanoseconds - timer.uptimeNanoseconds) / 1_000_000_000
        // Report every second
        var rate: Double? = nil
        if delta > 1 {
            // Update profiled rate
            rate = (Double(count) / 70244.0) * delta
            // Reset properties
            count = 0
            timer = check
        } else {
            // Increment counter
            count += 1
        }
        return rate
    }
}
