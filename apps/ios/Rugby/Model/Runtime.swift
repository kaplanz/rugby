//
//  Runtime.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import Foundation
import RugbyKit

@Observable
final class Runtime {
    /// Emulator instance.
    private(set) var emu: Emulator?

    /// Emulator is active.
    var active: Bool {
        emu != nil
    }

    /// Play a game.
    func play(_ game: Game) throws {
        // Ensure game is playable
        let _ = try Cartridge(data: game.data)
        // Instantiate an emulator
        emu = .init()
        // Insert game to emulator
        try emu?.play(game)
    }

    /// Stop playing.
    func stop() throws {
        // Stop emulator
        try emu?.stop()
        // Drop instance
        emu = nil
    }
}
