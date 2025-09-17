//
//  GameBoy.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import Foundation
import RugbyKit
import Synchronization

/// Clock frequency.
let CLOCK: UInt32 = 4_194_304
/// Audio sample divider.
let AUDIO: UInt32 = 32
/// Video frame rate.
let VIDEO: UInt32 = 70224

/// Emulation context.
private struct Context {
    var alive: Bool = true
    /// Clock timings.
    var clock: Clocking = .init()
    /// Batch counter.
    var batch: Profiler = .init()
    /// Start instant.
    let start: Date = .now
    /// Paused clock
    var delay: TimeInterval = .zero
    /// Total counter.
    var total: UInt64 = 0
}

/// Emulator connection.
private final class Connect: @unchecked Sendable {
    /// Pause signal.
    @Atomic var pause: Bool = false
    /// Reset signal.
    @Atomic var reset: Bool = false

    /// Clock speed.
    let speed: Mutex<Speed?> = .init(nil)

    /// Cartridge slot.
    let media: Media = .init()
    /// Joypad input.
    let input: Input = .init()

    /// Audio output.
    let audio: Audio = .init()
    /// Video output.
    let video: Video = .init()
}

/// Emulator main.
private func main(cxn: Connect) {
    do {
        // Start thread
        log.debug("starting thread: \(Thread.current)")
    }
    defer {
        // Close thread
        log.debug("finished thread: \(Thread.current)")
    }

    // Instantiate emulator
    let emu = RugbyKit.GameBoy()
    // Instantiate context
    var ctx = Context()
    // Prepare clocking
    var delay: Date?

    // Emulator loop
    //
    // Until the emulator exits, this loops is responsible for handling all
    // emulation logic. This includes cycling the system core, synchronizing
    // emulation frames to the wall-clock, and processing core input/output.
    //
    // If enabled, the debugger is also cycled here inline to manage the
    // core.
    while true {
        // Check exit condition
        ctx.alive = !Thread.current.isCancelled
        // Trigger exit cleanup
        if !ctx.alive {
            // Pause emulation
            cxn.pause = true
            // Eject cartridge
            cxn.media.request.withLock { event in
                event = .eject
            }
        }

        // Re-timing events
        cxn.speed.withLockIfAvailable { speed in
            guard let speed = speed.take() else { return }
            // Update emulation speed
            //
            // Also resets the synchronization, which is necessary whenever the
            // clock speed has been changed.
            ctx.clock.frq = speed.freq
        }

        // Cartridge events
        //
        // When requested, insert or eject the cartridge.
        cxn.media.request.withLockIfAvailable { event in
            switch event.take() {
            case .insert(let cart):
                // Insert cartridge
                emu.insert(cart: cart)
                event = nil
            case .eject:
                // Remove cartridge
                cxn.media.respond.withLock { event in
                    event =
                        if let cart = emu.eject() {
                            .eject(cart)
                        } else {
                            .empty
                        }
                }
            case nil:
                break
            }
        }

        // Reset emulator
        //
        // Performs a soft reset on the emulator core.
        if cxn.reset {
            emu.reset()
            cxn.reset = false  // lower flag
        }

        // Sleep while paused
        //
        // This preserves host compute cycles that would otherwise be
        // needlessly wasted spinning.
        if cxn.pause && ctx.alive {
            // Record paused time
            if delay == nil {
                delay = .now
            }
            // Use delay that is negligible in human time
            Thread.sleep(forTimeInterval: 0.01)
            // Once woken, restart loop to re-synchronize
            continue
        } else if let delay = delay.take() {
            // Record elapsed paused time
            ctx.delay += Date.now.timeIntervalSince(delay)
            // Reset clock synchronization
            ctx.clock.reset()
            ctx.batch.reset()
        }

        // Exit condition
        //
        // If the exit condition was triggered this cycle (see top of emulation
        // loop), then we can exit now having already performed necessary cleanup.
        if !ctx.alive {
            break
        }

        // Synchronize thread
        //
        // Ensures the thread doesn't exceed nominal frequency as
        // configured.
        if ctx.clock.sync() {
            continue
        }

        // Cycle emulator
        //
        // Advances the emulator by a single virtual clock cycle.
        emu.cycle()

        // Sample audio
        //
        // Audio is downsampled to a more reasonable frequency, as practically
        // generating samples each cycle is unnecessary.
        if ctx.total % UInt64(AUDIO) == 0 {
            // Produce audio sample
            let sample = emu.sample()
            // Forward to audio output
            cxn.audio.push(sample: sample)
        }

        // Sample video
        //
        // Video is sampled only once per vsync, then the emulator indicates
        // it has completed drawing the frame.
        if emu.vsync() {
            // Produce frame as buffer
            let frame = emu.frame()
            // Forward to video output
            let video = cxn.video
            Task.detached(priority: .userInitiated) { [video, frame] in
                video.push(frame: frame)
            }
        }

        // Perform lower-frequency actions
        if ctx.total % UInt64((ctx.clock.frq ?? CLOCK) / 64) == 0 {
            // Sample input
            //
            // Joypad input is sampled to the emulator ~64 times per second,
            // as doing so more often impacts performance and shouldn't be
            // noticeable to users. This improves overall emulation
            // efficiency.
            if let events = cxn.input.queue.withLockIfAvailable({ queue in
                defer { queue.removeAll() }
                return queue
            }) {
                events.forEach { (input, state) in
                    (state ? emu.press : emu.release)(input)
                }
            }

            // Report performance
            //
            // Approximately once per second, we should generate a
            // performance report. This will be logged and updated in the
            // window's title.
            if let freq = ctx.batch.reportDelay() {
                // Log performance
                log.notice("\(frequency(rate: freq))")
            }
        }

        // Count clocked cycle
        ctx.clock.tick()
        ctx.batch.tick()
        ctx.total += 1
    }

    // Report benchmark
    let time = ctx.start.addingTimeInterval(ctx.delay)
    log.info("\(benchmark(tick: ctx.total, time: Date.now.timeIntervalSince(time)))")
    // Report frequency
    var perf = Profiler(clk: time, idx: .init(ctx.total))
    log.info("\(frequency(rate: perf.report()))")
}

