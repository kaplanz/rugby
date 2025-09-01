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
    /// Running game
    private(set) var game: Game?

    /// Play a game.
    func play(_ game: Game) throws {
        // Ensure game is playable
        let _ = try Cartridge(data: game.data)
        // Retain game to play
        self.game = game
    }

    /// Stop playing.
    func stop() {
        // Remove game from play
        game = nil
    }
}
