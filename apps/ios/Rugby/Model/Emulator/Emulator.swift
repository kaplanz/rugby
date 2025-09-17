//
//  Emulator.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-01-09.
//

import Foundation
import RugbyKit
import SwiftUI

@Observable
final class Emulator {
    /// Emulator core.
    private var core: GameBoy = .init()
    /// Inserted game.
    private(set) var game: Game?

    /// Prepares system.
    func prepare(_ game: Game) throws {
        // Create cart
        let cart = try Cartridge(data: game.data)
        if let save = game.save {
            try cart.flash(save: save)
        }
        // Insert cart
        core.insert(cart: cart)
        // Retain game
        self.game = game
    }

    /// Starts emulation.
    func start() {
        self.core.start()
    }

    /// Plays a game.
    ///
    /// Prepares the system, then starts emulation.
    func play(_ game: Game) throws {
        // Prepare system
        try self.prepare(game)
        // Start emulator
        self.start()
    }

    /// Stops emulation.
    func stop() throws {
        // Pause emulator
        self.core.pause()
        // Save screenshot
        self.screenshot()
        // Eject cartridge
        let cart = core.eject()
        // Stop emulator
        self.core.stop()
        // Update save RAM
        if cart?.header().board.power == true {
            game?.save = try cart?.dump()
        }
    }

    /// Pause emulation.
    func pause(_ state: Bool = true) {
        // Save screenshot
        self.screenshot()
        // Pause emulator
        (state ? core.pause : core.start)()
    }

    /// Reset emulator.
    func reset(_ kind: Reset) {
        core.reset(kind)
    }

    /// Save screenshot.
    func screenshot() {
        game?.icon = core.video.image.map { UIImage(cgImage: $0) }
    }

    /// Change speed.
    func speed(_ speed: Speed) {
        core.speed(speed)
    }

    /// Video frame.
    var frame: CGImage? {
        core.video.image
    }

    /// Forward user input.
    func input(_ input: RugbyKit.Button, state: Bool) {
        core.input.queue.withLock { queue in
            queue.append((input, state))
        }
    }
}
