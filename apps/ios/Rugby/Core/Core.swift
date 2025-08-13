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

    /// Power on/off emulator.
    ///
    /// When powered on from off, the emulator will have been re-initialized.
    ///
    /// # Note
    ///
    /// This is a no-op if the emulator is already in the requested power state.
    func power(_ state: Power)

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

    /// Resume emulator.
    ///
    /// If paused, causes the emulator to resume.
    func start()

    /// Pause emulator.
    ///
    /// When running, causes the emulator to pause.
    func pause()
}

extension Core {
    /// Plays a game.
    ///
    /// Insert the selected cartridge, reset the core, and start emulation.
    func play(_ cart: Cartridge) {
        self.power(.off)
        self.insert(cart: cart)
        self.power(.on)
    }

    /// Stops emulation.
    ///
    /// Power off the emulator and ejects the cartridge.
    func stop() -> Cartridge? {
        self.pause()
        let cart = self.eject()
        self.power(.off)
        return cart
    }
}