/// Generates a benchmark report.
private func benchmark(tick: UInt64, time: TimeInterval) -> String {
    return String(
        format:
            "benchmark: %3d.%06d MCy, elapsed: %4.2f",
        tick / 1_000_000,
        tick % 1_000_000,
        time,
    )
}

/// Generates a frequency report.
private func frequency(rate freq: Double) -> String {
    return String(
        format:
            "frequency: %10.6f MHz, speedup: %4.2fx, frames: %6.2f FPS",
        freq / 1e6,
        freq / Double(CLOCK),
        freq / Double(VIDEO),
    )
}

/// Game Boy (DMG) core.
final class GameBoy {
    /// Emulator thread.
    private var job: Thread?
    /// Emulator connection.
    private let cxn: Connect = .init()

    deinit {
        self.cancel()
    }

    /// Emulator powered state
    var power: Power {
        job != nil ? .on : .off
    }

    /// Launch emulation thread.
    private func launch() {
        // Copy context
        let cxn = cxn
        // Launch thread
        job = Thread { main(cxn: cxn) }
        job?.qualityOfService = .userInitiated
        job?.start()
    }

    /// Cancel emulation thread.
    private func cancel() {
        // Cancel thread
        job?.cancel()
        job = nil
    }

    /// Power on/off emulator.
    ///
    /// When powered on from off, the emulator will have been re-initialized.
    ///
    /// # Note
    ///
    /// This is a no-op if the emulator is already in the requested power state.
    private func power(_ state: Power) {
        switch state {
        case .off:
            // No-op if stopped
            guard job != nil else { return }
            // Cancel emulation thread
            self.cancel()
        case .on:
            // No-op if running
            guard job == nil else { return }
            // Launch emulation thread
            self.launch()
        }
    }
}

extension GameBoy: Core {
    var input: Input {
        cxn.input
    }

    var audio: Audio {
        cxn.audio
    }

    var video: Video {
        cxn.video
    }

    func reset(_ kind: Reset) {
        // No-op if stopped
        guard power == .on else { return }

        // Perform reset
        switch kind {
        case .soft:
            // Signal reset
            cxn.reset = true
        case .hard:
            // Pause emulator
            self.pause()
            // Eject cartridge
            let cart = self.eject()
            // Stop emulator
            self.stop()
            // Re-insert cart
            if let cart {
                self.insert(cart: cart)
            }
            // Restart system
            self.start()
        }

        // Reset audio
        cxn.audio.reset()
    }

    func insert(cart: Cartridge) {
        cxn.media.insert(cart)
    }

    func eject() -> Cartridge? {
        job.flatMap { _ in cxn.media.eject() }
    }

    func start() {
        // Power emulator
        self.power(.on)
        // Start emulator
        cxn.pause = false
        // Start playback
        cxn.audio.start()
    }

    func pause() {
        // Pause emulator
        cxn.pause = true
        // Pause playback
        cxn.audio.pause()
    }

    func stop() {
        // Pause emulator
        self.pause()
        // Power emulator
        self.power(.off)
    }

    func speed(_ speed: Speed) {
        // Re-time emulator
        cxn.speed.withLock { $0 = speed }
        // Re-time playback
        cxn.audio.retime(rate: speed.freq ?? CLOCK)
    }
}
