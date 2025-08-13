//
//  Runtime.swift
//  Rugby
//
//  Created by Zakhary Kaplan on 2025-08-06.
//

import Foundation

@Observable
final class Runtime {
    /// Running game
    private(set) var game: Game?

    /// Play a game.
    func play(_ game: Game) {
        self.game = game
    }

    /// Stop playing.
    func stop() {
        game = nil
    }
}
