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
        set {}
    }

    /// Emulation error.
    var error: Error?

    /// Most recent frame data.
    private(set) var frame: Data?

    /// Communication channel.
    private var talk = PassthroughSubject<Message, Never>()

    /// Emulator control messages
    private enum Message {
        /// Start emulation with game.
        case play(Cartridge)
        /// Pause emulator.
        case pause(Bool)
        /// Reset emulator.
        case reset
        /// Stop emulator.
        case stop
        /// Input event.
        case input(RugbyKit.Button, Bool)
    }

    init() {
        // Start emulator task
        Task.detached(priority: .userInitiated) {
            // Initialize state
            var emu = RugbyKit.GameBoy()
            var run = false

            // Create profiler
            var prof = Profiler()

            // Handle messages
            let sub = await self.talk.sink { msg in
                switch msg {
                case .play(let cart):
                    // Hard reset
                    emu = RugbyKit.GameBoy()
                    // Insert cartridge
                    do {
                        try emu.insert(cart: cart)
                    } catch let error {
                        // Crash on unknown errors
                        fatalError(error.localizedDescription)
                    }
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
                    // Eject cartridge
                    if !emu.eject() {
                        fatalError("mismatch while ejecting cartridge")
                    }
                    // Reset emulator
                    fallthrough
                case .reset:
                    // Soft reset
                    emu.reset()
                case .input(let button, let pressed):
                    // Forward input
                    (pressed ? emu.press : emu.release)(button)
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
                emu.cycle()
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
        talk.send(.play(game.cart))
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
        game?.icon = frame.flatMap(render(frame:))
        game = nil
    }

    /// Forward input event.
    func input(_ button: RugbyKit.Button, pressed: Bool) {
        talk.send(.input(button, pressed))
    }

    /// Redraws the screen.
    func redraw(frame: Data) {
        self.frame = frame
    }

    func render(frame: Data) -> UIImage? {
        let (wd, ht) = (160, 144)

        // Convert frame to data
        let buf = frame.map { cfg.pal.data[Int($0)].bigEndian }

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
                bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedFirst.rawValue),
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
