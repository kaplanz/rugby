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
    private var core: GameBoy
    /// Inserted game.
    private var game: Game?

    /// Joypad input.
    private var input: Input

    /// Audio output.
    private var audio: Audio
    /// Video output.
    private var video: Video

    init() {
        let core = GameBoy()
        self.core = core
        self.input = core.input
        self.audio = core.audio
        self.video = core.video
    }

    /// Plays a game.
    func play(_ game: Game) throws {
        // Initialize
        let cart = try Cartridge(data: game.data)
        if let save = game.save {
            try cart.flash(save: save)
        }
        // Insert cart
        core.play(cart)
        // Retain game
        self.game = game
    }

    /// Stops emulation.
    func stop() throws {
        // Pause emulation
        self.pause()
        // Eject cartridge
        let cart = core.stop()
        // Update save RAM
        if cart?.header().board.power == true {
            game?.save = try cart?.dump()
        }
    }

    /// Pause emulation.
    func pause(_ state: Bool = true) {
        // Take screenshot
        game?.icon = video.image.map { UIImage(cgImage: $0) }
        // Pause emulation
        (state ? core.pause : core.start)()
        // Pause playback
        (state ? audio.pause : audio.start)()
    }

    /// Reset emulator.
    func reset(_ kind: Reset) {
        core.reset(kind)
    }

    /// Video frame.
    var frame: CGImage? {
        video.image
    }

    /// Forward user input.
    func input(_ input: RugbyKit.Button, state: Bool) {
        self.input.queue.withLock { queue in
            queue.append((input, state))
        }
    }
}
