//
//  Core.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import Foundation
import RugbyKit

/// Powered state.
enum Power {
    /// Powered off; idle.
    case off
    /// Powered on; running.
    case on
}

/// Emulator reset.
enum Reset {
    /// Soft reset; maintains previous state for undefined registers and memory.
    case soft
    /// Hard reset; completely re-initializes emulator to defined internal state.
    case hard
}

/// Emulator core.
protocol Core {
    /// Joypad input.
    var input: Input { get }
    /// Audio output.
    var audio: Audio { get }
    /// Video output.
    var video: Video { get }

    /// Reset emulator.
    ///
    /// Performs a resets on the emulator according to the specified semantics.
    func reset(_ kind: Reset)

    /// Insert cartridge.
    ///
    /// Cartridge is inserted into the emulator.
    ///
    /// # Note
    ///
    /// Only call this when powered off. When powered on, this leads to
    /// unpredictable behaviour.
    func insert(cart: Cartridge)

    /// Eject cartridge.
    ///
    /// Cartridge is ejected from the emulator.
    ///
    /// # Note
    ///
    /// Only call this when powered off. When powered on, this leads to
    /// unpredictable behaviour.
    func eject() -> Cartridge?

    /// Start emulator.
    ///
    /// If paused, causes the emulator to resume.
    func start()

    /// Pause emulator.
    ///
    /// When running, causes the emulator to pause.
    func pause()

    /// Stop emulator.
    ///
    /// Powers off the emulator.
    func stop()

    /// Change speed.
    ///
    /// Changes the emulated clock speed.
    func speed(_ speed: Speed)
}
