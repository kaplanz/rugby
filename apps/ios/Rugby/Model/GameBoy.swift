//
//  GameBoy.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-09.
//

@preconcurrency import Combine
import Foundation
import RugbyKit
import SwiftUI

@MainActor
@Observable
class GameBoy {
    /// Global configuration.
    var cfg = Settings(path: Library.root)

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
    private(set) var frame: Data? {
        didSet {
            image = frame.flatMap(render(frame:))
        }
    }

    /// Recent gameplay statistics.
    private(set) var stats = Stats()

    /// Gameplay statistics.
    struct Stats {
        /// Frame rate.
        var rate: Double?
    }

    /// Move recent frame image.
    private(set) var image: UIImage? = nil

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
        /// Clock retiming.
        case clock(Double?)
    }

    init() {
        // Start emulator task
        Task.detached(priority: .userInitiated) {
            // Initialize emulator
            var emu = RugbyKit.GameBoy()
            // Initialize state
            var pause = true
            // Initialize clock
            typealias Clock = ContinuousClock
            let clock: Clock = .init()
            var awake: Clock.Instant?
            var speed: Double? = 1.0
            // Initialize profiler
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
                    // Reset profiler
                    prof.reset()
                    // Start emulation
                    pause = false
                case .pause(let state):
                    pause = state
                    // Reset profiler
                    if !pause {
                        prof.reset()
                    }
                case .stop:
                    // Stop emulation
                    pause = true
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
                case .clock(let modifier):
                    // Update speed
                    speed = modifier
                }
            }

            // Emulator loop
            while true {
                // Sleep while paused
                if pause {
                    // Use delay that is negligible in human time
                    try? await Task.sleep(for: .milliseconds(10))
                    // Once woken, restart loop to determine state
                    continue
                }
                // Sleep until awake
                if let awake = awake, awake > .now {
                    // No work to be done... sleep until woken
                    try? await Task.sleep(until: awake)
                    // Once woken, perform this cycle's work
                } else {
                    // When lagging behind, reset clock
                    awake = nil
                }

                // Record pre-emulation timestamp
                let prior = clock.now

                // Emulate next frame
                let count = emu.run()
                // Redraw updated frame
                if emu.vsync() {
                    await self.redraw(frame: emu.frame())
                }

                // Update profiler
                if let rate = prof.tick(by: count) {
                    // Retain frame rate
                    log.trace("frame rate: \(rate)")
                    DispatchQueue.main.async {
                        self.stats.rate = rate
                    }
                }

                // Determine sync delay
                if let speed = speed, speed > 0 {
                    // Calculate expected delay
                    let delay = (.seconds(1) / 4_194_304) * (Double(count) / speed)
                    // Schedule next wake
                    awake = (awake ?? prior) + delay
                } else {
                    awake = nil
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
        // Reset clock speed
        clock(speed: cfg.data.spd.rawValue)
    }

    /// Pause emulation.
    func pause(_ state: Bool = true) {
        talk.send(.pause(state))
        // Save paused frame
        if state {
            game?.icon = image
        }
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
        // Save final frame
        game?.icon = image
        // Remove game
        game = nil
    }

    /// Forward input event.
    func input(_ button: RugbyKit.Button, pressed: Bool) {
        talk.send(.input(button, pressed))
    }

    /// Forward speed change.
    func clock(speed: Double?) {
        talk.send(.clock(speed))
    }

    /// Redraws the screen.
    func redraw(frame: Data) {
        self.frame = frame
    }

    /// Renders the screen to an image.
    private func render(frame: Data) -> UIImage? {
        let (wd, ht) = (160, 144)

        // Convert frame to data
        let buf = frame.map { cfg.data.pal.data[Int($0)].bigEndian }

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
                bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipFirst.rawValue),
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
    /// Elapsed cycle count.
    private var count: UInt32 = 0
    /// Statistics timestamp.
    private var timer = ContinuousClock.now

    mutating func reset() {
        self = .init()
    }

    mutating func tick(by update: UInt32 = 1) -> Double? {
        // Increment counter
        count += update
        // Read elapsed time
        let delta = timer.duration(to: .now)
        // Report every second
        var rate: Double? = nil
        if delta > .seconds(1) {
            // Update profiled rate
            rate = (Double(count) / 70_244.0) * (.seconds(1) / delta)
            // Reset profiler
            count = 0
            timer = .now
        }
        return rate
    }
}
